#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub name: String,
    pub state: SocketState,
    pub power_consumption: f32,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self {
            name: "TestSocket".to_string(),
            state: SocketState::Off,
            power_consumption: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SmartThermometer {
    pub name: String,
    pub state: ThermometerState,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SocketState {
    On,
    Off,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ThermometerState {
    Off,
    Temperature(f32),
}

// Enum representing different devices.
#[allow(dead_code)]
pub enum Device {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
}
