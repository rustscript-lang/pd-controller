# Repo Analysis

- Count basis: physical Rust lines in `src/**/*.rs` plus `build.rs` for each crate.
- Excluded from LOC totals: `tests/`, `examples/`, `docs/`, `target/`, and web assets.
- Feature buckets are source/module areas, not Cargo feature flags. Cargo feature flags are listed separately because they overlap.
- Test counts come from detected `#[test]` and `#[tokio::test]` functions in `src/**/*.rs`, `tests/**/*.rs`, and `build.rs` when present.

Workspace production LOC: **109650**
Detected tests: **842**

## Highlights

- Largest crates by LOC: `pd-vm` (50809 LOC, 501 tests); `pd-edge` (39164 LOC, 247 tests); `pd-controller` (11298 LOC, 32 tests).
- Largest functionality buckets: `pd-edge/abi_impl/http` (11206 LOC); `pd-controller/server/ui_codegen` (7424 LOC); `pd-vm/vm/jit` (6929 LOC); `pd-vm/compiler/parser` (6102 LOC); `pd-vm/compiler/typing` (5576 LOC).
- Heaviest test suites: `pd-vm/tests/compiler/compiler_rustscript_tests.rs` (57 tests); `pd-edge/tests/proxy_tests/http.rs` (55 tests); `pd-vm/tests/compiler/compiler_common_tests.rs` (54 tests); `pd-vm/tests/vm/vm_runtime_tests.rs` (54 tests); `pd-vm-wasm/src/lib.rs` (48 tests).

## Crate Summary

| Crate | LOC | Cumulative LOC | Tests | Cargo features |
| --- | ---: | ---: | ---: | --- |
| pd-controller | 11298 | 11298 | 32 | - |
| pd-edge | 39164 | 50462 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge-host-function | 1003 | 51465 | 0 | - |
| pd-edge-abi | 1804 | 53269 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-host-function | 387 | 104465 | 0 | - |
| pd-vm-wasm | 5185 | 109650 | 57 | default, runtime |

## Crate Feature Matrix

