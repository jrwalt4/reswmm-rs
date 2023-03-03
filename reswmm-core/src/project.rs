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
