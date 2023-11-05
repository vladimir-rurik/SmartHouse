mod device_info;
use device_info::devices::{SmartSocket, SocketState};
use std::str::from_utf8;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

async fn handle_client(mut stream: TcpStream, socket: Arc<Mutex<SmartSocket>>) {
    let mut data = vec![0_u8; 50]; // Use a Vec<u8> to allow for resizing if necessary
    loop {
        let size = match stream.read(&mut data).await {
            Ok(0) => return, // Connection closed
            Ok(size) => size,
            Err(_) => {
                eprintln!("An error occurred with the connection.");
                return;
            }
        };

        let cmd = from_utf8(&data[..size]).unwrap().trim();
        let mut socket = socket.lock().await; // Acquire lock before accessing socket

        match cmd {
            "status" => {
                let status = format!("{:?}, Power: {}", socket.state, socket.power_consumption);
                stream.write_all(status.as_bytes()).await.unwrap();
            }
            "on" => {
                socket.state = SocketState::On;
                socket.power_consumption = 100.0; // just an example value
                stream.write_all(b"Socket turned on").await.unwrap();
            }
            "off" => {
                socket.state = SocketState::Off;
                socket.power_consumption = 0.0;
                stream.write_all(b"Socket turned off").await.unwrap();
            }
            _ => {
                stream.write_all(b"Unknown command").await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    let socket = Arc::new(Mutex::new(SmartSocket::default()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let socket = Arc::clone(&socket);

        tokio::spawn(async move {
            handle_client(stream, socket).await;
        });
    }
}
