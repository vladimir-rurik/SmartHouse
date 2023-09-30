// Smart Socket device
#[allow(dead_code)]
struct SmartSocket {
    description: String,
    is_on: bool,
    current_power: f32, // in Watts (W)
}

impl SmartSocket {
    /// Creates a new instance of SmartSocket with a given description.
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            is_on: false,
            current_power: 0.0,
        }
    }

    /// Provides a textual description of the SmartSocket.
    pub fn _get_description(&self) -> &str {
        &self.description
    }

    /// Turns on the SmartSocket.
    pub fn _turn_on(&mut self) {
        // For prototype purposes, this method does not implement the actual functionality.
        todo!()
    }

    /// Turns off the SmartSocket.
    pub fn _turn_off(&mut self) {
        // For prototype purposes, this method does not implement the actual functionality.
        todo!()
    }

    /// Provides data on the currently consumed power.
    pub fn _get_power_usage(&self) -> f32 {
        // For prototype purposes, this method does not implement the actual functionality.
        todo!()
    }
}

// Smart Thermometer device
#[allow(dead_code)]
struct SmartThermometer {
    current_temperature: f32, // in Celsius (°C)
}

impl SmartThermometer {
    /// Creates a new instance of SmartThermometer.
    pub fn new() -> Self {
        Self {
            current_temperature: 0.0,
        }
    }

    /// Provides data on the current temperature.
    pub fn _get_temperature(&self) -> f32 {
        // For prototype purposes, this method does not implement the actual functionality.
        todo!()
    }
}

// For test purposes, main function is provided
fn main() {
    let _smart_socket = SmartSocket::new("Living Room Smart Socket");
    let _smart_thermometer = SmartThermometer::new();
}
