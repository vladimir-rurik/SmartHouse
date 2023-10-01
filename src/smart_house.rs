// Main structure representing the Smart House.
#[allow(dead_code)]
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
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
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
                    Device::Socket(name) => name.clone(),
                    Device::Thermometer(name) => name.clone(),
                })
                .collect()
        })
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
struct SmartSocket {}
struct SmartThermometer {}

// Information provider owning the device data.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

// Information provider borrowing device data.
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}


impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, _: &str, device: &Device) -> String {
        match device {
            Device::Socket(_) => format!("State of Socket: {}", self.socket_state),
            Device::Thermometer(_) => format!("Temperature of Thermometer: {}", self.thermo_state),
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, _: &str, device: &str) -> String {
        if device == "Socket1" {
            // Fetch the state from the borrowed SmartSocket.
            format!("State of {}: OFF", device)
        } else if device == "Thermometer1" {
            // Fetch the temperature from the borrowed SmartThermometer.
            format!("State of {}: 22C", device)
        } else {
            "Unknown device".to_string()
        }
    }
}


fn main() {

    let LivingRoomSocket = SmartSocket {};
    let KitchenSocket = SmartSocket {};
    let KitchenThermometer = SmartThermometer {};
    
    let rooms = vec![
        Room {
            name: "Living Room".to_string(),
            devices: vec![Device::Socket(LivingRoomSocket)],
        },
        Room {
            name: "Kitchen".to_string(),
            devices: vec![Device::Thermometer(KitchenThermometer), Device::Socket(KitchenSocket)],
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

    println!("Report with socket info:\n{}", report_with_socket);
    println!(
        "Report with multi-device info:\n{}",
        report_with_multi_device
    );
}
