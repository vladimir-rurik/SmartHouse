use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut buffer = String::new();

    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            loop {
                buffer.clear(); // Clear buffer
                std::io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read from stdin"); // Sync read

                let trimmed = buffer.trim();
                if trimmed.is_empty() {
                    continue; // Skip empty inputs
                }

                // Send to server asynchronously
                stream
                    .write_all(trimmed.as_bytes())
                    .await
                    .expect("Failed to write to stream");

                let mut data = vec![0; 50]; // Use Vec<u8> to be able to extend it later
                let mut buffer = tokio::io::ReadBuf::new(&mut data);
                stream
                    .read_buf(&mut buffer)
                    .await
                    .expect("Failed to read from stream");

                let response = buffer.filled(); // Get the filled part of the buffer
                if let Ok(text) = std::str::from_utf8(response) {
                    println!("Response: {}", text);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
