# Repo Analysis

- Count basis: physical Rust lines in `src/**/*.rs` plus `build.rs` for each crate.
- Excluded from LOC totals: `tests/`, `examples/`, `docs/`, `target/`, and web assets.
- Feature buckets are source/module areas, not Cargo feature flags. Cargo feature flags are listed separately because they overlap.
- Test counts come from detected `#[test]` and `#[tokio::test]` functions in `src/**/*.rs`, `tests/**/*.rs`, and `build.rs` when present.

Workspace production LOC: **124754**
Detected tests: **913**

## Highlights

- Largest crates by LOC: `pd-vm` (65840 LOC, 570 tests); `pd-edge` (39180 LOC, 247 tests); `pd-controller` (11298 LOC, 32 tests).
- Largest functionality buckets: `pd-edge/abi_impl/http` (11206 LOC); `pd-vm/vm/jit` (8997 LOC); `pd-controller/server/ui_codegen` (7424 LOC); `pd-vm/vm/aot` (7096 LOC); `pd-vm/compiler/parser` (6250 LOC).
- Heaviest test suites: `pd-vm/tests/jit/jit_tests.rs` (60 tests); `pd-vm/tests/compiler/compiler_rustscript_tests.rs` (57 tests); `pd-edge/tests/proxy_tests/http.rs` (55 tests); `pd-vm/tests/compiler/compiler_common_tests.rs` (54 tests); `pd-vm/tests/vm/vm_runtime_tests.rs` (54 tests).

## Crate Summary

| Crate | LOC | Cumulative LOC | Tests | Cargo features |
| --- | ---: | ---: | ---: | --- |
| pd-controller | 11298 | 11298 | 32 | - |
| pd-edge | 39180 | 50478 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge-host-function | 1003 | 51481 | 0 | - |
| pd-edge-abi | 1814 | 53295 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-host-function | 400 | 119535 | 0 | - |
| pd-vm-wasm | 5219 | 124754 | 59 | default, runtime |

## Crate Feature Matrix

