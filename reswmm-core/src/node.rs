use crate::element::Element;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use std::cmp::PartialEq;

#[enum_dispatch]
pub trait Node {
    fn invert(&self) -> f64;
}

#[enum_dispatch(Node)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum NodeKind {
    #[serde(alias = "junction")]
    Junction(Junction),
    #[cfg(feature = "custom_nodes")]
    #[serde(skip)]
    // TODO: Custom de/serialization
    Custom(CustomNode)
}

pub type NodeElement = Element<NodeKind>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Junction {
    invert: f64
}

impl Junction {
    pub fn new(invert: f64) -> Self {
        Self { invert }
    }
}

impl Node for Junction {
    fn invert(&self) -> f64 {
        self.invert
    }
}

#[cfg(feature="custom_nodes")]
mod custom {
    use super::*;

    use std::fmt::{self, Debug, Formatter};
    
    pub struct CustomNode(Box<dyn Node>);

    impl Node for CustomNode {
        fn invert(&self) -> f64 {
            self.0.invert()
        }
    }

    impl PartialEq for CustomNode {
        fn eq(&self, other: &Self) -> bool {
            self.0.invert() == other.0.invert()
        }
    }

    impl Debug for CustomNode {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomNode").finish_non_exhaustive()
        }
    }
}
#[cfg(feature="custom_nodes")]
pub use custom::*;