| Crate | Crate LOC | Crate Cum LOC | Feature / Functionality | Feature LOC | Feature Cum LOC | Tests | Cargo features |
| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| pd-controller | 11298 | 11298 | server/ui_codegen | 7424 | 7424 | 32 | - |
| pd-controller | 11298 | 11298 | server | 1923 | 9347 | 32 | - |
| pd-controller | 11298 | 11298 | server/handlers | 1682 | 11029 | 32 | - |
| pd-controller | 11298 | 11298 | root | 185 | 11214 | 32 | - |
| pd-controller | 11298 | 11298 | build | 84 | 11298 | 32 | - |
| pd-edge | 39164 | 50462 | abi_impl/http | 11206 | 11206 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/transport | 4383 | 15589 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | runtime/http_plane | 3537 | 19126 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/mqtt | 2313 | 21439 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/http2 | 1894 | 23333 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/websocket | 1837 | 25170 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | sample_echo | 1671 | 26841 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/http3 | 1313 | 28154 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/webrtc | 1091 | 29245 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/proxy | 1058 | 30303 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl | 1034 | 31337 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | bin/pd-edge-console | 995 | 32332 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | bin/pd-edge-http-proxy | 863 | 33195 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | runtime | 829 | 34024 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | debug_session | 772 | 34796 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | runtime/vm_runner | 663 | 35459 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | bin/pd-edge-transport-proxy | 617 | 36076 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | bin/pd-edge-sample-echo-server | 401 | 36477 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/io | 368 | 36845 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | lock_metrics | 347 | 37192 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | active_control_plane | 317 | 37509 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/quic | 293 | 37802 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | cache | 291 | 38093 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | control_plane_rpc | 196 | 38289 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/registry | 150 | 38439 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | runtime/transport_plane | 147 | 38586 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | logging | 112 | 38698 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/console | 110 | 38808 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | control_plane_http_client | 110 | 38918 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | build_info | 67 | 38985 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | root | 60 | 39045 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | compile | 41 | 39086 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/runtime | 38 | 39124 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | build | 30 | 39154 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge | 39164 | 50462 | abi_impl/value_bytes | 10 | 39164 | 247 | console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket |
| pd-edge-host-function | 1003 | 51465 | edge | 992 | 992 | 0 | - |
| pd-edge-host-function | 1003 | 51465 | root | 11 | 1003 | 0 | - |
| pd-edge-abi | 1804 | 53269 | build | 803 | 803 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | root | 168 | 971 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/http.exchange | 114 | 1085 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/mqtt | 84 | 1169 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/websocket | 84 | 1253 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/tcp | 76 | 1329 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/tls | 76 | 1405 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/webrtc | 76 | 1481 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/http.response | 68 | 1549 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/udp | 68 | 1617 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/http.request | 60 | 1677 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/console | 40 | 1717 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/proxy | 32 | 1749 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/functions | 22 | 1771 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/namespaces | 17 | 1788 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/runtime | 12 | 1800 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-edge-abi | 1804 | 53269 | abi_spec/http.downstream | 4 | 1804 | 5 | console, default, http, http2, mqtt, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/jit | 6929 | 6929 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/parser | 6102 | 13031 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/typing | 5576 | 18607 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/frontends | 5142 | 23749 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/lifetime | 4551 | 28300 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | builtins/runtime | 2826 | 31126 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/source_loader | 2329 | 33455 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/codegen | 1837 | 35292 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | build | 1738 | 37030 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | bin/pd-vm-run | 1540 | 38570 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm | 1240 | 39810 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | debugger/tests | 1130 | 40940 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/pipeline | 1009 | 41949 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | debugger | 964 | 42913 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/host | 866 | 43779 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vmbc | 864 | 44643 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | assembler | 796 | 45439 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/tests | 783 | 46222 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | bytecode | 759 | 46981 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/linker | 510 | 47491 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | debugger/replay | 484 | 47975 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/ir | 469 | 48444 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler | 426 | 48870 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | debugger/recording | 338 | 49208 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/format | 246 | 49454 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/superinstructions | 236 | 49690 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/source_map | 211 | 49901 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/epoch | 181 | 50082 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | debug_info | 179 | 50261 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/fuel | 148 | 50409 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/store | 118 | 50527 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | builtins | 80 | 50607 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | root | 70 | 50677 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | builtins/metadata | 61 | 50738 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | compiler/diagnostics | 50 | 50788 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-vm | 50809 | 104078 | vm/diagnostics | 21 | 50809 | 501 | cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket |
| pd-host-function | 387 | 104465 | root | 387 | 387 | 0 | - |
| pd-vm-wasm | 5185 | 109650 | root | 2320 | 2320 | 57 | default, runtime |
| pd-vm-wasm | 5185 | 109650 | completions | 1287 | 3607 | 57 | default, runtime |
| pd-vm-wasm | 5185 | 109650 | runtime | 1252 | 4859 | 57 | default, runtime |
| pd-vm-wasm | 5185 | 109650 | analyzer | 280 | 5139 | 57 | default, runtime |
| pd-vm-wasm | 5185 | 109650 | stdlib | 46 | 5185 | 57 | default, runtime |

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
| pd-vm | tests/compiler/compiler_rustscript_tests.rs | integration | 57 |
| pd-vm | tests/compiler/compiler_common_tests.rs | integration | 54 |
| pd-vm | tests/vm/vm_runtime_tests.rs | integration | 54 |
| pd-vm | src/bin/pd-vm-run.rs | unit | 47 |
| pd-vm | src/debugger/tests.rs | unit | 24 |
| pd-vm | src/vm/tests.rs | unit | 22 |
| pd-vm | tests/vm/drop_contract_tests.rs | integration | 22 |
| pd-vm | tests/jit/jit_tests.rs | integration | 21 |
| pd-vm | tests/compiler/compiler_javascript_tests.rs | integration | 17 |
| pd-vm | tests/compiler/diagnostics_tests.rs | integration | 16 |
| pd-vm | src/compiler/format.rs | unit | 12 |
| pd-vm | tests/builtins/stdlib_tests.rs | integration | 12 |
| pd-vm | tests/wire/wire_tests.rs | integration | 12 |
| pd-vm | tests/wire/assembler_vmbc_edge_tests.rs | integration | 11 |
| pd-vm | tests/vm/runtime_state_edge_tests.rs | integration | 8 |
| pd-vm | src/builtins/runtime/core.rs | unit | 7 |
| pd-vm | src/builtins/runtime/host.rs | unit | 7 |
| pd-vm | tests/builtins/io_builtin_edge_tests.rs | integration | 7 |
| pd-vm | tests/compiler/module_import_tests.rs | integration | 7 |
| pd-vm | tests/example_tests.rs | integration | 7 |
| pd-vm | tests/jit/jit_nyi_edge_tests.rs | integration | 7 |
| pd-vm | tests/vm/functional_parity_tests.rs | integration | 7 |
| pd-vm | tests/compiler/compiler_lua_tests.rs | integration | 6 |
| pd-vm | tests/compiler/type_inference_tests.rs | integration | 6 |
| pd-vm | tests/jit/perf_tests.rs | integration | 6 |
| pd-vm | src/builtins/runtime/math.rs | unit | 5 |
| pd-vm | tests/compiler/compiler_scheme_tests.rs | integration | 5 |
| pd-vm | src/builtins/runtime/bytes.rs | unit | 4 |
| pd-vm | src/bytecode.rs | unit | 4 |
| pd-vm | tests/compiler/whitespace_resilience_tests.rs | integration | 4 |
| pd-vm | src/builtins/runtime/print.rs | unit | 3 |
| pd-vm | src/builtins/runtime/typed.rs | unit | 3 |
| pd-vm | src/compiler/source_loader.rs | unit | 3 |
| pd-vm | src/compiler/source_loader/rewrite.rs | unit | 3 |
| pd-vm | src/compiler/parser/format.rs | unit | 2 |
| pd-vm | tests/vm/vm_async_runtime_tests.rs | integration | 2 |
| pd-vm | src/builtins/runtime/mod.rs | unit | 1 |
| pd-vm | src/compiler/frontends/scheme.rs | unit | 1 |
| pd-vm | src/debug_info.rs | unit | 1 |
| pd-vm | src/vm/jit/native/cranelift.rs | unit | 1 |
| pd-vm | src/vm/jit/native/mod.rs | unit | 1 |
| pd-vm | src/vm/jit/trace.rs | unit | 1 |
| pd-vm | tests/common/mod.rs | integration | 1 |
| pd-host-function | _none_ | - | 0 |
| pd-vm-wasm | src/lib.rs | unit | 48 |
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
- LOC: **39164**
- Cumulative workspace LOC at this crate: **50462**
- Cargo features: console, default, http, http2, http3, mqtt, native-jit, tls, webrtc, websocket
- Tests: **247**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| abi_impl/http | 11206 | 11206 |
| abi_impl/transport | 4383 | 15589 |
| runtime/http_plane | 3537 | 19126 |
| abi_impl/mqtt | 2313 | 21439 |
| abi_impl/http2 | 1894 | 23333 |
| abi_impl/websocket | 1837 | 25170 |
| sample_echo | 1671 | 26841 |
| abi_impl/http3 | 1313 | 28154 |
| abi_impl/webrtc | 1091 | 29245 |
| abi_impl/proxy | 1058 | 30303 |
| abi_impl | 1034 | 31337 |
| bin/pd-edge-console | 995 | 32332 |
| bin/pd-edge-http-proxy | 863 | 33195 |
| runtime | 829 | 34024 |
| debug_session | 772 | 34796 |
| runtime/vm_runner | 663 | 35459 |
| bin/pd-edge-transport-proxy | 617 | 36076 |
| bin/pd-edge-sample-echo-server | 401 | 36477 |
| abi_impl/io | 368 | 36845 |
| lock_metrics | 347 | 37192 |
| active_control_plane | 317 | 37509 |
| abi_impl/quic | 293 | 37802 |
| cache | 291 | 38093 |
| control_plane_rpc | 196 | 38289 |
| abi_impl/registry | 150 | 38439 |
| runtime/transport_plane | 147 | 38586 |
| logging | 112 | 38698 |
| abi_impl/console | 110 | 38808 |
| control_plane_http_client | 110 | 38918 |
| build_info | 67 | 38985 |
| root | 60 | 39045 |
| compile | 41 | 39086 |
| abi_impl/runtime | 38 | 39124 |
| build | 30 | 39154 |
| abi_impl/value_bytes | 10 | 39164 |

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
- Cumulative workspace LOC at this crate: **51465**
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
- LOC: **1804**
- Cumulative workspace LOC at this crate: **53269**
- Cargo features: console, default, http, http2, mqtt, tls, webrtc, websocket
- Tests: **5**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| build | 803 | 803 |
| root | 168 | 971 |
| abi_spec/http.exchange | 114 | 1085 |
| abi_spec/mqtt | 84 | 1169 |
| abi_spec/websocket | 84 | 1253 |
| abi_spec/tcp | 76 | 1329 |
| abi_spec/tls | 76 | 1405 |
| abi_spec/webrtc | 76 | 1481 |
| abi_spec/http.response | 68 | 1549 |
| abi_spec/udp | 68 | 1617 |
| abi_spec/http.request | 60 | 1677 |
| abi_spec/console | 40 | 1717 |
| abi_spec/proxy | 32 | 1749 |
| abi_spec/functions | 22 | 1771 |
| abi_spec/namespaces | 17 | 1788 |
| abi_spec/runtime | 12 | 1800 |
| abi_spec/http.downstream | 4 | 1804 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| src/lib.rs | unit | 5 |

