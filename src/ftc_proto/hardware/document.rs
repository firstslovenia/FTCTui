//! Parsing of the XML document

use std::io::Cursor;

use crate::ftc_proto::hardware::{
    FromXMLTag, MakeOwnedXMLTagAttributes, MakeXMLTag,
    device::{DeviceFlavor, HardwareDeviceType},
    lynx::{LynxModule, LynxUSBDevice, ServoHub},
    robot::{ConfigurationDevice, EthernetOverUsbConfiguration, Robot, Webcam},
};

use xml::{
    EmitterConfig, EventWriter,
    attribute::OwnedAttribute,
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
        r#type: None,
    };

    log::debug!("Starting XML parse..");

    let mut num_lynx_modules = 0;
    let mut num_servo_hubs = 0;

    let mut in_lynx_module_i: Option<usize> = None;
    let mut in_servo_hub_i: Option<usize> = None;

    let parser = EventReader::new(cursor);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(event) => {
                match event.clone() {
                    XmlEvent::StartElement {
                        name: tag_name,
                        attributes,
                        namespace: _namespace,
                    } => {
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

                        match tag_name.to_string().as_str() {
                            "Robot" => match Robot::from_xml_tag(event) {
                                Some(r) => robot = r,
                                None => {
                                    log::error!("Failed to parse Robot from XML! Bailing out..");
                                    return None;
                                }
                            },
                            "EthernetOverUsbConfiguration" => {
                                match EthernetOverUsbConfiguration::from_xml_tag(event) {
                                    Some(e) => robot.ethernet_over_usb_device = Some(e),
                                    None => {
                                        log::warn!(
                                            "Failed to parse Ethernet over usb configuration from XML! Ignoring.."
                                        );
                                    }
                                }
                            }
                            "Webcam" => match Webcam::from_xml_tag(event) {
                                Some(e) => robot.webcam = Some(e),
                                None => {
                                    log::warn!(
                                        "Failed to parse Webcam configuration from XML! Ignoring.."
                                    );
                                }
                            },
                            "LynxUsbDevice" => match LynxUSBDevice::from_xml_tag(event) {
                                Some(l) => robot.lynx_usb_device = Some(l),
                                None => {
                                    log::error!(
                                        "Failed to parse LynxUSBDevice from XML! Bailing out.."
                                    );
                                    return None;
                                }
                            },
                            "LynxModule" => match LynxModule::from_xml_tag(event) {
                                Some(l) => {
                                    robot.lynx_usb_device.as_mut().unwrap().lynx_modules.push(l);
                                    num_lynx_modules += 1;
                                    in_lynx_module_i = Some(num_lynx_modules - 1);
                                }
                                None => {
                                    log::error!(
                                        "Failed to parse LynxModule from XML! Bailing out.."
                                    );
                                    return None;
                                }
                            },
                            "ServoHub" => match ServoHub::from_xml_tag(event) {
                                Some(h) => {
                                    robot.lynx_usb_device.as_mut().unwrap().servo_hubs.push(h);
                                    num_servo_hubs += 1;
                                    in_servo_hub_i = Some(num_servo_hubs - 1);
                                }
                                None => {
                                    log::error!("Failed to parse ServoHub from XML! Bailing out..");
                                    return None;
                                }
                            },
                            _ => {
                                if in_servo_hub_i.is_none() && in_lynx_module_i.is_none() {
                                    log::warn!("Unhandled XML tag: {tag_name}");
                                    continue;
                                }

                                if let Some(servo_hub_index) = in_servo_hub_i {
                                    match ConfigurationDevice::from_xml_tag(event, device_types) {
                                        Some(device) => match device.device_type {
                                            DeviceFlavor::Servo => {
                                                robot.lynx_usb_device.as_mut().unwrap().servo_hubs
                                                    [servo_hub_index]
                                                    .servos
                                                    .push(device);
                                            }
                                            _ => {
                                                log::warn!(
                                                    "Non-servo ({}, which is {:?}) in ServoHub. Ignoring..",
                                                    device.xml_tag_name,
                                                    device.device_type
                                                );
                                            }
                                        },
                                        None => {
                                            log::error!(
                                                "Failed to parse tag {} as Configuration Device (in ServoHub)",
                                                tag_name.to_string()
                                            );
                                        }
                                    }
                                } else if let Some(lynx_module_index) = in_lynx_module_i {
                                    match ConfigurationDevice::from_xml_tag(event, device_types) {
                                        Some(device) => {
                                            match device.device_type {
                                                DeviceFlavor::Motor => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .motors
                                                        .push(device);
                                                }
                                                DeviceFlavor::Servo => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .servos
                                                        .push(device);
                                                }
                                                DeviceFlavor::I2C => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .i2c_devices
                                                        .push(device);
                                                }
                                                DeviceFlavor::AnalogSensor => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .analog_inputs
                                                        .push(device);
                                                }
                                                DeviceFlavor::DigitalIO => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .digital_devices
                                                        .push(device);
                                                }
                                                // ?? maybe
                                                DeviceFlavor::AnalogOutput => {
                                                    robot
                                                        .lynx_usb_device
                                                        .as_mut()
                                                        .unwrap()
                                                        .lynx_modules[lynx_module_index]
                                                        .pwm_outputs
                                                        .push(device);
                                                }
                                                _ => {
                                                    log::error!(
                                                        "Unhandled flavor {:?} - (tag {})",
                                                        device.device_type,
                                                        device.xml_tag_name
                                                    );
                                                }
                                            }
                                        }
                                        None => {
                                            log::error!(
                                                "Failed to parse tag {} as Configuration Device (in LynxModule)",
                                                tag_name.to_string()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    XmlEvent::EndElement { name } => {
                        depth -= 1;
                        log::trace!("{:spaces$}-{name}", "", spaces = depth * 2);

                        match name.to_string().as_str() {
                            "LynxModule" => {
                                in_lynx_module_i = None;
                            }
                            "ServoHub" => {
                                in_servo_hub_i = None;
                            }
                            _ => {}
                        }
                    }
                    XmlEvent::StartDocument {
                        version,
                        encoding,
                        standalone,
                    } => {
                        log::trace!(
                            "Start document: v{}, encoding {}, standalone: {:?}",
                            version,
                            encoding,
                            standalone
                        );
                    }
                    XmlEvent::EndDocument => {
                        log::trace!("End document");
                    }
                    XmlEvent::CData(something) => {
                        log::trace!("CData: {}", something);
                    }
                    XmlEvent::Comment(comment) => {
                        log::trace!("Comment: {}", comment);
                    }
                    // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                    _ => {}
                }
            }
            Err(e) => {
                log::error!("Error: {e}");
                return None;
            }
        }
    }

    Some(robot)
}

