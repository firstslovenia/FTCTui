//! Contains the root XML tags for the robot hardware configuration

use std::{net::IpAddr, str::FromStr};

use serde::{Deserialize, Serialize};
use xml::{
    attribute::{Attribute, OwnedAttribute},
    name::OwnedName,
    namespace::Namespace,
};

use crate::ftc_proto::hardware::{
    FromXMLTag, MakeOwnedXMLTagAttributes, MakeXMLTag, MakeXMLTagAttributes,
    device::{DeviceFlavor, HardwareDeviceType},
    lynx::LynxUSBDevice,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Possible XML tags used in the XML document.
///
/// Note that some devices (vendor specific) are also sent over by the server.
pub enum ConfigurationXMLTag {
    Accelerometer,
    AdafruitColorSensor,
    ColorSensor,
    Compass,
    #[serde(rename = "EthernetDevice")]
    EthernetOverUSBDevice,
    Gyroscope,
    #[serde(rename = "IrSeeker")]
    IRSeeker,
    #[serde(rename = "IrSeekerV3")]
    IRSeekerV3,
    LightSensor,
    LynxColorSensor,
    LynxModule,
    #[serde(rename = "LynxUsbDevice")]
    LynxUSBDevice,
    Nothing,
    PulseWidthDevice,
    Robot,
    ServoHub,
    UltrasonicSensor,
    #[serde(rename = "<unknown>")]
    Unknown,
    Webcam,

    /// Added by us - captures other tags, specifically ones provided in the hardware device types
    #[serde(untagged)]
    Other(String),
}

/// The XML robot tag, the root tag of our configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robot {
    /// Optional, sometimes set to "FirstInspires-FTC"
    pub r#type: Option<String>,
    /// Parent of our Control and Expansion hub objects
    pub lynx_usb_device: Option<LynxUSBDevice>,
    pub webcam: Option<Webcam>,
    pub ethernet_over_usb_device: Option<EthernetOverUsbConfiguration>,
}

impl MakeXMLTagAttributes for Robot {
    fn make_attributes(&self) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        if let Some(r#type) = &self.r#type {
            attributes.push(Attribute {
                name: "type".into(),
                value: &r#type,
            });
        }

        return attributes;
    }
}

impl MakeXMLTag for Robot {
    fn opening_event(&self) -> xml::writer::XmlEvent {
        let attributes = self.make_attributes();

        xml::writer::XmlEvent::StartElement {
            name: "Robot".into(),
            attributes: std::borrow::Cow::Owned(attributes),
            namespace: std::borrow::Cow::Owned(Namespace::empty()),
        }
    }

    fn closing_event(&self) -> xml::writer::XmlEvent {
        xml::writer::XmlEvent::EndElement {
            name: Some("Robot".into()),
        }
    }
}

