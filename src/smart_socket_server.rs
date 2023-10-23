mod device_info;
use device_info::devices::*;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream, socket: &mut SmartSocket) {
    let mut data = [0_u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            let cmd = from_utf8(&data[..size]).unwrap().trim();
            match cmd {
                "status" => {
                    let status = format!("{:?}, Power: {}", socket.state, socket.power_consumption);
                    stream.write_all(status.as_bytes()).unwrap();
                }
                "on" => {
                    socket.state = SocketState::On;
                    socket.power_consumption = 100.0; // just an example value
                    stream.write_all(b"Socket turned on").unwrap();
                }
                "off" => {
                    socket.state = SocketState::Off;
                    socket.power_consumption = 0.0;
                    stream.write_all(b"Socket turned off").unwrap();
                }
                _ => {
                    stream.write_all(b"Unknown command").unwrap();
                }
            }
            true
        }
        Err(_) => {
            println!("An error occurred with the connection.");
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");
    let mut socket = SmartSocket::default();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut socket);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
