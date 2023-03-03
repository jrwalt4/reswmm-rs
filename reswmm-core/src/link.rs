use crate::element::{Element, ElementRef};
use crate::node::NodeBase;
use crate::table::TableBase;
use crate::xsection::XSection;

#[cfg(feature="custom_links")]
pub use self::custom::CustomLink;

use serde::{Serialize, Deserialize};

pub trait Link {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkBase {
    us_node: ElementRef<NodeBase>,
    ds_node: ElementRef<NodeBase>,
    
    flow_init: f32,
    flow_limit: f32,
    
    k_entrance: f32,
    k_avg: f32,
    k_exit: f32,
    
    flap_gate: bool,

    kind: LinkKind
}

pub type LinkElement = Element<LinkBase>;

pub struct LinkState {
    pub flow: f32,
    pub depth: f32,
    pub volume: f32,
    pub surf_area: (f32, f32),
    pub froude: f32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "link_kind")]
pub enum LinkKind {
    Conduit(Conduit),
    Orifice(Orifice),
    Weir(Weir),
    Outlet(Outlet),
    // TODO: Pump(Pump),
    #[cfg(feature = "custom_links")]
    #[serde(skip)]
    // TODO: de/serailization
    Custom(CustomLink)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Conduit {
    length: f64,
    roughness: f64,
    barrels: u8,
    xsect: XSection
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Orifice {
    kind: OrificeKind,
    shape: XSection,
    cd: f64,
    open_rate: f64
}

#[derive(Debug, Serialize, Deserialize, PartialEq )]
pub enum OrificeKind {
    Side,
    Bottom
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Weir {
    kind: WeirKind,
    cd_1: f64,
    cd_2: f64,
    contractions: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq )]
pub enum WeirKind {
    Transverse,
    Sideflow,
    Notch,
    Trapezoid,
    Roadway
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Outlet {
    Functional {
        coeff: f64,
        expon: f64
    },
    Tabular(ElementRef<TableBase>),
}

#[cfg(feature="custom_links")]
mod custom {
    use super::*;

    use std::fmt::{self, Debug, Formatter};

    pub struct CustomLink(Box<dyn Link>);

    impl CustomLink {
        pub fn new(custom: Box<dyn Link>) -> Self {
            CustomLink(custom)
        }
    }

    impl Debug for CustomLink {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomLink").finish_non_exhaustive()
        }
    }

}
