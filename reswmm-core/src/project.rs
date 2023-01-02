
use crate::element::{Element, UID};
use crate::node::NodeKind;
use crate::link::LinkKind;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Project {
    nodes: HashMap<UID, Element<NodeKind>>,
    links: HashMap<UID, Element<LinkKind>>
}
