use std::default;
use std::time::Duration;
use rusb::{DeviceHandle, Direction, GlobalContext, TransferType};

#[derive(Debug, PartialEq, Clone, Copy)]
struct UsbEndpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
    max_package: u16
}

#[derive(Debug)]
pub struct UsbDevice {
    h: DeviceHandle<GlobalContext>,
    r: Option<UsbEndpoint>,
    w: Option<UsbEndpoint>,
    read_timeout: Duration
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
                    return Some(UsbEndpoint{
                        config: cfg_desc.number(),
                        iface: descriptor.interface_number(),
                        setting: descriptor.setting_number(),
                        address: endpoint_descriptor.address(),
                        max_package: endpoint_descriptor.max_packet_size()
                    });
                }
            }
        }
    }
    None
}

pub(crate) fn print_devices() {

}

pub(crate) fn open(vendor_id: u16, product_id: u16) -> Option<UsbDevice> {
    if let Some(echo_device) = rusb::open_device_with_vid_pid(vendor_id, product_id) {
        return Some(UsbDevice{
            r: find_ep(&echo_device, Direction::In),
            w: find_ep(&echo_device, Direction::Out),
            h: echo_device,
            read_timeout: Duration::from_millis(500)
        });
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

impl UsbDevice {

    pub fn write(&mut self) -> Result<usize, rusb::Error> {
        if self.w == None {
            panic!("Write Endpoint not set");
        }

        let write_ep = self.w.clone().unwrap();
        let has_kernel_driver  = match self.h.kernel_driver_active(write_ep.iface.clone()) {
            Ok(true) => {
                self.h.detach_kernel_driver(write_ep.iface.clone()).ok();
                true
            },
            _ => false
        };

        let foo = "foo";

        let result = match self.h.write_bulk(write_ep.address.clone(), foo.as_bytes(), self.read_timeout.clone()) {
            Ok(len) => Ok(len),
            Err(e) => Err(e)
        };

        if has_kernel_driver {
            self.h.attach_kernel_driver(write_ep.iface.clone()).ok();
        }
        return result
    }
    pub fn read(&mut self) -> Result<Vec<u8>, rusb::Error> {
        if self.r == None {
            panic!("Read Endpoint not set!")
        }
        let read_ep = self.r.clone().unwrap();
        let has_kernel_driver  = match self.h.kernel_driver_active(read_ep.iface.clone()) {
            Ok(true) => {
                self.h.detach_kernel_driver(read_ep.iface.clone()).ok();
                true
            },
            _ => false
        };

        let mut read_buff: [u8; 125] = [0; 125];
        let result = match self.h.read_bulk(read_ep.address.clone(), read_buff.as_mut_slice(), self.read_timeout.clone()) {
            Ok(len) => Ok(read_buff[0..len].to_vec()),
            Err(e) => Err(e)
        };

        if has_kernel_driver {
            self.h.attach_kernel_driver(read_ep.iface.clone()).ok();
        }
        return result
    }
}