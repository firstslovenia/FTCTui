//! Contains Lynx USB Device, LynxModule and ServoHub types
//! for the robot hardware configuration

use serde::{Deserialize, Serialize};
use xml::{attribute::OwnedAttribute, name::OwnedName};

use crate::ftc_proto::hardware::{
    FromXMLTag, MakeOwnedXMLTagAttributes,
    robot::{ConfigurationController, ConfigurationDevice},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Our Control Hub Portal, parent of [LynxModule] (Control and Expansion hub) and [ServoHub]
pub struct LynxUSBDevice {
    pub controller_meta: ConfigurationController,
    pub parent_module_address: u32,
    pub lynx_modules: Vec<LynxModule>,
    pub servo_hubs: Vec<ServoHub>,
}

impl MakeOwnedXMLTagAttributes for LynxUSBDevice {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        let mut attributes = self.controller_meta.make_owned_attributes();

        attributes.push(OwnedAttribute {
            name: OwnedName {
                local_name: "parentModuleAddress".to_string(),
                namespace: None,
                prefix: None,
            },
            value: self.parent_module_address.to_string(),
        });

        attributes
    }
}

impl FromXMLTag for LynxUSBDevice {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        let controller_meta = ConfigurationController::from_xml_tag(event.clone())?;

        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes,
                namespace: _namespace,
            } => {
                if tag_name.to_string().as_str() != "LynxUsbDevice" {
                    log::error!(
                        "Name of element is {}, expected LynxUsbDevice",
                        tag_name.to_string()
                    );
                    return None;
                }

                let mut parent_module_address = None;

                for attr in attributes {
                    if attr.name.to_string().as_str() == "parentModuleAddress" {
                        parent_module_address = attr.value.parse().ok();
                    }
                }

                let Some(parent_module_address) = parent_module_address else {
                    log::error!("Parent module address is missing!");
                    return None;
                };

                // These vectors need to be filled in later
                return Some(Self {
                    controller_meta,
                    parent_module_address,
                    lynx_modules: Vec::new(),
                    servo_hubs: Vec::new(),
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Our tag which is actually the parent of our motors, servos, ...
///
/// Used for Control and Expansion hubs
pub struct LynxModule {
    pub controller_meta: ConfigurationController,
    pub servos: Vec<ConfigurationDevice>,
    pub motors: Vec<ConfigurationDevice>,
    pub i2c_devices: Vec<ConfigurationDevice>,
    pub digital_devices: Vec<ConfigurationDevice>,
    pub pwm_outputs: Vec<ConfigurationDevice>,
    pub analog_inputs: Vec<ConfigurationDevice>,
}

impl LynxModule {
    /// Returns a lits of all devices (concatenation of servos, motors, i2c_devices, ...)
    ///
    /// Leaves the devices in self empty
    pub fn all_devices(&mut self) -> Vec<ConfigurationDevice> {
        let mut devices = Vec::new();
        devices.append(&mut self.servos);
        devices.append(&mut self.motors);
        devices.append(&mut self.i2c_devices);
        devices.append(&mut self.digital_devices);
        devices.append(&mut self.pwm_outputs);
        devices.append(&mut self.analog_inputs);
        return devices;
    }
}

impl MakeOwnedXMLTagAttributes for LynxModule {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        self.controller_meta.make_owned_attributes()
    }
}

impl FromXMLTag for LynxModule {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        let controller_meta = ConfigurationController::from_xml_tag(event.clone())?;

        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes: _attributes,
                namespace: _namespace,
            } => {
                if tag_name.to_string().as_str() != "LynxModule" {
                    log::error!(
                        "Name of element is {}, expected LynxModule",
                        tag_name.to_string()
                    );
                    return None;
                }

                // These vectors need to be filled in later
                return Some(Self {
                    controller_meta,
                    analog_inputs: Vec::new(),
                    pwm_outputs: Vec::new(),
                    digital_devices: Vec::new(),
                    i2c_devices: Vec::new(),
                    motors: Vec::new(),
                    servos: Vec::new(),
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// External REV servo hub,
/// see REV-11-1855
pub struct ServoHub {
    pub controller_meta: ConfigurationController,
    pub servos: Vec<ConfigurationDevice>,
}

impl MakeOwnedXMLTagAttributes for ServoHub {
    fn make_owned_attributes(&self) -> Vec<xml::attribute::OwnedAttribute> {
        self.controller_meta.make_owned_attributes()
    }
}

impl FromXMLTag for ServoHub {
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self> {
        let controller_meta = ConfigurationController::from_xml_tag(event.clone())?;

        match event {
            xml::reader::XmlEvent::StartElement {
                name: tag_name,
                attributes: _attributes,
                namespace: _namespace,
            } => {
                if tag_name.to_string().as_str() != "ServoHub" {
                    log::error!(
                        "Name of element is {}, expected ServoHub",
                        tag_name.to_string()
                    );
                    return None;
                }

                // These vectors need to be filled in later
                return Some(Self {
                    controller_meta,
                    servos: Vec::new(),
                });
            }
            _ => {
                log::error!("Event is not a StartElement event!");
                None
            }
        }
    }
}
