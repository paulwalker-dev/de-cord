# De-cord

A Discord clone

## Requirements

- Have `rustup` installed
- Have the `wasm32-unknown-unknown` Rust toolchain installed
- Have `trunk` and `wasm-bindgen-cli` installed via Cargo
- Optionally install `tauri-cli` version `^1.0.0` via Cargo
- Optionally have `docker` and `docker-compose` installed

## Running the project

### Backend

In the `backend` directory run:

```sh
cargo run
```

### Frontend website

In the `frontend` directory run:

```sh
trunk serve
```

### Frontend app

At the root of the project run:

```sh
cargo tauri dev
```
