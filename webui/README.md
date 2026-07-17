# PD Controller Web UI

React/Vite Web UI for `pd-controller`.

The app talks to the controller backend APIs and is intended to run from the controller service during production builds.

## Features

- Edge and program management views
- Program composer workspace
- RustScript Monaco editor integration
- wasm-backed RustScript linting
- Generated code panel and debug session views

## Development

The web app expects a sibling RustScript checkout by default:

```bash
cd /home/wow/rustscript/pd-controller/webui
bun install
bun run dev
```

Set `RUSTSCRIPT_REPO=/path/to/rustscript` if the checkout is elsewhere. Set `VITE_CONTROLLER_URL=http://host:9100` to point the dev proxy at a different controller.

Useful commands:

- `bun run dev` — rebuilds the wasm linter and starts Vite
- `bun run build` — rebuilds wasm, type-checks, and produces `dist/`
- `bun run preview` — serves the built bundle locally

## Production bundle

Build `webui/dist` before compiling `pd-controller` to embed the UI into the controller binary:

```bash
cd /home/wow/rustscript/pd-controller/webui
bun install
bun run build
cd ..
cargo build -p pd-controller --release
```

The controller serves the embedded bundle from `/ui`.
