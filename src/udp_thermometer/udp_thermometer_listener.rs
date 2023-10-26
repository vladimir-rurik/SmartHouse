use crate::{SmartThermometer, ThermometerState};

use std::convert::TryInto;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    pub fn start_listening(&self) {
        let socket = UdpSocket::bind(&self.address).expect("Couldn't bind to address");

        // Clone the Arc<Mutex<SmartThermometer>> handle
        let thermometer_handle = self.thermometer.clone();

        thread::spawn(move || loop {
            let mut buf = [0u8; 8];
            if let Ok((amt, _src)) = socket.recv_from(&mut buf) {
                let temperature: f32 = f32::from_be_bytes(buf[0..amt].try_into().unwrap());
                thermometer_handle.lock().unwrap().state =
                    ThermometerState::Temperature(temperature);
            }
            thread::sleep(Duration::from_secs(2));
        });
    }

    #[allow(dead_code)]
    pub fn get_temperature(&self) -> ThermometerState {
        self.thermometer.lock().unwrap().state.clone()
    }
}
