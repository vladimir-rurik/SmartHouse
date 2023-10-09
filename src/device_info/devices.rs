#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub name: String,
    pub state: SocketState,
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
pub enum Device {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
}