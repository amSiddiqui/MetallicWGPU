# MetallicWGPU

A project to learn and experiment with [WGPU](https://wgpu.rs) on Apple's Metal API. This repository provides a learning environment for exploring WGPU's capabilities while leveraging Metal's backend.

## Features
- Cross-platform rendering with WGPU.
- Runs natively on Metal.
- Optionally supports DX12 (DirectX 12) or other backends.

## Prerequisites
- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **wasm-bindgen-cli**: Install using `cargo install wasm-bindgen-cli`.
- **Tokio Runtime**: Used for the asynchronous event loop.

## Running the Project

### Run Locally
To run the project using Metal:
```bash
cargo run
```

### Run on DX12
If you want to experiment with DX12, update your `Cargo.toml` to include the `dx12` feature under the WGPU dependency:

```toml
[dependencies]
wgpu = { version = "23.0.1", features = ["dx12", "wgsl"] }
```

Then, re-run the project:
```bash
cargo run
```

### Run on Web (WASM)
To build the project for WebAssembly and serve it locally:

1. Build and bundle the WebAssembly using:
    ```bash
    cargo run -p xtask
    ```

2. Open `http://localhost:8000` in your browser to view the project.

## Learn More
Visit the official [WGPU website](https://wgpu.rs) for documentation, tutorials, and examples.

---

## Project Structure
- **`src/`**: Contains the main Rust codebase.
- **`xtask/`**: A custom task runner to build WebAssembly and serve the project.
- **`index.html`**: A simple HTML file to load the WebAssembly bundle.

## License
This project is licensed under MIT. See `LICENSE` for details.