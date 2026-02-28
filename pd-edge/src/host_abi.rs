use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::http::{HeaderMap, HeaderName, HeaderValue};
use url::Url;
use vm::{CallOutcome, HostFunction, Value, Vm, VmError};

pub type SharedRateLimiter = Arc<Mutex<RateLimiterStore>>;

#[derive(Debug, Default)]
pub struct RateLimiterStore {
    buckets: HashMap<String, RateLimitBucket>,
}

#[derive(Debug)]
struct RateLimitBucket {
    window_start: Instant,
    count: u64,
}

impl RateLimiterStore {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    fn allow(&mut self, key: &str, limit: u64, window_seconds: u64) -> bool {
        if limit == 0 || window_seconds == 0 {
            return false;
        }

        let now = Instant::now();
        let window = Duration::from_secs(window_seconds);
        let bucket = self
            .buckets
            .entry(key.to_string())
            .or_insert_with(|| RateLimitBucket {
                window_start: now,
                count: 0,
            });

        if now.duration_since(bucket.window_start) >= window {
            bucket.window_start = now;
            bucket.count = 0;
        }

        if bucket.count < limit {
            bucket.count += 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProxyVmContext {
    request_headers: HeaderMap,
    response_headers: HeaderMap,
    response_content: Option<String>,
    response_status: Option<u16>,
    upstream: Option<String>,
    rate_limiter: SharedRateLimiter,
}

impl ProxyVmContext {
    pub fn from_request_headers(
        request_headers: HeaderMap,
        rate_limiter: SharedRateLimiter,
    ) -> Self {
        Self {
            request_headers,
            response_headers: HeaderMap::new(),
            response_content: None,
            response_status: None,
            upstream: None,
            rate_limiter,
        }
    }
}

pub type SharedProxyVmContext = Arc<Mutex<ProxyVmContext>>;

#[derive(Clone, Debug)]
pub struct VmExecutionOutcome {
    pub response_headers: HeaderMap,
    pub response_content: Option<String>,
    pub response_status: Option<u16>,
    pub upstream: Option<String>,
}

pub fn snapshot_execution_outcome(context: &SharedProxyVmContext) -> VmExecutionOutcome {
    let context = context.lock().expect("vm context lock poisoned");
    VmExecutionOutcome {
        response_headers: context.response_headers.clone(),
        response_content: context.response_content.clone(),
        response_status: context.response_status,
        upstream: context.upstream.clone(),
    }
}

pub fn register_host_module(vm: &mut Vm, context: SharedProxyVmContext) -> Result<(), VmError> {
    vm.bind_function(
        "get_header",
        Box::new(GetHeaderFunction::new(context.clone())),
    );
    vm.bind_function(
        "set_header",
        Box::new(SetHeaderFunction::new(context.clone())),
    );
    vm.bind_function(
        "set_response_content",
        Box::new(SetResponseContentFunction::new(context.clone())),
    );
    vm.bind_function(
        "set_upstream",
        Box::new(SetUpstreamFunction::new(context.clone())),
    );
    vm.bind_function(
        "set_response_status",
        Box::new(SetResponseStatusFunction::new(context.clone())),
    );
    vm.bind_function(
        "rate_limit_allow",
        Box::new(RateLimitAllowFunction::new(context)),
    );
    Ok(())
}

struct GetHeaderFunction {
    context: SharedProxyVmContext,
}

impl GetHeaderFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

impl HostFunction for GetHeaderFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 1)?;
        let name = expect_string(args, 0)?;
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| VmError::HostError(format!("invalid header name '{name}'")))?;

        let context = self.context.lock().expect("vm context lock poisoned");
        let value = context
            .request_headers
            .get(&header_name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        Ok(CallOutcome::Return(vec![Value::String(value.to_string())]))
    }
}

struct SetHeaderFunction {
    context: SharedProxyVmContext,
}

impl SetHeaderFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

impl HostFunction for SetHeaderFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 2)?;
        let name = expect_string(args, 0)?;
        let value = expect_string(args, 1)?;

        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| VmError::HostError(format!("invalid header name '{name}'")))?;
        let header_value = HeaderValue::from_str(&value)
            .map_err(|_| VmError::HostError(format!("invalid header value '{value}'")))?;

        let mut context = self.context.lock().expect("vm context lock poisoned");
        context.response_headers.insert(header_name, header_value);
        Ok(CallOutcome::Return(vec![]))
    }
}

struct SetResponseContentFunction {
    context: SharedProxyVmContext,
}

impl SetResponseContentFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

impl HostFunction for SetResponseContentFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 1)?;
        let body = expect_string(args, 0)?;
        let mut context = self.context.lock().expect("vm context lock poisoned");
        context.response_content = Some(body);
        Ok(CallOutcome::Return(vec![]))
    }
}

