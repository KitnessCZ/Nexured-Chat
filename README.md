# Nexured

Nexured is an Android-first messaging project built with Rust. The repository is a Cargo workspace containing a Macroquad client, shared message types, and an Axum + SQLite server.

## Workspace
- `apps/android` — Rust/Macroquad Android client and desktop development preview.
- `crates/shared` — data types shared by client and server.
- `server` — HTTP API backed by SQLite.

## Requirements
- Rust stable (2021 edition)
- For the server: a C toolchain may be needed by SQLite's bundled build.
- For Android builds: Android SDK, NDK, and the Rust Android target for your device ABI.

## Run the server
```sh
cargo run -p nexured-server
```
The server listens on `0.0.0.0:8080` by default and creates `nexured.db` in its working directory. Set `NEXURED_BIND` or `NEXURED_DATABASE_URL` to override these defaults.

- `GET /health` — health check
- `GET /api/messages?limit=50` — latest messages, oldest first
- `POST /api/messages` — create a message; JSON body: `{"sender":"Drume","body":"Hello"}`

Example:
```sh
curl http://127.0.0.1:8080/health
curl -X POST http://127.0.0.1:8080/api/messages \
  -H 'content-type: application/json' \
  -d '{"sender":"Drume","body":"Hello from Nexured"}'
curl 'http://127.0.0.1:8080/api/messages?limit=50'
```

## Run the Android client as a desktop preview
```sh
cargo run -p nexured-android
```
The initial client is an interactive interface preview with local messages. It is not yet connected to the server. Server-backed login, message synchronization, WebSocket delivery, push notifications, and Android packaging are subsequent milestones.

## Development
```sh
cargo fmt --all -- --check
cargo check --workspace
```

## Security status
This is an early development scaffold, not a production-ready messenger. The initial API has no accounts, authorization, rate limiting, TLS deployment configuration, or end-to-end encryption. Do not expose it to the public internet or use it for sensitive conversations until those controls are implemented.