impl FromXMLTag for Robot {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self>
    where
        Self: Sized,
    {
        match event {
            xml::reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace: _namespace,
            } => {
                if name.to_string().as_str() != "Robot" {
                    log::error!("Name of element is {}, expected Robot", name.to_string());
                    return None;
                }

                let mut r#type = None;

                for attr in attributes {
                    if attr.name.to_string().as_str() == "type" {
                        r#type = Some(attr.value);
                    }
                }

                // The other fields need to be added later
                Some(Self {
                    r#type,
                    lynx_usb_device: None,
                    webcam: None,
                    ethernet_over_usb_device: None,
                })
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

/// A configuration for a webcam, a child of [Robot]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Webcam {
    pub controller_meta: ConfigurationController,

    /// False by default
    pub auto_open: bool,
}

impl MakeOwnedXMLTagAttributes for Webcam {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        let mut attributes = self.controller_meta.make_owned_attributes();

        attributes.push(OwnedAttribute {
            name: OwnedName {
                local_name: "autoOpen".to_string(),
                namespace: None,
                prefix: None,
            },
            value: self.auto_open.to_string(),
        });

        attributes
    }
}

impl FromXMLTag for Webcam {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        let controller_meta = ConfigurationController::from_xml_tag(event.clone())?;

        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            } => {
                if tag_name.to_string().as_str() != "Webcam" {
                    log::error!(
                        "Name of element is {}, expected Webcam",
                        tag_name.to_string()
                    );
                    return None;
                }

                let mut auto_open = false;

                for attr in attributes {
                    if attr.name.to_string().as_str() == "autoOpen" {
                        match attr.value.parse() {
                            Ok(b) => auto_open = b,
                            Err(e) => {
                                log::error!(
                                    "Failed to parse autoOpen as bool: {} ({})",
                                    e,
                                    attr.value
                                );
                                return None;
                            }
                        }
                    }
                }

                return Some(Self {
                    controller_meta,
                    auto_open,
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

/// A configuration for ethernet over usb, a child of [Robot]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetOverUsbConfiguration {
    pub controller_meta: ConfigurationController,

    /// 127.0.0.1 by default; I think? it has to be provided in the XML
    pub ip_address: IpAddr,
}

impl MakeOwnedXMLTagAttributes for EthernetOverUsbConfiguration {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        let mut attributes = self.controller_meta.make_owned_attributes();

        attributes.push(OwnedAttribute {
            name: OwnedName {
                local_name: "ipAddress".to_string(),
                namespace: None,
                prefix: None,
            },
            value: self.ip_address.to_string(),
        });

        attributes
    }
}

impl FromXMLTag for EthernetOverUsbConfiguration {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        let controller_meta = ConfigurationController::from_xml_tag(event.clone())?;

        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            } => {
                if tag_name.to_string().as_str() != "EthernetOverUsbConfiguration" {
                    log::error!(
                        "Name of element is {}, expected EthernetOverUsbConfiguration",
                        tag_name.to_string()
                    );
                    return None;
                }

                let mut ip_addr = None;

                for attr in attributes {
                    if attr.name.to_string().as_str() == "ipAddress" {
                        match IpAddr::from_str(attr.value.as_str()) {
                            Ok(ip) => ip_addr = Some(ip),
                            Err(e) => {
                                log::error!("Failed to parse ipAddress: {} ({})", e, attr.value);
                                return None;
                            }
                        }
                    }
                }

                let Some(ip_address) = ip_addr else {
                    log::error!("Missing field ipAddress in EthernetOverUsbConfiguration");
                    return None;
                };

                return Some(Self {
                    controller_meta,
                    ip_address,
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

/// Generic data for almost every device in our configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurationDevice {
    /// Always set when deserializing, needs to be set when serializing
    pub xml_tag_name: String,
    pub name: Option<String>,
    pub port: Option<u32>,
    pub bus: Option<u32>,
    /// Not actually de/serialized, but useful to save
    pub device_type: DeviceFlavor,
}

impl MakeOwnedXMLTagAttributes for ConfigurationDevice {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        let mut attributes = Vec::new();

        if let Some(name) = &self.name {
            attributes.push(OwnedAttribute {
                name: OwnedName {
                    local_name: "name".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: name.clone(),
            });
        }

        if let Some(port) = self.port {
            attributes.push(OwnedAttribute {
                name: OwnedName {
                    local_name: "port".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: port.to_string(),
            });
        }

        if let Some(bus) = self.bus {
            attributes.push(OwnedAttribute {
                name: OwnedName {
                    local_name: "bus".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: bus.to_string(),
            });
        }

        attributes
    }
}

impl ConfigurationDevice {
    pub fn from_xml_tag(
        event: xml::reader::XmlEvent,
        device_types: &Vec<HardwareDeviceType>,
    ) -> Option<Self> {
        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            } => {
                for device_type in device_types {
                    if tag_name.to_string() == device_type.xml_tag
                        || device_type.xml_tag_aliases.contains(&tag_name.to_string())
                    {
                        let mut name = None;
                        let mut port = None;
                        let mut bus = None;

                        for attr in attributes {
                            if attr.name.to_string().as_str() == "name" {
                                name = Some(attr.value);
                            } else if attr.name.to_string().as_str() == "port" {
                                port = Some(attr.value.parse().ok()?);
                            } else if attr.name.to_string().as_str() == "bus" {
                                bus = Some(attr.value.parse().ok()?);
                            }
                        }

                        return Some(Self {
                            xml_tag_name: tag_name.to_string(),
                            name,
                            port,
                            bus,
                            device_type: device_type.flavor,
                        });
                    }
                }

                return None;
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Generic data for almost every device which can have child devices
pub struct ConfigurationController {
    pub device_meta: ConfigurationDevice,
    pub serial_number: Option<String>,
}

impl MakeOwnedXMLTagAttributes for ConfigurationController {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        let mut attributes = self.device_meta.make_owned_attributes();

        if let Some(serial_number) = &self.serial_number {
            attributes.push(OwnedAttribute {
                name: OwnedName {
                    local_name: "serialNumber".to_string(),
                    namespace: None,
                    prefix: None,
                },
                value: serial_number.clone(),
            });
        }

        attributes
    }
}

impl FromXMLTag for ConfigurationController {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            } => {
                let mut name = None;
                let mut port = None;
                let mut bus = None;
                let mut serial_number = None;

                for attr in attributes {
                    if attr.name.to_string().as_str() == "name" {
                        name = Some(attr.value);
                    } else if attr.name.to_string().as_str() == "port" {
                        port = Some(attr.value.parse().ok()?);
                    } else if attr.name.to_string().as_str() == "bus" {
                        bus = Some(attr.value.parse().ok()?);
                    } else if attr.name.to_string().as_str() == "serialNumber" {
                        serial_number = Some(attr.value)
                    }
                }

                return Some(Self {
                    device_meta: ConfigurationDevice {
                        xml_tag_name: tag_name.to_string(),
                        name,
                        port,
                        bus,
                        device_type: DeviceFlavor::BuiltIn,
                    },
                    serial_number,
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}
