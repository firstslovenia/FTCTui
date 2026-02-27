//! Top level module for the robot's hardware configuration
//!
//! Contains some useful traits for the XML parsing

use xml::attribute::{Attribute, OwnedAttribute};

pub mod device;
pub mod document;
pub mod lynx;
pub mod robot;

/// Trait for objects that can be written as XML tags
pub trait MakeXMLTag {
    /// Returns an event to open the tag
    fn opening_event(&self) -> xml::writer::XmlEvent<'_>;

    /// Returns an event to close the tag
    ///
    /// Technically this doesn't need to be specific for an event, but it
    /// reduces potential problems to have it
    fn closing_event(&self) -> xml::writer::XmlEvent<'_>;
}

/// Trait for objects that have values that are expressed as XML Attributes
pub trait MakeXMLTagAttributes {
    /// Returns a list of attributes that exist within self
    fn make_attributes(&self) -> Vec<Attribute<'_>>;
}

/// Trait for objects that have values that are expressed as owned XML Attributes
pub trait MakeOwnedXMLTagAttributes {
    /// Returns a list of attributes that exist within self
    fn make_owned_attributes(&self) -> Vec<OwnedAttribute>;
}

/// Trait for objects that can be potentially created from the xml opening tag event
pub trait FromXMLTag {
    /// Tries to construct self from the opening tag event
    fn from_xml_tag(event: xml::reader::XmlEvent) -> Option<Self>
    where
        Self: Sized;
}
