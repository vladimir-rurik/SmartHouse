// Main structure representing the Smart House.
struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

// Structure representing a room.
struct Room {
    name: String,
    devices: Vec<Device>,
}

// Enum representing different devices.
enum Device {
    Socket(String),       // Electrical socket
    Thermometer(String),  // Thermometer
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
    fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|r| r.name.clone()).collect()
    }

    // Get a list of devices in the specified room.
    fn devices(&self, room_name: &str) -> Option<Vec<String>> {
        self.rooms
            .iter()
            .find(|r| r.name == room_name)
            .map(|r| r.devices.iter().map(|d| match d {
                Device::Socket(name) => name.clone(),
                Device::Thermometer(name) => name.clone(),
            }).collect())
    }

    // Generate a textual report about the status of all devices in the house.
    fn create_report<P: DeviceInfoProvider>(&self, provider: &P) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            for device in &room.devices {
                let device_info = match device {
                    Device::Socket(name) => provider.device_info(&room.name, name),
                    Device::Thermometer(name) => provider.device_info(&room.name, name),
                };
                report.push_str(&format!(
                    "Room: {}, Device: {}, Info: {}\n",
                    room.name,
                    match device {
                        Device::Socket(name) => name,
                        Device::Thermometer(name) => name,
                    },
                    device_info
                ));
            }
        }
        report
    }
}

// Trait for providing information about the status of devices.
trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device: &str) -> String;
}

// Information provider owning the device data.
struct SocketInfoProvider {
    socket_state: String,
}

// Information provider borrowing device data.
struct MultiDeviceInfoProvider<'a> {
    socket_state: &'a str,
    thermo_state: &'a str,
}

impl DeviceInfoProvider for SocketInfoProvider {
    fn device_info(&self, _: &str, device: &str) -> String {
        format!("State of {}: {}", device, self.socket_state)
    }
}

impl<'a> DeviceInfoProvider for MultiDeviceInfoProvider<'a> {
    fn device_info(&self, _: &str, device: &str) -> String {
        if device == "Socket" {
            format!("State of {}: {}", device, self.socket_state)
        } else {
            format!("State of {}: {}", device, self.thermo_state)
        }
    }
}

fn main() {
    let rooms = vec![
        Room {
            name: "Living Room".to_string(),
            devices: vec![Device::Socket("LivingRoomSocket".to_string())],
        },
        Room {
            name: "Kitchen".to_string(),
            devices: vec![Device::Thermometer("KitchenThermometer".to_string())],
        },
    ];
    let house = SmartHouse::new("Home", rooms);

    let socket_info_provider = SocketInfoProvider {
        socket_state: "ON".to_string(),
    };
    let report_with_socket = house.create_report(&socket_info_provider);

    let multi_device_info_provider = MultiDeviceInfoProvider {
        socket_state: "OFF",
        thermo_state: "22C",
    };
    let report_with_multi_device = house.create_report(&multi_device_info_provider);

    println!("Report with socket info: {}", report_with_socket);
    println!("Report with multi-device info: {}", report_with_multi_device);
}
