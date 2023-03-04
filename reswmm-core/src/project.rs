/// Project container for nodes, links, regions, etc.

use crate::element::{Element, UID};
use crate::node::{NodeElement};
use crate::link::{LinkElement};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Project {
    model: Model
}

impl Project {
    pub fn new() -> Self {
        Project { model: Model::new() }
    }
}

pub struct Model {
    nodes: HashMap<UID, NodeElement>,
    links: HashMap<UID, LinkElement>
}

impl Model {
    pub fn new() -> Self {
        Self { 
            nodes: HashMap::new(), 
            links: HashMap::new()
        }
    }

    pub fn get_node<U: Hash + Eq>(&self, uid: &U) -> Option<&NodeElement> 
    where UID: Borrow<U> {
        self.nodes.get(uid)
    }
}