| Crate | Crate LOC | Crate Cum LOC | Feature / Functionality | Feature LOC | Feature Cum LOC | Tests | Cargo features |
| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| pd-controller | 11298 | 11298 | server/ui_codegen | 7424 | 7424 | 32 | - |
| pd-controller | 11298 | 11298 | server | 1923 | 9347 | 32 | - |
| pd-controller | 11298 | 11298 | server/handlers | 1682 | 11029 | 32 | - |
| pd-controller | 11298 | 11298 | root | 185 | 11214 | 32 | - |
| pd-controller | 11298 | 11298 | build | 84 | 11298 | 32 | - |
| pd-edge | 39180 | 50478 | abi_impl/http | 11206 | 11206 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/transport | 4383 | 15589 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | runtime/http_plane | 3537 | 19126 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/mqtt | 2329 | 21455 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/http2 | 1894 | 23349 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/websocket | 1837 | 25186 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | sample_echo | 1671 | 26857 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/http3 | 1313 | 28170 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/webrtc | 1091 | 29261 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/proxy | 1058 | 30319 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl | 1034 | 31353 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | bin/pd-edge-console | 995 | 32348 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | bin/pd-edge-http-proxy | 863 | 33211 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | runtime | 829 | 34040 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | debug_session | 772 | 34812 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | runtime/vm_runner | 663 | 35475 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | bin/pd-edge-transport-proxy | 617 | 36092 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | bin/pd-edge-sample-echo-server | 401 | 36493 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/io | 368 | 36861 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | lock_metrics | 347 | 37208 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | active_control_plane | 317 | 37525 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/quic | 293 | 37818 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | cache | 291 | 38109 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | control_plane_rpc | 196 | 38305 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/registry | 150 | 38455 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | runtime/transport_plane | 147 | 38602 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | logging | 112 | 38714 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/console | 110 | 38824 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | control_plane_http_client | 110 | 38934 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | build_info | 67 | 39001 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | root | 60 | 39061 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | compile | 41 | 39102 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/runtime | 38 | 39140 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | build | 30 | 39170 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39180 | 50478 | abi_impl/value_bytes | 10 | 39180 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge-host-function | 1003 | 51481 | edge | 992 | 992 | 0 | - |
| pd-edge-host-function | 1003 | 51481 | root | 11 | 1003 | 0 | - |
| pd-edge-abi | 1814 | 53295 | build | 803 | 803 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | root | 168 | 971 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/http.exchange | 114 | 1085 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/mqtt | 94 | 1179 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/websocket | 84 | 1263 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/tcp | 76 | 1339 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/tls | 76 | 1415 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/webrtc | 76 | 1491 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/http.response | 68 | 1559 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/udp | 68 | 1627 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/http.request | 60 | 1687 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/console | 40 | 1727 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/proxy | 32 | 1759 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/functions | 22 | 1781 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/namespaces | 17 | 1798 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/runtime | 12 | 1810 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1814 | 53295 | abi_spec/http.downstream | 4 | 1814 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/jit | 8997 | 8997 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/aot | 7096 | 16093 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/parser | 6250 | 22343 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/typing | 5584 | 27927 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/frontends | 5142 | 33069 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/native | 4949 | 38018 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/lifetime | 4566 | 42584 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | builtins/runtime | 2933 | 45517 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/source_loader | 2330 | 47847 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/codegen | 1864 | 49711 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | bin/pd-vm-run | 1757 | 51468 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | build | 1752 | 53220 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm | 1572 | 54792 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | debugger/tests | 1130 | 55922 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/pipeline | 1010 | 56932 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | debugger | 964 | 57896 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/host | 866 | 58762 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vmbc | 864 | 59626 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | assembler | 796 | 60422 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | bytecode | 786 | 61208 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/tests | 783 | 61991 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/linker | 515 | 62506 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | debugger/replay | 484 | 62990 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/ir | 471 | 63461 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler | 426 | 63887 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | debugger/recording | 338 | 64225 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/format | 246 | 64471 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/superinstructions | 236 | 64707 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/source_map | 211 | 64918 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/epoch | 181 | 65099 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | debug_info | 179 | 65278 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/fuel | 148 | 65426 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/store | 118 | 65544 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | root | 84 | 65628 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | builtins | 80 | 65708 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | builtins/metadata | 61 | 65769 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | compiler/diagnostics | 50 | 65819 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 65840 | 119135 | vm/diagnostics | 21 | 65840 | 570 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-host-function | 400 | 119535 | root | 400 | 400 | 0 | - |
| pd-vm-wasm | 5219 | 124754 | root | 2354 | 2354 | 59 | default, runtime |
| pd-vm-wasm | 5219 | 124754 | completions | 1287 | 3641 | 59 | default, runtime |
| pd-vm-wasm | 5219 | 124754 | runtime | 1252 | 4893 | 59 | default, runtime |
| pd-vm-wasm | 5219 | 124754 | analyzer | 280 | 5173 | 59 | default, runtime |
| pd-vm-wasm | 5219 | 124754 | stdlib | 46 | 5219 | 59 | default, runtime |

## Crate Test Matrix