## pd-vm

- Manifest: `pd-vm/Cargo.toml`
- LOC: **50809**
- Cumulative workspace LOC at this crate: **104078**
- Cargo features: cli, cranelift-jit, default, http, http2, mqtt, runtime, tls, webrtc, websocket
- Tests: **501**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| vm/jit | 6929 | 6929 |
| compiler/parser | 6102 | 13031 |
| compiler/typing | 5576 | 18607 |
| compiler/frontends | 5142 | 23749 |
| compiler/lifetime | 4551 | 28300 |
| builtins/runtime | 2826 | 31126 |
| compiler/source_loader | 2329 | 33455 |
| compiler/codegen | 1837 | 35292 |
| build | 1738 | 37030 |
| bin/pd-vm-run | 1540 | 38570 |
| vm | 1240 | 39810 |
| debugger/tests | 1130 | 40940 |
| compiler/pipeline | 1009 | 41949 |
| debugger | 964 | 42913 |
| vm/host | 866 | 43779 |
| vmbc | 864 | 44643 |
| assembler | 796 | 45439 |
| vm/tests | 783 | 46222 |
| bytecode | 759 | 46981 |
| compiler/linker | 510 | 47491 |
| debugger/replay | 484 | 47975 |
| compiler/ir | 469 | 48444 |
| compiler | 426 | 48870 |
| debugger/recording | 338 | 49208 |
| compiler/format | 246 | 49454 |
| vm/superinstructions | 236 | 49690 |
| compiler/source_map | 211 | 49901 |
| vm/epoch | 181 | 50082 |
| debug_info | 179 | 50261 |
| vm/fuel | 148 | 50409 |
| vm/store | 118 | 50527 |
| builtins | 80 | 50607 |
| root | 70 | 50677 |
| builtins/metadata | 61 | 50738 |
| compiler/diagnostics | 50 | 50788 |
| vm/diagnostics | 21 | 50809 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| tests/compiler/compiler_rustscript_tests.rs | integration | 57 |
| tests/compiler/compiler_common_tests.rs | integration | 54 |
| tests/vm/vm_runtime_tests.rs | integration | 54 |
| src/bin/pd-vm-run.rs | unit | 47 |
| src/debugger/tests.rs | unit | 24 |
| src/vm/tests.rs | unit | 22 |
| tests/vm/drop_contract_tests.rs | integration | 22 |
| tests/jit/jit_tests.rs | integration | 21 |
| tests/compiler/compiler_javascript_tests.rs | integration | 17 |
| tests/compiler/diagnostics_tests.rs | integration | 16 |
| src/compiler/format.rs | unit | 12 |
| tests/builtins/stdlib_tests.rs | integration | 12 |
| tests/wire/wire_tests.rs | integration | 12 |
| tests/wire/assembler_vmbc_edge_tests.rs | integration | 11 |
| tests/vm/runtime_state_edge_tests.rs | integration | 8 |
| src/builtins/runtime/core.rs | unit | 7 |
| src/builtins/runtime/host.rs | unit | 7 |
| tests/builtins/io_builtin_edge_tests.rs | integration | 7 |
| tests/compiler/module_import_tests.rs | integration | 7 |
| tests/example_tests.rs | integration | 7 |
| tests/jit/jit_nyi_edge_tests.rs | integration | 7 |
| tests/vm/functional_parity_tests.rs | integration | 7 |
| tests/compiler/compiler_lua_tests.rs | integration | 6 |
| tests/compiler/type_inference_tests.rs | integration | 6 |
| tests/jit/perf_tests.rs | integration | 6 |
| src/builtins/runtime/math.rs | unit | 5 |
| tests/compiler/compiler_scheme_tests.rs | integration | 5 |
| src/builtins/runtime/bytes.rs | unit | 4 |
| src/bytecode.rs | unit | 4 |
| tests/compiler/whitespace_resilience_tests.rs | integration | 4 |
| src/builtins/runtime/print.rs | unit | 3 |
| src/builtins/runtime/typed.rs | unit | 3 |
| src/compiler/source_loader.rs | unit | 3 |
| src/compiler/source_loader/rewrite.rs | unit | 3 |
| src/compiler/parser/format.rs | unit | 2 |
| tests/vm/vm_async_runtime_tests.rs | integration | 2 |
| src/builtins/runtime/mod.rs | unit | 1 |
| src/compiler/frontends/scheme.rs | unit | 1 |
| src/debug_info.rs | unit | 1 |
| src/vm/jit/native/cranelift.rs | unit | 1 |
| src/vm/jit/native/mod.rs | unit | 1 |
| src/vm/jit/trace.rs | unit | 1 |
| tests/common/mod.rs | integration | 1 |

## pd-host-function

- Manifest: `pd-vm/pd-host-function/Cargo.toml`
- LOC: **387**
- Cumulative workspace LOC at this crate: **104465**
- Cargo features: -
- Tests: **0**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| root | 387 | 387 |

### Test Suites

_No detected tests._

## pd-vm-wasm

- Manifest: `pd-vm/pd-vm-wasm/Cargo.toml`
- LOC: **5185**
- Cumulative workspace LOC at this crate: **109650**
- Cargo features: default, runtime
- Tests: **57**

### Feature Areas

| Feature area | LOC | Cumulative in crate |
| --- | ---: | ---: |
| root | 2320 | 2320 |
| completions | 1287 | 3607 |
| runtime | 1252 | 4859 |
| analyzer | 280 | 5139 |
| stdlib | 46 | 5185 |

### Test Suites

| Suite | Kind | Tests |
| --- | --- | ---: |
| src/lib.rs | unit | 48 |
| src/completions.rs | unit | 9 |
