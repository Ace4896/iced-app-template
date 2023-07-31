# iced-app-template

A template project for [Iced](https://iced.rs/) which can run natively and on the Web via wgpu.

## Requirements

- Stable Rust
- [Iced Dependencies](https://github.com/iced-rs/iced/blob/master/DEPENDENCIES.md)

## Usage

First, you'll need to rename a few fields:

- In [`Cargo.toml`](./Cargo.toml), replace `iced-app-template` with your crate name
- In [`index.html`](./index.html):
  - In the `<title>` tag, replace `Iced App` with your app name
  - In the `<link data-trunk rel="rust" href="Cargo.toml" ...>` tag, replace `iced-app-template` with your crate name
- In [`src/app.rs`](./src/app.rs), replace the `Iced App` title with your app name

To run the app natively, use `cargo run` as normal:

```bash
cargo run               # Debug Build
cargo run --release     # Release Build
```

To run the app within a browser, use one of trunk's commands:

```bash
# Build the WASM app
trunk build             # Debug Build
trunk build --release   # Release Build

# Build and serve the WASM app
# The served app can be found at http://127.0.0.1:8080
trunk serve             # Debug Build
trunk serve --release   # Release Build
```

## Deploying to GitHub Pages

A workflow for deploying to GitHub pages is setup in [`.github/workflows/static.yml`](./.github/workflows/static.yml). This runs automatically when any of the source files change on the `master` branch or manually via the `Actions` tab on GitHub.

To enable this for the repository, go to `Settings -> Pages -> Build and deployment`, and set the source to `GitHub Actions`.

## Outstanding Issues

- [WASM: Canvas is stuck at a fixed size](https://github.com/iced-rs/iced/issues/1265)
  - A potential workaround is described in the linked issue, but there doesn't seem to be a way to make it reactive (even after adding JavaScript event listeners and a custom canvas target)
  - It may be possible with winit 0.29, due to [this PR](https://github.com/rust-windowing/winit/pull/2859)
- [WASM: No text is rendered](https://github.com/iced-rs/iced/issues/1974)
