use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

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

    while let Ok((mut stream, addr)) = listener.accept().await {
        task::spawn(async move {
            println!("Received connection");
            if let Err(e) = handle_connection(&mut stream).await {
                eprintln!("Error handling connection from: {addr} Reason: {e}");
            }
        });
    }
    println!("Connection closed");
    Ok(())
}

async fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0; 4096];
    let bytes_read = stream.read(&mut buffer).await?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_ascii_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    if method != "GET" {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
        return Ok(());
    }

    let sanitized_path = sanitize_path(path)?;
    let (file_path, content_type) = if sanitized_path.as_os_str().is_empty() {
        (PathBuf::from("index.html"), "text/html")
    } else {
        let dist_path = sanitized_path;

        if !dist_path.starts_with("dist") {
            return response_not_found(stream).await;
        }

        let content_type = match dist_path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("wasm") => "application/wasm",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("json") => "application/json",
            _ => "application/octet-stream",
        };

        (dist_path, content_type)
    };

    match fs::read(&file_path).await {
        Ok(content) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                content.len(),
                content_type
            );
            stream.write_all(response.as_bytes()).await?;
            stream.write_all(&content).await?;
        }
        Err(_) => {
            return response_not_found(stream).await;
        }
    }

    Ok(())
}

fn sanitize_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = Path::new(path);

    let mut sanitized = PathBuf::new();

    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => sanitized.push(part),
            std::path::Component::RootDir => continue,
            _ => return Err("Invalid path".into()),
        }
    }

    Ok(sanitized)
}

async fn response_not_found(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
