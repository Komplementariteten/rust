use rusb::{DeviceHandle, devices, Direction, GlobalContext, TransferType};

#[derive(Debug)]
struct UsbEndpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

#[derive(Debug)]
pub struct UsbDevice {
    handle: DeviceHandle<GlobalContext>,
    read_ep: Option<UsbEndpoint>,
    write_ep: Option<UsbEndpoint>,
}

fn find_ep(dev: &DeviceHandle<GlobalContext>, direction: Direction) -> Option<UsbEndpoint> {
    let dev_desc = dev.device();
    let cfg_desc = match dev_desc.active_config_descriptor() {
        Ok(c) => c,
        Err(_) => return None
    };
    for interface in cfg_desc.interfaces() {
        for descriptor in interface.descriptors() {
            for endpoint_descriptor in descriptor.endpoint_descriptors() {
                if endpoint_descriptor.transfer_type() == TransferType::Bulk && endpoint_descriptor.direction() == direction {
                    Some(UsbEndpoint{
                        config: 0,
                        iface: 0,
                        setting: 0,
                        address: 0
                    })
                }
            }
        }
    }
    None
}

pub(crate) fn open(vendor_id: u16, product_id: u16) -> Option<UsbDevice> {
    if let Some(echo_device) = rusb::open_device_with_vid_pid(0x0483, 0x374f) {
        Some(UsbDevice{
            handle: echo_device,
            read_ep: find_ep(&echo_device, Direction::In),
            write_ep: find_ep(&echo_device, Direction::Out)
        })
    }
    None
        /* let timeout = Duration::from_secs(1);
        let active_config = echo_device.active_configuration().expect("No active Configuration");
        let dev = echo_device.device();
        println!("Active configuration: {}", &active_config);
        println!("Language: {:?}", echo_device.read_languages(timeout).unwrap());
        println!("Address: {}", dev.address());
        let cfg = match dev.active_config_descriptor() {
            Ok(c) => c,
            Err(_) => panic!("Failed to get active config")
        };
        println!("Interface found: {}", cfg.num_interfaces());
        for interface in cfg.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_descriptor in interface_desc.endpoint_descriptors() {
                    let type_name = match endpoint_descriptor.transfer_type() {
                        TransferType::Bulk => "BULK",
                        TransferType::Interrupt => "Interrupt",
                        TransferType::Isochronous => "Isochronous",
                        TransferType::Control => "Control"
                    };

                    match endpoint_descriptor.direction() {
                        Direction::In => println!("IN=>Adr:{}, Transfer: {} Max-Packet: {} ", endpoint_descriptor.address(), type_name, endpoint_descriptor.max_packet_size()),
                        Direction::Out => println!("Out=>Adr:{}, Transfer: {}, Max-Packet: {} ", endpoint_descriptor.address(), type_name, endpoint_descriptor.max_packet_size())
                    }
                }
            }
        }*/
}