struct SetUpstreamFunction {
    context: SharedProxyVmContext,
}

impl SetUpstreamFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

struct SetResponseStatusFunction {
    context: SharedProxyVmContext,
}

impl SetResponseStatusFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

impl HostFunction for SetResponseStatusFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 1)?;
        let status = expect_int(args, 0)?;
        if !(100..=599).contains(&status) {
            return Err(VmError::HostError(format!(
                "status code must be in range 100..=599, got '{status}'",
            )));
        }

        let mut context = self.context.lock().expect("vm context lock poisoned");
        context.response_status = Some(status as u16);
        Ok(CallOutcome::Return(vec![]))
    }
}

impl HostFunction for SetUpstreamFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 1)?;
        let upstream = expect_string(args, 0)?;
        if !is_valid_upstream(&upstream) {
            return Err(VmError::HostError(format!(
                "upstream must be host:port or http(s)://host[:port][/path], got '{upstream}'",
            )));
        }

        let mut context = self.context.lock().expect("vm context lock poisoned");
        context.upstream = Some(upstream);
        Ok(CallOutcome::Return(vec![]))
    }
}

struct RateLimitAllowFunction {
    context: SharedProxyVmContext,
}

impl RateLimitAllowFunction {
    fn new(context: SharedProxyVmContext) -> Self {
        Self { context }
    }
}

impl HostFunction for RateLimitAllowFunction {
    fn call(&mut self, _vm: &mut Vm, args: &[Value]) -> Result<CallOutcome, VmError> {
        expect_arg_count(args, 3)?;
        let key = expect_string(args, 0)?;
        let limit = expect_int(args, 1)?;
        let window_seconds = expect_int(args, 2)?;
        if limit <= 0 || window_seconds <= 0 {
            return Ok(CallOutcome::Return(vec![Value::Bool(false)]));
        }

        let rate_limiter = {
            let context = self.context.lock().expect("vm context lock poisoned");
            context.rate_limiter.clone()
        };
        let allowed = rate_limiter
            .lock()
            .expect("rate limiter lock poisoned")
            .allow(&key, limit as u64, window_seconds as u64);
        Ok(CallOutcome::Return(vec![Value::Bool(allowed)]))
    }
}

fn expect_arg_count(args: &[Value], expected: usize) -> Result<(), VmError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(VmError::HostError(format!(
            "expected {expected} arguments, got {}",
            args.len()
        )))
    }
}

fn expect_string(args: &[Value], index: usize) -> Result<String, VmError> {
    match args.get(index) {
        Some(Value::String(value)) => Ok(value.clone()),
        _ => Err(VmError::TypeMismatch("string")),
    }
}

fn expect_int(args: &[Value], index: usize) -> Result<i64, VmError> {
    match args.get(index) {
        Some(Value::Int(value)) => Ok(*value),
        _ => Err(VmError::TypeMismatch("int")),
    }
}