/// Tries to write the robot configuration to an XML string
///
/// clones required data
pub fn write_xml_document(robot: &Robot) -> Option<String> {
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

        events.push(robot.opening_event());

        let lynx_usb_device = robot.lynx_usb_device.as_ref().unwrap();
        let lynx_usb_device_attributes = lynx_usb_device.make_owned_attributes();

        events.push(writer::XmlEvent::StartElement {
            name: "LynxUsbDevice".into(),
            attributes: std::borrow::Cow::Owned(
                lynx_usb_device_attributes
                    .iter()
                    .map(|x| x.borrow())
                    .collect(),
            ),
            namespace: std::borrow::Cow::Owned(Namespace::empty()),
        });

        write_xml_events(&mut writer, events)?;

        let mut lynx_module_attributes: Vec<Vec<OwnedAttribute>> = Vec::new();

        for module in lynx_usb_device.lynx_modules.iter() {
            lynx_module_attributes.push(module.make_owned_attributes());
        }

        for (i, module) in lynx_usb_device.lynx_modules.iter().enumerate() {
            write_xml_event(
                &mut writer,
                writer::XmlEvent::StartElement {
                    name: "LynxModule".into(),
                    attributes: std::borrow::Cow::Owned(
                        lynx_module_attributes[i]
                            .iter()
                            .map(|x| x.borrow())
                            .collect(),
                    ),
                    namespace: std::borrow::Cow::Owned(Namespace::empty()),
                },
            )?;

            let lynx_devices = module.clone().all_devices();

            for device in lynx_devices {
                let attributes = device.make_owned_attributes();

                write_xml_event(
                    &mut writer,
                    writer::XmlEvent::StartElement {
                        name: device.xml_tag_name.as_str().into(),
                        attributes: std::borrow::Cow::Owned(
                            attributes.iter().map(|x| x.borrow()).collect(),
                        ),
                        namespace: std::borrow::Cow::Owned(Namespace::empty()),
                    },
                )?;

                write_xml_event(
                    &mut writer,
                    writer::XmlEvent::EndElement {
                        name: Some(device.xml_tag_name.as_str().into()),
                    },
                )?;
            }

            write_xml_event(
                &mut writer,
                writer::XmlEvent::EndElement {
                    name: Some("LynxModule".into()),
                },
            )?;
        }

        let mut servo_hub_attributes: Vec<Vec<OwnedAttribute>> = Vec::new();

        for servo_hub in lynx_usb_device.servo_hubs.iter() {
            servo_hub_attributes.push(servo_hub.make_owned_attributes());
        }

        for (i, module) in lynx_usb_device.servo_hubs.iter().enumerate() {
            write_xml_event(
                &mut writer,
                writer::XmlEvent::StartElement {
                    name: "ServoHub".into(),
                    attributes: std::borrow::Cow::Owned(
                        servo_hub_attributes[i].iter().map(|x| x.borrow()).collect(),
                    ),
                    namespace: std::borrow::Cow::Owned(Namespace::empty()),
                },
            )?;

            for servo in &module.servos {
                let attributes = servo.make_owned_attributes();

                write_xml_event(
                    &mut writer,
                    writer::XmlEvent::StartElement {
                        name: servo.xml_tag_name.as_str().into(),
                        attributes: std::borrow::Cow::Owned(
                            attributes.iter().map(|x| x.borrow()).collect(),
                        ),
                        namespace: std::borrow::Cow::Owned(Namespace::empty()),
                    },
                )?;

                write_xml_event(
                    &mut writer,
                    writer::XmlEvent::EndElement {
                        name: Some(servo.xml_tag_name.as_str().into()),
                    },
                )?;
            }

            write_xml_event(
                &mut writer,
                writer::XmlEvent::EndElement {
                    name: Some("ServoHub".into()),
                },
            )?;
        }

        events = Vec::new();

        events.push(writer::XmlEvent::EndElement {
            name: Some("LynxUsbDevice".into()),
        });

        let webcam_attributes;

        if let Some(webcam) = &robot.webcam {
            webcam_attributes = webcam.make_owned_attributes();

            events.push(writer::XmlEvent::StartElement {
                name: "Webcam".into(),
                attributes: std::borrow::Cow::Owned(
                    webcam_attributes.iter().map(|x| x.borrow()).collect(),
                ),
                namespace: std::borrow::Cow::Owned(Namespace::empty()),
            });

            events.push(writer::XmlEvent::EndElement {
                name: Some("Webcam".into()),
            });
        }

        let ethernet_over_usb_attributes;

        if let Some(ethernet_over_usb) = &robot.ethernet_over_usb_device {
            ethernet_over_usb_attributes = ethernet_over_usb.make_owned_attributes();

            events.push(writer::XmlEvent::StartElement {
                name: "EthernetOverUsbConfiguration".into(),
                attributes: std::borrow::Cow::Owned(
                    ethernet_over_usb_attributes
                        .iter()
                        .map(|x| x.borrow())
                        .collect(),
                ),
                namespace: std::borrow::Cow::Owned(Namespace::empty()),
            });

            events.push(writer::XmlEvent::EndElement {
                name: Some("EthernetOverUsbConfiguration".into()),
            });
        }

        events.push(robot.closing_event());

        write_xml_events(&mut writer, events)?;

        return Some(output);
    }
}

/// Helper function to reduce boilerplate in our main XML writer function
fn write_xml_event(
    writer: &mut EventWriter<Cursor<&mut Vec<u8>>>,
    event: xml::writer::XmlEvent,
) -> Option<()> {
    match writer.write(event) {
        Ok(_) => Some(()),
        Err(e) => {
            log::error!("Write error: {e}");
            None
        }
    }
}

/// Helper function to reduce boilerplate in our main XML writer function
fn write_xml_events(
    writer: &mut EventWriter<Cursor<&mut Vec<u8>>>,
    events: Vec<xml::writer::XmlEvent>,
) -> Option<()> {
    for event in events {
        match writer.write(event) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Write error: {e}");
                return None;
            }
        }
    }

    Some(())
}
