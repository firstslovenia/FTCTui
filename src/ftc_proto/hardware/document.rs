//! Parsing of the XML document

use std::{io::Cursor, os::linux::fs::MetadataExt};

use crate::ftc_proto::hardware::{
    device::{DeviceFlavor, HardwareDeviceType},
    robot::{ConfigurationController, ConfigurationDevice, LynxModule, LynxUSBDevice, Robot},
};

use xml::{
    EmitterConfig, Encoding,
    attribute::Attribute,
    common::XmlVersion,
    namespace::Namespace,
    reader::{EventReader, XmlEvent},
    writer,
};

/// Tries to parse an xml robot configuration
pub fn try_parse_xml_document(
    xml: String,
    device_types: &Vec<HardwareDeviceType>,
) -> Option<Robot> {
    let cursor = Cursor::new(xml);

    let mut robot = Robot {
        webcam: None,
        lynx_usb_device: None,
        ethernet_over_usb_device: None,
    };

    log::debug!("Starting XML parse..");

    let parser = EventReader::new(cursor);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            }) => {
                log::trace!("{:spaces$}+{}", "", tag_name, spaces = depth * 2);

                for attr in &attributes {
                    log::trace!(
                        "{:spaces$}{} - {}",
                        "",
                        attr.name,
                        attr.value,
                        spaces = depth * 2,
                    );
                }

                depth += 1;

                let mut parent_module_address = u32::MAX;
                let mut port = None;
                let mut bus = u32::MAX;
                let mut name = tag_name.to_string();
                let mut serial_number = None;

                for attribute in attributes {
                    match attribute.name.to_string().as_str() {
                        "parentModuleAddress" => {
                            parent_module_address = attribute.value.parse().unwrap()
                        }
                        "port" => port = Some(attribute.value.parse().unwrap()),
                        "bus" => bus = attribute.value.parse().unwrap(),
                        "name" => {
                            name = attribute.value;
                        }
                        "serialNumber" => {
                            serial_number = Some(attribute.value);
                        }
                        _ => {}
                    }
                }

                match tag_name.to_string().as_str() {
                    "Robot" => {}
                    "LynxUsbDevice" => {
                        robot.lynx_usb_device = Some(LynxUSBDevice {
                            servo_hub: None,
                            lynx_module: None,
                            parent_module_address,
                            controller_meta: ConfigurationController {
                                serial_number,
                                device_meta: ConfigurationDevice {
                                    name,
                                    device_type: DeviceFlavor::BuiltIn,
                                    port,
                                },
                            },
                        });
                    }
                    "LynxModule" => {
                        robot.lynx_usb_device.as_mut().unwrap().lynx_module = Some(LynxModule {
                            controller_meta: ConfigurationController {
                                serial_number,
                                device_meta: ConfigurationDevice {
                                    name,
                                    device_type: DeviceFlavor::BuiltIn,
                                    port,
                                },
                            },
                            servos: Vec::new(),
                            motors: Vec::new(),
                            i2c_devices: Vec::new(),
                            pwm_outputs: Vec::new(),
                            analog_inputs: Vec::new(),
                            digital_devices: Vec::new(),
                        });
                    }
                    _ => {
                        let mut handled = false;

                        for device_type in device_types {
                            if tag_name.to_string() == device_type.xml_tag {
                                let device = ConfigurationDevice {
                                    device_type: device_type.flavor,
                                    name: name.clone(),
                                    port,
                                };

                                match device_type.flavor {
                                    DeviceFlavor::Motor => {
                                        robot
                                            .lynx_usb_device
                                            .as_mut()
                                            .unwrap()
                                            .lynx_module
                                            .as_mut()
                                            .unwrap()
                                            .motors
                                            .push(device);
                                    }
                                    DeviceFlavor::Servo => {
                                        robot
                                            .lynx_usb_device
                                            .as_mut()
                                            .unwrap()
                                            .lynx_module
                                            .as_mut()
                                            .unwrap()
                                            .servos
                                            .push(device);
                                    }
                                    DeviceFlavor::I2C => {
                                        robot
                                            .lynx_usb_device
                                            .as_mut()
                                            .unwrap()
                                            .lynx_module
                                            .as_mut()
                                            .unwrap()
                                            .i2c_devices
                                            .push(device);
                                    }
                                    DeviceFlavor::AnalogSensor => {
                                        robot
                                            .lynx_usb_device
                                            .as_mut()
                                            .unwrap()
                                            .lynx_module
                                            .as_mut()
                                            .unwrap()
                                            .analog_inputs
                                            .push(device);
                                    }
                                    // ?? maybe
                                    DeviceFlavor::AnalogOutput => {
                                        robot
                                            .lynx_usb_device
                                            .as_mut()
                                            .unwrap()
                                            .lynx_module
                                            .as_mut()
                                            .unwrap()
                                            .pwm_outputs
                                            .push(device);
                                    }
                                    _ => {
                                        println!(
                                            "Unhandled flavor {:?} - (name {})",
                                            device_type.flavor, device_type.xml_tag
                                        );
                                    }
                                }

                                handled = true;
                                break;
                            }
                        }

                        if !handled {
                            log::warn!("Unhandled tag {}", tag_name);
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                log::trace!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Ok(XmlEvent::StartDocument {
                version,
                encoding,
                standalone,
            }) => {
                log::trace!(
                    "Start document: v{}, encoding {}, standalone: {:?}",
                    version,
                    encoding,
                    standalone
                );
            }
            Ok(XmlEvent::EndDocument) => {
                log::trace!("End document");
            }
            Ok(XmlEvent::CData(something)) => {
                log::trace!("CData: {}", something);
            }
            Ok(XmlEvent::Comment(comment)) => {
                log::trace!("Comment: {}", comment);
            }
            Err(e) => {
                log::error!("Error: {e}");
                return None;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Some(robot)
}

/// Writes the robot configuration to an XML string
///
/// clones required data
pub fn write_xml_document(robot: &Robot) -> String {
    unsafe {
        let mut output = String::new();

        log::debug!("Starting XML write..");

        // Can this be safe? idk
        let cursor = std::io::Cursor::new(output.as_mut_vec());

        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(cursor);

        let mut events = Vec::new();

        events.push(writer::XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: Some(true),
        });

        events.push(writer::XmlEvent::StartElement {
            name: "Robot".into(),
            attributes: std::borrow::Cow::Owned(vec![Attribute {
                name: "type".into(),
                value: "FirstInspires-FTC",
            }]),
            namespace: std::borrow::Cow::Owned(Namespace::empty()),
        });

        let lynx_usb_device = robot.lynx_usb_device.as_ref().unwrap();

        let lynx_parent_module_address = lynx_usb_device.parent_module_address.to_string();

        let lynx_module = lynx_usb_device.lynx_module.as_ref().unwrap();

        let control_hub_port = lynx_module
            .controller_meta
            .device_meta
            .port
            .map(|x| x.to_string())
            .unwrap_or(lynx_parent_module_address.clone());

        events.push(writer::XmlEvent::StartElement {
            name: "LynxUsbDevice".into(),
            attributes: std::borrow::Cow::Owned(vec![
                Attribute {
                    name: "name".into(),
                    value: "Control Hub Portal",
                },
                Attribute {
                    name: "serialNumber".into(),
                    value: "(embedded)",
                },
                Attribute {
                    name: "parentModuleAddress".into(),
                    value: lynx_parent_module_address.as_str(),
                },
            ]),
            namespace: std::borrow::Cow::Owned(Namespace::empty()),
        });

        events.push(writer::XmlEvent::StartElement {
            name: "LynxModule".into(),
            attributes: std::borrow::Cow::Owned(vec![
                Attribute {
                    name: "name".into(),
                    value: "Control Hub",
                },
                Attribute {
                    name: "port".into(),
                    value: control_hub_port.as_str(),
                },
            ]),
            namespace: std::borrow::Cow::Owned(Namespace::empty()),
        });

        for event in events {
            if let Err(e) = writer.write(event) {
                log::error!("Write error: {e}")
            }
        }

        let mut lynx_devices = Vec::new();

        lynx_devices.append(&mut lynx_module.motors.clone());
        lynx_devices.append(&mut lynx_module.servos.clone());
        lynx_devices.append(&mut lynx_module.i2c_devices.clone());
        lynx_devices.append(&mut lynx_module.digital_devices.clone());
        lynx_devices.append(&mut lynx_module.pwm_outputs.clone());
        lynx_devices.append(&mut lynx_module.analog_inputs.clone());

        for device in &lynx_devices {
            let mut attributes = vec![Attribute {
                name: "name".into(),
                value: device.name.as_str(),
            }];

            let port_string;

            if let Some(port) = device.port {
                port_string = port.to_string();

                attributes.push(Attribute {
                    name: "port".into(),
                    value: port_string.as_str(),
                });
            }

            if let Err(e) = writer.write(writer::XmlEvent::StartElement {
                name: device.name.as_str().into(),
                attributes: attributes.into(),
                namespace: std::borrow::Cow::Owned(Namespace::empty()),
            }) {
                log::error!("Write error: {e}")
            }

            if let Err(e) = writer.write(writer::XmlEvent::EndElement {
                name: Some(device.name.as_str().into()),
            }) {
                log::error!("Write error: {e}")
            }
        }

        events = Vec::new();

        events.push(writer::XmlEvent::EndElement {
            name: Some("LynxModule".into()),
        });
        events.push(writer::XmlEvent::EndElement {
            name: Some("LynxUsbDevice".into()),
        });
        events.push(writer::XmlEvent::EndElement {
            name: Some("Robot".into()),
        });

        for event in events {
            if let Err(e) = writer.write(event) {
                log::error!("Write error: {e}")
            }
        }

        return output;
    }
}
