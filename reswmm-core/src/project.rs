/// Project container for nodes, links, regions, etc.

use crate::element::{Element, UID};
use crate::node::{NodeElement};
use crate::link::{LinkElement};

use std::collections::HashMap;

pub struct Project {
    nodes: HashMap<UID, NodeElement>,
    links: HashMap<UID, LinkElement>
}

impl Project {
    pub fn new() -> Self {
        Project { nodes: HashMap::new(), links: HashMap::new() }
    }

    pub fn links(&self) -> impl Iterator<Item = &LinkElement> {
        self.links.values()
    }
}

pub mod ir {
    //! Intermediate representation of a Project

    use super::*;
    use crate::element::{UID, Element, InputRef};
    use crate::node::ir::NodeIR;
    use crate::link::ir::LinkIR;
    use crate::table::TableBase;

    use serde::Serialize;

    use std::collections::HashMap;
    use std::convert::TryFrom;
    use std::iter::FromIterator;

    #[derive(Serialize)]
    pub struct ProjectIR {
        nodes: Vec<Element<NodeIR>>,
        links: Vec<Element<LinkIR>>,
        tables: Vec<Element<TableBase>>
    }

    impl TryFrom<ProjectIR> for Project {
        type Error = String;

        fn try_from(prj: ProjectIR) -> Result<Self, Self::Error> {
            let ProjectIR {
                nodes: ir_nodes,
                links: ir_links,
                tables: ir_tables
            }  = prj;
            let tables: HashMap<UID, Element<TableBase>> = ir_tables.into_iter().map(|tbl| {
                (tbl.uid, tbl)
            }).collect();
            let nodes: HashMap<UID, NodeElement> = ir_nodes.into_iter().map(|node| {
                (node.uid, Element {
                    uid: node.uid, 
                    name: node.name, 
                    kind: node.kind.into()
                })
            });

        }
    }
}
