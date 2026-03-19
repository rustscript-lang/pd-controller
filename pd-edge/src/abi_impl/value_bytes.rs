use vm::{Value, VmError};

pub(crate) fn bytes_to_value(bytes: &[u8]) -> Value {
    Value::bytes(bytes.to_vec())
}

pub(crate) fn value_to_bytes(value: &Value, _label: &str) -> Result<Vec<u8>, VmError> {
    match value {
        Value::Bytes(values) => Ok(values.as_ref().clone()),
        _ => Err(VmError::TypeMismatch("bytes")),
    }
}
