mod device_info;

pub use device_info::devices::*;
pub use device_info::*;

pub mod prelude {
    pub use crate::device_info::devices::*;
    pub use crate::device_info::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_socket_creation() {
        let socket = SmartSocket {
            name: "TestSocket".to_string(),
            state: SocketState::On,
        };
        assert_eq!(socket.name, "TestSocket");
        assert_eq!(socket.state, SocketState::On);
    }

    #[test]
    fn test_smart_thermometer_creation() {
        let thermo = SmartThermometer {
            name: "TestThermo".to_string(),
            state: ThermometerState::Temperature(23.0),
        };
        assert_eq!(thermo.name, "TestThermo");
        match thermo.state {
            ThermometerState::Temperature(t) => assert_eq!(t, 23.0),
            _ => panic!("Unexpected state for the thermometer"),
        }
    }

    #[test]
    fn test_owning_device_info_provider() {
        let socket = SmartSocket {
            name: "SocketInRoom".to_string(),
            state: SocketState::On,
        };
        let provider = OwningDeviceInfoProvider { socket };
        let info = provider.device_info("LivingRoom", "SocketInRoom").unwrap();
        assert_eq!(
            info,
            "In room LivingRoom, the socket named SocketInRoom is On"
        );
    }

    #[test]
    fn test_borrowing_device_info_provider() {
        let socket = SmartSocket {
            name: "SocketInKitchen".to_string(),
            state: SocketState::Off,
        };
        let thermo = SmartThermometer {
            name: "ThermoInKitchen".to_string(),
            state: ThermometerState::Temperature(20.0),
        };
        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };
        let socket_info = provider.device_info("Kitchen", "SocketInKitchen").unwrap();
        assert_eq!(
            socket_info,
            "Room: Kitchen, Device: SmartSocket named SocketInKitchen, State: Off"
        );
        let thermo_info = provider.device_info("Kitchen", "ThermoInKitchen").unwrap();
        assert_eq!(
            thermo_info,
            "Room: Kitchen, Device: SmartThermometer named ThermoInKitchen, State: Temperature(20.0)"
        );
    }
}