| Crate | Suite | Kind | Tests |
| --- | --- | --- | ---: |
| pd-controller | tests/controller_tests/ui.rs | integration | 14 |
| pd-controller | src/main.rs | unit | 7 |
| pd-controller | tests/controller_tests/programs.rs | integration | 5 |
| pd-controller | tests/controller_tests/rpc.rs | integration | 4 |
| pd-controller | tests/controller_tests/debug.rs | integration | 1 |
| pd-controller | tests/e2e_demo_tests.rs | integration | 1 |
| pd-edge | tests/proxy_tests/http.rs | integration | 55 |
| pd-edge | tests/proxy_tests/transport.rs | integration | 24 |
| pd-edge | src/bin/pd-edge-http-proxy.rs | unit | 19 |
| pd-edge | src/bin/pd-edge-console.rs | unit | 18 |
| pd-edge | src/abi_impl/transport/state.rs | unit | 14 |
| pd-edge | src/abi_impl/http/state.rs | unit | 11 |
| pd-edge | tests/compile_tests.rs | integration | 9 |
| pd-edge | src/abi_impl/mod.rs | unit | 8 |
| pd-edge | tests/proxy_tests/tls.rs | integration | 7 |
| pd-edge | tests/proxy_tests/websocket.rs | integration | 7 |
| pd-edge | src/bin/pd-edge-transport-proxy.rs | unit | 6 |
| pd-edge | src/debug_session.rs | unit | 5 |
| pd-edge | tests/proxy_tests/io.rs | integration | 5 |
| pd-edge | tests/sample_echo.rs | integration | 5 |
| pd-edge | src/abi_impl/http/fast_path.rs | unit | 4 |
| pd-edge | src/abi_impl/http2/model.rs | unit | 4 |
| pd-edge | src/abi_impl/http2/upstream.rs | unit | 4 |
| pd-edge | src/bin/pd-edge-sample-echo-server.rs | unit | 4 |
| pd-edge | src/cache.rs | unit | 4 |
| pd-edge | src/runtime/http_plane/proxy_path.rs | unit | 4 |
| pd-edge | src/runtime.rs | unit | 3 |
| pd-edge | src/runtime/vm_runner.rs | unit | 3 |
| pd-edge | tests/proxy_tests/debug.rs | integration | 3 |
| pd-edge | src/abi_impl/http/outbound_http1.rs | unit | 2 |
| pd-edge | src/abi_impl/mqtt/upstream.rs | unit | 2 |
| pd-edge | src/abi_impl/proxy.rs | unit | 2 |
| pd-edge | src/abi_impl/transport/mod.rs | unit | 2 |
| pd-edge | src/abi_impl/websocket/state.rs | unit | 2 |
| pd-edge | src/build_info.rs | unit | 2 |
| pd-edge | tests/proxy_tests/attach_transport.rs | integration | 2 |
| pd-edge | tests/proxy_tests/mqtt.rs | integration | 2 |
| pd-edge | tests/proxy_tests/webrtc.rs | integration | 2 |
| pd-edge | src/abi_impl/webrtc/mod.rs | unit | 1 |
| pd-edge | tests/proxy_tests/control_plane.rs | integration | 1 |
| pd-edge | tests/proxy_tests/forward_proxy.rs | integration | 1 |
| pd-edge-host-function | _none_ | - | 0 |
| pd-edge-abi | src/lib.rs | unit | 5 |
| pd-vm | tests/jit/jit_tests.rs | integration | 60 |
| pd-vm | tests/compiler/compiler_rustscript_tests.rs | integration | 57 |
| pd-vm | tests/compiler/compiler_common_tests.rs | integration | 54 |
| pd-vm | tests/vm/vm_runtime_tests.rs | integration | 54 |
| pd-vm | src/bin/pd-vm-run.rs | unit | 53 |
| pd-vm | src/debugger/tests.rs | unit | 24 |
| pd-vm | src/vm/tests.rs | unit | 22 |
| pd-vm | tests/vm/drop_contract_tests.rs | integration | 22 |
| pd-vm | tests/compiler/compiler_javascript_tests.rs | integration | 17 |
| pd-vm | tests/compiler/diagnostics_tests.rs | integration | 16 |
| pd-vm | src/compiler/format.rs | unit | 12 |
| pd-vm | tests/builtins/stdlib_tests.rs | integration | 12 |
| pd-vm | tests/wire/wire_tests.rs | integration | 12 |
| pd-vm | tests/wire/assembler_vmbc_edge_tests.rs | integration | 11 |
| pd-vm | tests/jit/perf_tests.rs | integration | 10 |
| pd-vm | tests/jit/jit_nyi_edge_tests.rs | integration | 8 |
| pd-vm | tests/vm/runtime_state_edge_tests.rs | integration | 8 |
| pd-vm | src/builtins/runtime/core.rs | unit | 7 |
| pd-vm | src/builtins/runtime/host.rs | unit | 7 |
| pd-vm | tests/builtins/io_builtin_edge_tests.rs | integration | 7 |
| pd-vm | tests/compiler/module_import_tests.rs | integration | 7 |
| pd-vm | tests/example_tests.rs | integration | 7 |
| pd-vm | tests/vm/functional_parity_tests.rs | integration | 7 |
| pd-vm | tests/compiler/compiler_lua_tests.rs | integration | 6 |
| pd-vm | tests/compiler/type_inference_tests.rs | integration | 6 |
| pd-vm | src/builtins/runtime/math.rs | unit | 5 |
| pd-vm | src/bytecode.rs | unit | 5 |
| pd-vm | src/vm/aot/ir.rs | unit | 5 |
| pd-vm | tests/compiler/compiler_scheme_tests.rs | integration | 5 |
| pd-vm | src/builtins/runtime/bytes.rs | unit | 4 |
| pd-vm | tests/compiler/whitespace_resilience_tests.rs | integration | 4 |
| pd-vm | src/builtins/runtime/print.rs | unit | 3 |
| pd-vm | src/builtins/runtime/typed.rs | unit | 3 |
| pd-vm | src/compiler/source_loader.rs | unit | 3 |
| pd-vm | src/compiler/source_loader/rewrite.rs | unit | 3 |
| pd-vm | src/vm/aot/artifact.rs | unit | 3 |
| pd-vm | src/vm/aot/cfg.rs | unit | 3 |
| pd-vm | src/vm/jit/recorder.rs | unit | 3 |
| pd-vm | src/builtins/runtime/aot.rs | unit | 2 |
| pd-vm | src/compiler/parser/format.rs | unit | 2 |
| pd-vm | src/vm/jit/ir.rs | unit | 2 |
| pd-vm | tests/vm/vm_async_runtime_tests.rs | integration | 2 |
| pd-vm | src/builtins/runtime/mod.rs | unit | 1 |
| pd-vm | src/compiler/frontends/scheme.rs | unit | 1 |
| pd-vm | src/debug_info.rs | unit | 1 |
| pd-vm | src/vm/aot/ssa.rs | unit | 1 |
| pd-vm | src/vm/jit/native/mod.rs | unit | 1 |
| pd-vm | src/vm/jit/trace.rs | unit | 1 |
| pd-vm | tests/common/mod.rs | integration | 1 |
| pd-host-function | _none_ | - | 0 |
| pd-vm-wasm | src/lib.rs | unit | 50 |
| pd-vm-wasm | src/completions.rs | unit | 9 |

