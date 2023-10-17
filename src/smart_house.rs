mod device_info;
pub mod smart_house {
    use super::device_info::devices::Device;
    use super::device_info::DeviceInfoProvider;

    // Main structure representing the Smart House.
    #[allow(dead_code)]
    pub struct SmartHouse {
        pub name: String,
        pub rooms: Vec<Room>,
    }

    impl SmartHouse {
        // Create a new Smart House.
        pub(crate) fn new(name: &str, rooms: Vec<Room>) -> Self {
            SmartHouse {
                name: name.to_string(),
                rooms,
            }
        }

        pub fn add_room(&mut self, room: Room) {
            self.rooms.push(room);
        }

        pub fn remove_room(&mut self, room_name: &str) {
            self.rooms.retain(|room| room.name != room_name);
        }

        pub fn list_rooms(&self) -> Vec<&str> {
            self.rooms.iter().map(|room| room.name.as_str()).collect()
        }

        pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
            self.rooms.iter_mut().find(|room| room.name == room_name)
        }

        // Get a list of rooms in the house.
        #[allow(dead_code)]
        pub(crate) fn get_rooms(&self) -> Vec<String> {
            self.rooms.iter().map(|r| r.name.clone()).collect()
        }

        // Get a list of devices in the specified room.
        #[allow(dead_code)]
        pub(crate) fn devices(&self, room_name: &str) -> Option<Vec<String>> {
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
                        Device::SmartSocket(device) => {
                            provider.device_info(&room.name, &device.name)
                        }
                        Device::SmartThermometer(device) => {
                            provider.device_info(&room.name, &device.name)
                        }
                    };
                    let device_name = match device {
                        Device::SmartSocket(device) => &device.name,
                        Device::SmartThermometer(device) => &device.name,
                    };
                    report.push_str(&format!(
                        "Room: {}, Device: {}, Info: {}\n",
                        room.name,
                        device_name,
                        device_info.unwrap_or_else(|e| format!("Error: {:?}", e))
                    ));
                }
            }
            report
        }
    }

    // Structure representing a room.
    pub struct Room {
        pub name: String,
        pub devices: Vec<Device>,
    }

    impl Room {
        // Create a new room.
        #[allow(dead_code)]
        pub(crate) fn new(name: &str, devices: Vec<Device>) -> Self {
            Room {
                name: name.to_string(),
                devices,
            }
        }

        // Add a device to the room.
        #[allow(dead_code)]
        pub(crate) fn add_device(&mut self, device: Device) {
            self.devices.push(device);
        }

        // Remove a device from the room.
        #[allow(dead_code)]
        pub(crate) fn remove_device(&mut self, device_name: &str) {
            self.devices.retain(|device| match device {
                Device::SmartSocket(socket) => socket.name.as_str() != device_name,
                Device::SmartThermometer(thermometer) => thermometer.name.as_str() != device_name,
            });
        }

        // Get a list of devices in the room.
        #[allow(dead_code)]
        pub(crate) fn list_devices(&self) -> Vec<&str> {
            self.devices
                .iter()
                .map(|device| match device {
                    Device::SmartSocket(socket) => socket.name.as_str(),
                    Device::SmartThermometer(thermometer) => thermometer.name.as_str(),
                })
                .collect()
        }
    }
}

use device_info::devices::*;
use device_info::*;
use smart_house::*;

fn main() {
    let living_room_socket = SmartSocket {
        name: "LivingRoomSocket".to_string(),
        state: SocketState::On,
    };
    let kitchen_socket = SmartSocket {
        name: "KitchenSocket".to_string(),
        state: SocketState::Off,
    };
    let kitchen_thermometer = SmartThermometer {
        name: "KitchenThermometer".to_string(),
        state: ThermometerState::Temperature(25.0f32),
    };

    let rooms = vec![
        Room {
            name: "Living Room".to_string(),
            devices: vec![Device::SmartSocket(living_room_socket.clone())],
        },
        Room {
            name: "Kitchen".to_string(),
            devices: vec![
                Device::SmartThermometer(kitchen_thermometer.clone()),
                Device::SmartSocket(kitchen_socket.clone()),
            ],
        },
    ];

    let house = SmartHouse::new("Home", rooms);

    let socket_info_provider = OwningDeviceInfoProvider {
        socket: living_room_socket,
    };
    let report_with_socket = house.create_report(&socket_info_provider);

    println!("Report with socket info:\n{}", report_with_socket);

    let multi_device_info_provider = BorrowingDeviceInfoProvider {
        socket: &kitchen_socket,
        thermo: &kitchen_thermometer,
    };
    let report_with_multi_device = house.create_report(&multi_device_info_provider);

    println!(
        "Report with multi-device info:\n{}",
        report_with_multi_device
    );
}

#[cfg(test)]
mod tests {
    use super::device_info::devices::{
        Device, SmartSocket, SmartThermometer, SocketState, ThermometerState,
    };
    use super::device_info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
    use crate::smart_house::*;

    #[test]
    fn test_add_remove_room() {
        let mut house = SmartHouse::new("HouseName", Vec::new()); // Added "HouseName" to the SmartHouse initialization
        let room = Room::new("Living Room", Vec::new()); // Added devices Vec::new() to Room initialization
        house.add_room(room);
        assert_eq!(house.list_rooms(), vec!["Living Room"]);
        house.remove_room("Living Room");
        assert_eq!(house.list_rooms().len(), 0);
    }

    #[test]
    fn test_add_remove_device() {
        let mut room = Room::new("Bedroom", Vec::new()); // Added devices Vec::new() to Room initialization
        let device = Device::SmartSocket(SmartSocket {
            name: "Socket1".to_string(),
            state: SocketState::On,
        });
        room.add_device(device);
        assert_eq!(room.list_devices(), vec!["Socket1"]);
        room.remove_device("Socket1");
        assert_eq!(room.list_devices().len(), 0);
    }

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
            devices: vec![
                Device::SmartSocket(socket.clone()),
                Device::SmartThermometer(thermo.clone()),
            ],
        };
        let house = SmartHouse::new("MyHouse", vec![room]);

        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };
        let report = house.create_report(&provider);
        assert!(report.contains("Socket1"));
        assert!(report.contains("Thermo1"));
        assert!(!report.contains("Error:"));
    }

    #[test]
    fn test_smart_house_report_generation() {
        let socket = SmartSocket {
            name: "SocketInRoom".to_string(),
            state: SocketState::On,
        };
        let room = Room {
            name: "LivingRoom".to_string(),
            devices: vec![Device::SmartSocket(socket.clone())],
        };
        let house = SmartHouse {
            name: "MyHome".to_string(),
            rooms: vec![room],
        };

        let provider = OwningDeviceInfoProvider {
            socket: socket.clone(),
        };
        let report = house.create_report(&provider);
        assert_eq!(
            report,
            "Room: LivingRoom, Device: SocketInRoom, Info: In room LivingRoom, the socket named SocketInRoom is On\n"
        );
        assert!(!report.contains("Error:"));
    }
}
