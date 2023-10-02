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
#[derive(Clone)]
struct SmartSocket {
    name: String,
    state: SocketState,
}
#[derive(Clone)]
#[allow(dead_code)]
struct SmartThermometer {
    name: String,
    state: ThermometerState,
}

#[derive(Clone, Debug)]
enum SocketState {
    On,
    Off,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum ThermometerState {
    Off,
    Temperature(f32),
}

// Enum representing different devices.
enum Device {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
}

// Trait for providing information about the status of devices.
trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device_name: &str) -> String;
}

// Information provider owning the device data.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room_name: &str, device_name: &str) -> String {
        // Check if the device name matches the socket owned by the provider
        if self.socket.name == device_name {
            match self.socket.state {
                SocketState::On => format!(
                    "In room {}, the socket named {} is On",
                    room_name, device_name
                ),
                SocketState::Off => format!(
                    "In room {}, the socket named {} is Off",
                    room_name, device_name
                ),
            }
        } else {
            // We don't have information about the given device name
            format!(
                "No information available for device named {} in room {}",
                device_name, room_name
            )
        }
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room_name: &str, device_name: &str) -> String {
        // Check if the requested device name matches the name of the socket or thermometer
        if device_name == self.socket.name {
            format!(
                "Room: {}, Device: SmartSocket named {}, State: {:?}",
                room_name, device_name, self.socket.state
            )
        } else if device_name == self.thermo.name {
            format!(
                "Room: {}, Device: SmartThermometer named {}, State: {:?}",
                room_name, device_name, self.thermo.state
            )
        } else {
            format!(
                "Device named '{}' not found in room '{}'",
                device_name, room_name
            )
        }
    }
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
    fn create_report<P: DeviceInfoProvider>(&self, provider: &P) -> String {
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