## pd-controller

- Manifest: `pd-controller/Cargo.toml`
- LOC: **11298**
- Cumulative workspace LOC at this crate: **11298**
- Cargo features: -
- Tests: **32**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| server/ui_codegen | 7424 | 7424 |
| server | 1923 | 9347 |
| server/handlers | 1682 | 11029 |
| root | 185 | 11214 |
| build | 84 | 11298 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| tests/controller_tests/ui.rs | integration | 14 |
| src/main.rs | unit | 7 |
| tests/controller_tests/programs.rs | integration | 5 |
| tests/controller_tests/rpc.rs | integration | 4 |
| tests/controller_tests/debug.rs | integration | 1 |
| tests/e2e_demo_tests.rs | integration | 1 |

## pd-edge

- Manifest: `pd-edge/Cargo.toml`
- LOC: **39180**
- Cumulative workspace LOC at this crate: **50478**
- Cargo features: console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket
- Tests: **247**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| abi_impl/http | 11206 | 11206 |
| abi_impl/transport | 4383 | 15589 |
| runtime/http_plane | 3537 | 19126 |
| abi_impl/mqtt | 2329 | 21455 |
| abi_impl/http2 | 1894 | 23349 |
| abi_impl/websocket | 1837 | 25186 |
| sample_echo | 1671 | 26857 |
| abi_impl/http3 | 1313 | 28170 |
| abi_impl/webrtc | 1091 | 29261 |
| abi_impl/proxy | 1058 | 30319 |
| abi_impl | 1034 | 31353 |
| bin/pd-edge-console | 995 | 32348 |
| bin/pd-edge-http-proxy | 863 | 33211 |
| runtime | 829 | 34040 |
| debug_session | 772 | 34812 |
| runtime/vm_runner | 663 | 35475 |
| bin/pd-edge-transport-proxy | 617 | 36092 |
| bin/pd-edge-sample-echo-server | 401 | 36493 |
| abi_impl/io | 368 | 36861 |
| lock_metrics | 347 | 37208 |
| active_control_plane | 317 | 37525 |
| abi_impl/quic | 293 | 37818 |
| cache | 291 | 38109 |
| control_plane_rpc | 196 | 38305 |
| abi_impl/registry | 150 | 38455 |
| runtime/transport_plane | 147 | 38602 |
| logging | 112 | 38714 |
| abi_impl/console | 110 | 38824 |
| control_plane_http_client | 110 | 38934 |
| build_info | 67 | 39001 |
| root | 60 | 39061 |
| compile | 41 | 39102 |
| abi_impl/runtime | 38 | 39140 |
| build | 30 | 39170 |
| abi_impl/value_bytes | 10 | 39180 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| tests/proxy_tests/http.rs | integration | 55 |
| tests/proxy_tests/transport.rs | integration | 24 |
| src/bin/pd-edge-http-proxy.rs | unit | 19 |
| src/bin/pd-edge-console.rs | unit | 18 |
| src/abi_impl/transport/state.rs | unit | 14 |
| src/abi_impl/http/state.rs | unit | 11 |
| tests/compile_tests.rs | integration | 9 |
| src/abi_impl/mod.rs | unit | 8 |
| tests/proxy_tests/tls.rs | integration | 7 |
| tests/proxy_tests/websocket.rs | integration | 7 |
| src/bin/pd-edge-transport-proxy.rs | unit | 6 |
| src/debug_session.rs | unit | 5 |
| tests/proxy_tests/io.rs | integration | 5 |
| tests/sample_echo.rs | integration | 5 |
| src/abi_impl/http/fast_path.rs | unit | 4 |
| src/abi_impl/http2/model.rs | unit | 4 |
| src/abi_impl/http2/upstream.rs | unit | 4 |
| src/bin/pd-edge-sample-echo-server.rs | unit | 4 |
| src/cache.rs | unit | 4 |
| src/runtime/http_plane/proxy_path.rs | unit | 4 |
| src/runtime.rs | unit | 3 |
| src/runtime/vm_runner.rs | unit | 3 |
| tests/proxy_tests/debug.rs | integration | 3 |
| src/abi_impl/http/outbound_http1.rs | unit | 2 |
| src/abi_impl/mqtt/upstream.rs | unit | 2 |
| src/abi_impl/proxy.rs | unit | 2 |
| src/abi_impl/transport/mod.rs | unit | 2 |
| src/abi_impl/websocket/state.rs | unit | 2 |
| src/build_info.rs | unit | 2 |
| tests/proxy_tests/attach_transport.rs | integration | 2 |
| tests/proxy_tests/mqtt.rs | integration | 2 |
| tests/proxy_tests/webrtc.rs | integration | 2 |
| src/abi_impl/webrtc/mod.rs | unit | 1 |
| tests/proxy_tests/control_plane.rs | integration | 1 |
| tests/proxy_tests/forward_proxy.rs | integration | 1 |

