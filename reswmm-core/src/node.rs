use crate::{element::{Element, Ref}, table::TableBase};

#[cfg(feature="custom_nodes")]
pub use self::custom::*;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use std::cmp::PartialEq;

#[enum_dispatch]
pub trait Node {
    fn invert(&self) -> f64;
}

#[derive(Debug)]
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

#[enum_dispatch(Node)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "node_kind")]
pub enum NodeKind {
    #[serde(alias = "junction")]
    Junction(Junction),
    #[cfg(feature = "custom_nodes")]
    #[serde(skip)]
    // TODO: Custom de/serialization
    Custom(CustomNode)
}

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

pub enum Storage {
    Functional {
        evap: f64,
        f_const: f64,
        f_coeff: f64,
        f_expon: f64
    },
    Tabular(Ref<TableBase>)
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

pub mod ir {
    //! Intermediate Representation of node elements

    use super::*;

    use crate::element::{UID, ir::{RefIR, Resolver, ResolveWith}};
    use crate::table::TableBase;

    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct NodeIR {
        pub invert_elev: f64,
        pub full_depth: f64,
        pub surch_depth: Option<f64>,
        pub pond_area: Option<f64>,
        pub init_depth: Option<f64>,
        pub kind: NodeKindIR
    }

    impl<R: Resolver<TableBase>> ResolveWith<R> for NodeIR {
        type Output = NodeKind;

        fn resolve(self, resolver: R) -> Option<Self::Output> {
            use NodeKindIR as IR;
            let kind = match self.kind {
                IR::Junction => Some(NodeKind::Junction(Junction::new(self.invert_elev))),
                IR::Outfall(outf) => unimplemented!(),
                IR::Storage(stor) => {
                    use StorageKindIR::*;
                    match stor {
                        Functional {
                            evap, f_const, f_coeff, f_expon 
                        } => Some(Storage::Functional { evap, f_const, f_coeff, f_expon }),
                        Tabular(ref_ir) => resolver(ref_ir).map(|r| Storage::Tabular(r).into())
                    }
                }
            }
        }

        
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum NodeKindIR {
        Junction,
        Outfall(OutfallKindIR),
        Storage(StorageKindIR)
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum OutfallKindIR {

        /// critical depth outfall condition
        Free,

        /// normal depth outfall condition
        Normal,

        /// fixed depth outfall condition
        Fixed(f64)
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum StorageKindIR {
        Functional {
            evap: f64,
            f_const: f64,
            f_coeff: f64,
            f_expon: f64
        },
        Tabular(RefIR<TableBase>)
    }

    #[cfg(test)]
    #[test]
    fn serde_node() {
        use serde_json;

        let node = Element {
            uid: 1, 
            name: "N1".to_string(), 
            kind: NodeIR { 
                kind: NodeKindIR::Junction,
                invert_elev: 101.0, 
                full_depth: 10.0,
                surch_depth: None,
                pond_area: None,
                init_depth: None,
            }
        };

        let node_serialized = serde_json::to_string(&node).unwrap();
        let node_deserialized = serde_json::from_str(&node_serialized).unwrap();
        assert_eq!(node, node_deserialized);
    }

}
