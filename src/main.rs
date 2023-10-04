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