## pd-edge-host-function

- Manifest: `pd-edge/pd-edge-host-function/Cargo.toml`
- LOC: **1003**
- Cumulative workspace LOC at this crate: **51481**
- Cargo features: -
- Tests: **0**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| edge | 992 | 992 |
| root | 11 | 1003 |

### Test Suites

_No detected tests._

## pd-edge-abi

- Manifest: `pd-edge-abi/Cargo.toml`
- LOC: **1814**
- Cumulative workspace LOC at this crate: **53295**
- Cargo features: console, default, http, http2, mqtt, tls, webrtc, websocket
- Tests: **5**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| build | 803 | 803 |
| root | 168 | 971 |
| abi_spec/http.exchange | 114 | 1085 |
| abi_spec/mqtt | 94 | 1179 |
| abi_spec/websocket | 84 | 1263 |
| abi_spec/tcp | 76 | 1339 |
| abi_spec/tls | 76 | 1415 |
| abi_spec/webrtc | 76 | 1491 |
| abi_spec/http.response | 68 | 1559 |
| abi_spec/udp | 68 | 1627 |
| abi_spec/http.request | 60 | 1687 |
| abi_spec/console | 40 | 1727 |
| abi_spec/proxy | 32 | 1759 |
| abi_spec/functions | 22 | 1781 |
| abi_spec/namespaces | 17 | 1798 |
| abi_spec/runtime | 12 | 1810 |
| abi_spec/http.downstream | 4 | 1814 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| src/lib.rs | unit | 5 |

## pd-vm

