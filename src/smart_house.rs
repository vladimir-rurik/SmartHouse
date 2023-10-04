use crate::Device;
// use crate::DeviceInfoProvider;

// Main structure representing the Smart House.
#[allow(dead_code)]
pub struct SmartHouse {
    pub name: String,
    pub rooms: Vec<Room>,
}
// Structure representing a room.
pub struct Room {
    pub name: String,
    pub devices: Vec<Device>,
}

impl SmartHouse {
    // Create a new Smart House.
    fn new(name: &str, rooms: Vec<Room>) -> Self {
        SmartHouse {
            name: name.to_string(),
            rooms,
        }
    }

    // Get a list of rooms in the house.
    #[allow(dead_code)]
    fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|r| r.name.clone()).collect()
    }

    // Get a list of devices in the specified room.
    #[allow(dead_code)]
    fn devices(&self, room_name: &str) -> Option<Vec<String>> {
        self.rooms.iter().find(|r| r.name == room_name).map(|r| {
            r.devices
                .iter()
                .map(|d| match d {
                    Device::SmartSocket(socket) => socket.name.clone(),
                    Device::SmartThermometer(thermometer) => thermometer.name.clone(),
                })
                .collect()
        })
    }

    // Generate a textual report about the status of all devices in the house.
    pub fn create_report<P: DeviceInfoProvider>(&self, provider: &P) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            for device in &room.devices {
                let device_info = match device {
                    Device::SmartSocket(device) => provider.device_info(&room.name, &device.name),
                    Device::SmartThermometer(device) => {
                        provider.device_info(&room.name, &device.name)
                    }
                };
                report.push_str(&format!(
                    "Room: {}, Device: {}, Info: {}\n",
                    room.name,
                    match device {
                        Device::SmartSocket(device) => &device.name,
                        Device::SmartThermometer(device) => &device.name,
                    },
                    device_info
                ));
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BorrowingDeviceInfoProvider;

    #[test]
    fn test_smart_house_creation() {
        let house = SmartHouse::new("MyHouse", Vec::new());
        assert_eq!(house.name, "MyHouse");
        assert_eq!(house.rooms.len(), 0);
    }

    #[test]
    fn test_get_rooms() {
        let room1 = Room {
            name: "LivingRoom".to_string(),
            devices: Vec::new(),
        };
        let room2 = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };
        let house = SmartHouse::new("MyHouse", vec![room1, room2]);
        let rooms = house.get_rooms();
        assert_eq!(rooms, vec!["LivingRoom", "Kitchen"]);
    }

    #[test]
    fn test_devices_in_room() {
        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
        };
        let room = Room {
            name: "LivingRoom".to_string(),
            devices: vec![Device::SmartSocket(socket)],
        };
        let house = SmartHouse::new("MyHouse", vec![room]);
        let devices = house.devices("LivingRoom");
        assert_eq!(devices, Some(vec!["Socket1".to_string()]));
    }

    #[test]
    fn test_smart_house_report() {

        let socket = SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
        };
        let thermo = SmartThermometer {
            name: "Thermo1".to_string(),
            state: ThermometerState::Temperature(22.0),
        };
        let room = Room {
            name: "LivingRoom".to_string(),
            devices: vec![Device::SmartSocket(socket.clone()), Device::SmartThermometer(thermo)],
        };
        let house = SmartHouse::new("MyHouse", vec![room]);

        let provider = BorrowingDeviceInfoProvider { socket: &socket, thermo: &thermo };
        let report = house.create_report(&provider);
        assert!(report.contains("Socket1"));
        assert!(report.contains("Thermo1"));
    }
}
