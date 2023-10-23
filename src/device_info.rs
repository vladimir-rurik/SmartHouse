pub mod devices;
use devices::{SmartSocket, SmartThermometer, SocketState};
// Trait for providing information about the status of devices.
pub trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device_name: &str) -> Result<String, DeviceInfoError>;
}

// Information provider owning the device data.
pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

// Define an error type for device information.
#[derive(Debug, PartialEq, Clone)]
#[warn(dead_code)]
pub enum DeviceInfoError {
    NotFound,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room_name: &str, device_name: &str) -> Result<String, DeviceInfoError> {
        if self.socket.name == device_name {
            match self.socket.state {
                SocketState::On => Ok(format!(
                    "In room {}, the socket named {} is On",
                    room_name, device_name
                )),
                SocketState::Off => Ok(format!(
                    "In room {}, the socket named {} is Off",
                    room_name, device_name
                )),
            }
        } else {
            // We don't have information about the given device name
            Err(DeviceInfoError::NotFound)
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room_name: &str, device_name: &str) -> Result<String, DeviceInfoError> {
        if device_name == self.socket.name {
            Ok(format!(
                "Room: {}, Device: SmartSocket named {}, State: {:?}",
                room_name, device_name, self.socket.state
            ))
        } else if device_name == self.thermo.name {
            Ok(format!(
                "Room: {}, Device: SmartThermometer named {}, State: {:?}",
                room_name, device_name, self.thermo.state
            ))
        } else {
            Err(DeviceInfoError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use devices::{SmartSocket, SmartThermometer, SocketState, ThermometerState};

    #[test]
    fn test_owning_device_info_provider_socket() {
        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
            power_consumption: 100.0,
        };
        let provider = OwningDeviceInfoProvider { socket };
        let info = provider.device_info("LivingRoom", "Socket1").unwrap();
        assert_eq!(info, "In room LivingRoom, the socket named Socket1 is On");
    }

    #[test]
    fn test_owning_device_info_provider_no_info() {
        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
            power_consumption: 100.0,
        };
        let provider = OwningDeviceInfoProvider { socket };
        let info = provider.device_info("LivingRoom", "Socket2");
        assert_eq!(info, Err(DeviceInfoError::NotFound));
    }

    #[test]
    fn test_borrowing_device_info_provider_socket() {
        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
            power_consumption: 100.0,
        };
        let thermo = SmartThermometer {
            name: "Thermo1".to_string(),
            state: ThermometerState::Temperature(22.0),
        };
        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };
        let info = provider.device_info("LivingRoom", "Socket1").unwrap();
        assert_eq!(
            info,
            "Room: LivingRoom, Device: SmartSocket named Socket1, State: On"
        );
    }

    #[test]
    fn test_borrowing_device_info_provider_thermo() {
        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
            power_consumption: 100.0,
        };
        let thermo = SmartThermometer {
            name: "Thermo1".to_string(),
            state: ThermometerState::Temperature(22.0),
        };
        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };
        let info = provider.device_info("LivingRoom", "Thermo1").unwrap();
        assert_eq!(
            info,
            "Room: LivingRoom, Device: SmartThermometer named Thermo1, State: Temperature(22.0)"
        );
    }
}
