use rand::Rng;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub struct UdpThermometerSimulator {
    destination: String,
}

impl UdpThermometerSimulator {
    pub fn new(destination: &str) -> Self {
        Self {
            destination: destination.to_string(),
        }
    }

    pub fn start_sending(&self) {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
        let destination = self.destination.clone();
        thread::spawn(move || loop {
            let temperature: f32 = rand::thread_rng().gen_range(15.0..30.0);
            socket
                .send_to(&temperature.to_be_bytes(), &destination)
                .expect("Failed to send data");
            thread::sleep(Duration::from_secs(5));
        });
    }
}
