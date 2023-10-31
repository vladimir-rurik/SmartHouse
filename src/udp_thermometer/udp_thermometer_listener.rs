use crate::{SmartThermometer, ThermometerState};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

pub struct UdpThermometerListener {
    address: String,
    thermometer: Arc<Mutex<SmartThermometer>>,
}

impl UdpThermometerListener {
    pub fn new(address: &str, name: &str) -> Self {
        Self {
            address: address.to_string(),
            thermometer: Arc::new(Mutex::new(SmartThermometer {
                name: name.to_string(),
                state: ThermometerState::Off,
            })),
        }
    }

    pub async fn start_listening(&self) {
        let socket = UdpSocket::bind(&self.address)
            .await
            .expect("Couldn't bind to address");
        let thermometer_handle = self.thermometer.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 8];
            loop {
                if let Ok((amt, _src)) = socket.recv_from(&mut buf).await {
                    let temperature: f32 = f32::from_be_bytes(buf[0..amt].try_into().unwrap());
                    thermometer_handle.lock().await.state =
                        ThermometerState::Temperature(temperature);
                    println!("Received temperature: {}", temperature);
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });
    }

    #[allow(dead_code)]
    pub async fn get_temperature(&self) -> ThermometerState {
        self.thermometer.lock().await.state.clone()
    }
}
