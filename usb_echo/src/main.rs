mod usbhelper;

fn main() {
    /* for device in rusb::devices().unwrap().iter() {
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
    } */

    // let vendor_id = 0x0483;
    // let product_id = 0x374f;
    let vendor_id = 0x1234;
    let product_id = 0xabcd;

    if let Some(mut dev) = usbhelper::open(vendor_id, product_id) {
        // let cfg = echo_device.device().config_descriptor(active_config).expect("Failed to get config");
        match dev.write() {
            Err(e) => panic!("{:?}", e),
            Ok(v) => println!("written {}", v)
        }

        match dev.read() {
            Err(e) => panic!("{:?}", e),
            Ok(v) => println!("{:?}", v)
        }
        // let ifc = echo_device.device().interfaces();
    } else {
        println!("USB Device with ID: {:04x}:{:04x} not found", vendor_id, product_id);
    }
}
