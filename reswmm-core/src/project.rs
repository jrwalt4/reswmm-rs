
use crate::element::{Element, UID};
use crate::input::{Input, Inserter};
use crate::node::NodeKind;
use crate::link::LinkKind;

use std::collections::HashMap;

pub struct Project {
    nodes: HashMap<UID, Element<NodeKind>>,
    links: HashMap<UID, Element<LinkKind>>
}

impl Project {
    pub fn from_input<I: Input<NodeKind>>(input: I) -> Project {
        let node_inserter = Inserter::<NodeKind>::new();
        Project {
            nodes: Default::default(),
            links: Default::default()
        }
    }
}
