use rand::Rng;
use std::time::Duration;
use tokio::net::UdpSocket;

pub struct UdpThermometerSimulator {
    destination: String,
}

impl UdpThermometerSimulator {
    pub fn new(destination: &str) -> Self {
        Self {
            destination: destination.to_string(),
        }
    }

    pub async fn start_sending(&self) {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("Couldn't bind to address");
        let destination = self.destination.clone();

        tokio::spawn(async move {
            loop {
                let temperature: f32 = rand::thread_rng().gen_range(15.0..30.0);
                socket
                    .send_to(&temperature.to_be_bytes(), &destination)
                    .await
                    .expect("Failed to send data");
                println!("Sent temperature: {}", temperature);
                tokio::time::sleep(Duration::from_secs(4)).await;
            }
        });
    }
}
