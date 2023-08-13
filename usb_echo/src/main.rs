use std::time::Duration;

fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id());
        match device.open() {
            Ok(_) => println!("Device opened!"),
            Err(e) => println!("Can't open with {:?}", e)
        };
    }

    if let Some(echo_device) = rusb::open_device_with_vid_pid(0x1234, 0xabcd) {
        let timeout = Duration::from_secs(1);
        println!("Active configuration: {}", echo_device.active_configuration().unwrap());
        println!("Language: {:?}", echo_device.read_languages(timeout).unwrap());
    } else {
        println!("USB Device with ID: {:04x}:{:04x} not found", 0x1234, 0xabcd);
    }
}
