use crate::element::{Element, Ref};
use crate::node::{NodeElement};

#[cfg(feature="custom_links")]
pub use self::custom::CustomLink;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

#[enum_dispatch]
pub trait Link {
    fn length(&self) -> f64;
}

#[derive(Debug)]
pub struct LinkBase {
    length: f64,
    us_node: Ref<NodeElement>
}

pub type LinkElement = Element<LinkBase>;

pub struct LinkState {
    pub flow: f32,
    pub depth: f32,
    pub volume: f32,
    pub surf_area: (f32, f32),
    pub froude: f32
}

#[enum_dispatch(Link)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "link_kind")]
pub enum LinkKind {
    Conduit(Conduit),
    #[cfg(feature = "custom_links")]
    #[serde(skip)]
    // TODO: de/serailization
    Custom(CustomLink)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Conduit {
    pub length: f64
}

impl Conduit {
    pub fn new(length: f64) -> Self {
        Self { length }
    }
}

impl Link for Conduit {
    fn length(&self) -> f64 {
        self.length
    }
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

    impl Link for CustomLink {
        fn length(&self) -> f64 {
            self.0.length()
        }
    }

    impl Debug for CustomLink {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomLink").finish_non_exhaustive()
        }
    }

    impl std::cmp::PartialEq for CustomLink {
        fn eq(&self, other: &Self) -> bool {
            self.0.length() == other.0.length()
        }
    }
}

pub mod ir {
    //! Intermediate Representation of link elements

    use crate::element::InputRef;
    use crate::node::NodeKind;

    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LinkIR {
        us_node: InputRef<NodeKind>,
        ds_node: InputRef<NodeKind>,

        flow_init: f32,
        flow_limit: f32,
        k_entrance: f32,
        k_avg: f32,
        k_exit: f32,
        flap_gate: bool
    }
}