- Manifest: `pd-vm/Cargo.toml`
- LOC: **65840**
- Cumulative workspace LOC at this crate: **119135**
- Cargo features: cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket
- Tests: **570**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| vm/jit | 8997 | 8997 |
| vm/aot | 7096 | 16093 |
| compiler/parser | 6250 | 22343 |
| compiler/typing | 5584 | 27927 |
| compiler/frontends | 5142 | 33069 |
| vm/native | 4949 | 38018 |
| compiler/lifetime | 4566 | 42584 |
| builtins/runtime | 2933 | 45517 |
| compiler/source_loader | 2330 | 47847 |
| compiler/codegen | 1864 | 49711 |
| bin/pd-vm-run | 1757 | 51468 |
| build | 1752 | 53220 |
| vm | 1572 | 54792 |
| debugger/tests | 1130 | 55922 |
| compiler/pipeline | 1010 | 56932 |
| debugger | 964 | 57896 |
| vm/host | 866 | 58762 |
| vmbc | 864 | 59626 |
| assembler | 796 | 60422 |
| bytecode | 786 | 61208 |
| vm/tests | 783 | 61991 |
| compiler/linker | 515 | 62506 |
| debugger/replay | 484 | 62990 |
| compiler/ir | 471 | 63461 |
| compiler | 426 | 63887 |
| debugger/recording | 338 | 64225 |
| compiler/format | 246 | 64471 |
| vm/superinstructions | 236 | 64707 |
| compiler/source_map | 211 | 64918 |
| vm/epoch | 181 | 65099 |
| debug_info | 179 | 65278 |
| vm/fuel | 148 | 65426 |
| vm/store | 118 | 65544 |
| root | 84 | 65628 |
| builtins | 80 | 65708 |
| builtins/metadata | 61 | 65769 |
| compiler/diagnostics | 50 | 65819 |
| vm/diagnostics | 21 | 65840 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| tests/jit/jit_tests.rs | integration | 60 |
| tests/compiler/compiler_rustscript_tests.rs | integration | 57 |
| tests/compiler/compiler_common_tests.rs | integration | 54 |
| tests/vm/vm_runtime_tests.rs | integration | 54 |
| src/bin/pd-vm-run.rs | unit | 53 |
| src/debugger/tests.rs | unit | 24 |
| src/vm/tests.rs | unit | 22 |
| tests/vm/drop_contract_tests.rs | integration | 22 |
| tests/compiler/compiler_javascript_tests.rs | integration | 17 |
| tests/compiler/diagnostics_tests.rs | integration | 16 |
| src/compiler/format.rs | unit | 12 |
| tests/builtins/stdlib_tests.rs | integration | 12 |
| tests/wire/wire_tests.rs | integration | 12 |
| tests/wire/assembler_vmbc_edge_tests.rs | integration | 11 |
| tests/jit/perf_tests.rs | integration | 10 |
| tests/jit/jit_nyi_edge_tests.rs | integration | 8 |
| tests/vm/runtime_state_edge_tests.rs | integration | 8 |
| src/builtins/runtime/core.rs | unit | 7 |
| src/builtins/runtime/host.rs | unit | 7 |
| tests/builtins/io_builtin_edge_tests.rs | integration | 7 |
| tests/compiler/module_import_tests.rs | integration | 7 |
| tests/example_tests.rs | integration | 7 |
| tests/vm/functional_parity_tests.rs | integration | 7 |
| tests/compiler/compiler_lua_tests.rs | integration | 6 |
| tests/compiler/type_inference_tests.rs | integration | 6 |
| src/builtins/runtime/math.rs | unit | 5 |
| src/bytecode.rs | unit | 5 |
| src/vm/aot/ir.rs | unit | 5 |
| tests/compiler/compiler_scheme_tests.rs | integration | 5 |
| src/builtins/runtime/bytes.rs | unit | 4 |
| tests/compiler/whitespace_resilience_tests.rs | integration | 4 |
| src/builtins/runtime/print.rs | unit | 3 |
| src/builtins/runtime/typed.rs | unit | 3 |
| src/compiler/source_loader.rs | unit | 3 |
| src/compiler/source_loader/rewrite.rs | unit | 3 |
| src/vm/aot/artifact.rs | unit | 3 |
| src/vm/aot/cfg.rs | unit | 3 |
| src/vm/jit/recorder.rs | unit | 3 |
| src/builtins/runtime/aot.rs | unit | 2 |
| src/compiler/parser/format.rs | unit | 2 |
| src/vm/jit/ir.rs | unit | 2 |
| tests/vm/vm_async_runtime_tests.rs | integration | 2 |
| src/builtins/runtime/mod.rs | unit | 1 |
| src/compiler/frontends/scheme.rs | unit | 1 |
| src/debug_info.rs | unit | 1 |
| src/vm/aot/ssa.rs | unit | 1 |
| src/vm/jit/native/mod.rs | unit | 1 |
| src/vm/jit/trace.rs | unit | 1 |
| tests/common/mod.rs | integration | 1 |

## pd-host-function

- Manifest: `pd-vm/pd-host-function/Cargo.toml`
- LOC: **400**
- Cumulative workspace LOC at this crate: **119535**
- Cargo features: -
- Tests: **0**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| root | 400 | 400 |

### Test Suites

_No detected tests._

## pd-vm-wasm

- Manifest: `pd-vm/pd-vm-wasm/Cargo.toml`
- LOC: **5219**
- Cumulative workspace LOC at this crate: **124754**
- Cargo features: default, runtime
- Tests: **59**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| root | 2354 | 2354 |
| completions | 1287 | 3641 |
| runtime | 1252 | 4893 |
| analyzer | 280 | 5173 |
| stdlib | 46 | 5219 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| src/lib.rs | unit | 50 |
| src/completions.rs | unit | 9 |
