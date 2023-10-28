mod udp_thermometer;
use udp_thermometer::udp_thermometer_listener::UdpThermometerListener;
use udp_thermometer::udp_thermometer_simulator::UdpThermometerSimulator;

mod device_info;
use device_info::devices::{SmartThermometer, ThermometerState};

fn main() {
    let listener = UdpThermometerListener::new("127.0.0.1:7878", "Living Room Thermometer");
    listener.start_listening();

    let simulator = UdpThermometerSimulator::new("127.0.0.1:7878");
    simulator.start_sending();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(8));
    }
}
