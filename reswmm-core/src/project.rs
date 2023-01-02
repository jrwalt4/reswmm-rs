/// Project container for nodes, links, regions, etc.

use crate::element::UID;
use crate::node::NodeElement;
use crate::link::{LinkElement, LinkKind};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Project {
    nodes: HashMap<UID, NodeElement>,
    links: HashMap<UID, LinkElement>
}

impl Project {
    pub fn new() -> Self {
        Project { nodes: HashMap::new(), links: HashMap::new() }
    }
    pub fn add_node(&mut self, node: NodeElement) -> Option<NodeElement> {
        self.nodes.insert(node.uid, node)
    }

    pub fn add_link<L: Into<LinkKind>, S: ToString>(&mut self, uid: UID, name: S, link: L) -> Option<LinkElement> {
        self.links.insert(uid, LinkElement::from((uid, name.to_string(), link.into())))
    }

    pub fn links(&self) -> impl Iterator<Item = &LinkElement> {
        self.links.values()
    }
}
