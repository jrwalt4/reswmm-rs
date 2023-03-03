use crate::{element::{Element, ElementRef}, table::TableBase};

#[cfg(feature="custom_nodes")]
pub use self::custom::*;

use serde::{Serialize, Deserialize};

use std::cmp::PartialEq;

pub trait Node {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NodeBase {
    invert: f64,
    kind: NodeKind
}

pub type NodeElement = Element<NodeBase>;

pub struct NodeState {
    pub ext_inflow: f64,
    pub lat_inflow: f64,
    pub overflow: f64,
    pub new_flow: f64,
    pub depth: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "node_kind")]
pub enum NodeKind {
    #[serde(alias = "junction")]
    Junction(Junction),
    Outfall(Outfall),
    Storage(Storage),
    #[cfg(feature = "custom_nodes")]
    #[serde(skip)]
    // TODO: Custom de/serialization
    Custom(CustomNode)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Junction {
    full_depth: f64,
    surch_depth: Option<f64>,
    pond_area: Option<f64>,
    init_depth: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Outfall {
    /// critical depth outfall condition
    Free,

    /// normal depth outfall condition
    Normal,

    /// fixed depth outfall condition
    Fixed(f64)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Storage {
    evap: f64,
    kind: StorageKind
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageKind {
    Functional {
        f_const: f64,
        f_coeff: f64,
        f_expon: f64
    },
    Tabular(ElementRef<TableBase>)
}

#[cfg(feature="custom_nodes")]
mod custom {
    use super::*;

    use std::fmt::{self, Debug, Formatter};
    
    pub struct CustomNode;

    impl PartialEq for CustomNode {
        fn eq(&self, other: &Self) -> bool {
            false
        }
    }

    impl Debug for CustomNode {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomNode").finish_non_exhaustive()
        }
    }
}

#[cfg(test)]
#[test]
fn serde_node() {
    use serde_json;

    let node = Element {
        uid: 1, 
        name: "N1".to_string(), 
        kind: NodeBase { 
            invert: 101.0, 
            kind: NodeKind::Junction(Junction {
                full_depth: 10.0,
                surch_depth: Some(5.0),
                pond_area: None,
                init_depth: Some(5.0)
            }),
        }
    };

    let node_serialized = serde_json::to_string(&node).unwrap();
    let node_deserialized = serde_json::from_str(&node_serialized).unwrap();
    assert_eq!(node, node_deserialized);
}
