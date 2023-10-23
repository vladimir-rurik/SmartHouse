use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();

                stream.write_all(trimmed.as_bytes()).unwrap();

                let mut data = [0_u8; 50];
                match stream.read(&mut data) {
                    Ok(_) => {
                        if let Ok(text) = from_utf8(&data) {
                            println!("Response: {}", text);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