fn is_valid_upstream(value: &str) -> bool {
    if value.is_empty()
        || value.contains('/')
        || value.contains('?')
        || value.contains('#')
        || value.chars().any(|ch| ch.is_whitespace())
    {
        if let Ok(url) = Url::parse(value) {
            if url.scheme() != "http" && url.scheme() != "https" {
                return false;
            }
            if url.host_str().is_none() {
                return false;
            }
            if !url.username().is_empty() || url.password().is_some() {
                return false;
            }
            return true;
        }
        return false;
    }

    let Some((host, port)) = value.rsplit_once(':') else {
        return false;
    };
    if host.is_empty() || port.is_empty() || host.contains(':') {
        return false;
    }
    match port.parse::<u16>() {
        Ok(port) => port != 0,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoopFunction;

    impl HostFunction for NoopFunction {
        fn call(&mut self, _vm: &mut Vm, _args: &[Value]) -> Result<CallOutcome, VmError> {
            Ok(CallOutcome::Return(vec![]))
        }
    }

    fn dummy_vm() -> Vm {
        Vm::new(vm::Program::new(vec![], vec![vm::OpCode::Ret as u8]))
    }

    fn empty_context() -> SharedProxyVmContext {
        Arc::new(Mutex::new(ProxyVmContext::from_request_headers(
            HeaderMap::new(),
            Arc::new(Mutex::new(RateLimiterStore::new())),
        )))
    }

    #[test]
    fn register_host_module_allows_preexisting_bindings() {
        let mut vm = dummy_vm();
        vm.register_function(Box::new(NoopFunction));

        let result = register_host_module(&mut vm, empty_context());
        assert!(result.is_ok());
    }

    #[test]
    fn get_header_reads_request_header_and_returns_empty_if_missing() {
        let mut headers = HeaderMap::new();
        headers.insert("x-hello", HeaderValue::from_static("world"));
        let context = Arc::new(Mutex::new(ProxyVmContext::from_request_headers(
            headers,
            Arc::new(Mutex::new(RateLimiterStore::new())),
        )));
        let mut function = GetHeaderFunction::new(context);
        let mut vm = dummy_vm();

        let present = function
            .call(&mut vm, &[Value::String("x-hello".to_string())])
            .expect("call should succeed");
        assert_eq!(
            present,
            CallOutcome::Return(vec![Value::String("world".to_string())])
        );

        let missing = function
            .call(&mut vm, &[Value::String("x-missing".to_string())])
            .expect("call should succeed");
        assert_eq!(
            missing,
            CallOutcome::Return(vec![Value::String(String::new())])
        );
    }

    #[test]
    fn set_header_stores_response_header() {
        let context = empty_context();
        let mut function = SetHeaderFunction::new(context.clone());
        let mut vm = dummy_vm();

        let result = function.call(
            &mut vm,
            &[
                Value::String("x-set".to_string()),
                Value::String("ok".to_string()),
            ],
        );
        assert!(matches!(result, Ok(CallOutcome::Return(_))));

        let guard = context.lock().expect("vm context lock poisoned");
        let value = guard
            .response_headers
            .get("x-set")
            .and_then(|value| value.to_str().ok());
        assert_eq!(value, Some("ok"));
    }

    #[test]
    fn set_upstream_accepts_valid_and_rejects_invalid_values() {
        let context = empty_context();
        let mut function = SetUpstreamFunction::new(context.clone());
        let mut vm = dummy_vm();

        let ok = function.call(&mut vm, &[Value::String("localhost:8080".to_string())]);
        assert!(matches!(ok, Ok(CallOutcome::Return(_))));
        {
            let guard = context.lock().expect("vm context lock poisoned");
            assert_eq!(guard.upstream.as_deref(), Some("localhost:8080"));
        }

        let ok = function.call(
            &mut vm,
            &[Value::String("https://example.com/path".to_string())],
        );
        assert!(matches!(ok, Ok(CallOutcome::Return(_))));
        {
            let guard = context.lock().expect("vm context lock poisoned");
            assert_eq!(
                guard.upstream.as_deref(),
                Some("https://example.com/path")
            );
        }

        let err = function.call(&mut vm, &[Value::String("ftp://localhost".to_string())]);
        assert!(matches!(err, Err(VmError::HostError(_))));
    }

    #[test]
    fn set_response_content_marks_short_circuit_body() {
        let context = empty_context();
        let mut function = SetResponseContentFunction::new(context.clone());
        let mut vm = dummy_vm();

        let result = function.call(&mut vm, &[Value::String("hello".to_string())]);
        assert!(matches!(result, Ok(CallOutcome::Return(_))));

        let guard = context.lock().expect("vm context lock poisoned");
        assert_eq!(guard.response_content.as_deref(), Some("hello"));
    }

    #[test]
    fn set_response_status_stores_status_code() {
        let context = empty_context();
        let mut function = SetResponseStatusFunction::new(context.clone());
        let mut vm = dummy_vm();

        let ok = function.call(&mut vm, &[Value::Int(429)]);
        assert!(matches!(ok, Ok(CallOutcome::Return(_))));
        {
            let guard = context.lock().expect("vm context lock poisoned");
            assert_eq!(guard.response_status, Some(429));
        }

        let err = function.call(&mut vm, &[Value::Int(42)]);
        assert!(matches!(err, Err(VmError::HostError(_))));
    }

    #[test]
    fn rate_limit_allow_limits_by_key_within_window() {
        let shared_limiter = Arc::new(Mutex::new(RateLimiterStore::new()));
        let context = Arc::new(Mutex::new(ProxyVmContext::from_request_headers(
            HeaderMap::new(),
            shared_limiter,
        )));
        let mut function = RateLimitAllowFunction::new(context);
        let mut vm = dummy_vm();

        let args = [
            Value::String("client-a".to_string()),
            Value::Int(2),
            Value::Int(60),
        ];

        let first = function
            .call(&mut vm, &args)
            .expect("first call should succeed");
        assert_eq!(first, CallOutcome::Return(vec![Value::Bool(true)]));

        let second = function
            .call(&mut vm, &args)
            .expect("second call should succeed");
        assert_eq!(second, CallOutcome::Return(vec![Value::Bool(true)]));

        let third = function
            .call(&mut vm, &args)
            .expect("third call should succeed");
        assert_eq!(third, CallOutcome::Return(vec![Value::Bool(false)]));
    }
}
