use std::process::Command;

use tokio::net::TcpListener;
use tokio::task;
use tokio::fs;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    println!("Building WebAssembly...");
    build_wasm();

    println!("Starting server at http://localhost:{PORT}...");
    serve().await.unwrap();
}


fn build_wasm() {
    let status = Command::new("cargo")
    .args(&["build", "--target", "wasm32-unknown-unknown", "--release"])
    .status()
    .expect("Failed to build WebAssembly");

    if !status.success() {
        panic!("Failed to compile WebAssembly");
    }

    let status = Command::new("wasm-bindgen")
        .args(&[
            "--out-dir",
            "./dist",
            "--target",
            "web",
            "./target/wasm32-unknown-unknown/release/learn-metal.wasm",
        ])
        .status()
        .expect("Failed to build wasm bindings");

    if !status.success() {
        panic!("Failed to generate WebAssembly bindings");
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).await?;
    println!("Server running at http://127.0.0.1:{PORT}");

    while let Ok((mut stream, _)) = listener.accept().await {
        task::spawn(async move {
            println!("Received connection");
            let html_content = fs::read_to_string("./index.html")
            .await.expect("Failed to read index.html");


            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer).await;
            let response = if buffer.starts_with(b"GET / ") {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                    html_content.len(),
                    html_content
                )
            } else {
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n".to_string()
            };

            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to write response: {}", e);
            }
            
        });
    }
    println!("Connection closed");
    Ok(())
}
