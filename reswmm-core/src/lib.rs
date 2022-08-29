extern crate enum_dispatch;
pub extern crate furlong as units;
pub mod xsection;
pub mod element;
pub mod node;
pub mod link;

use std::{collections::HashMap};

use element::UID;
use link::{LinkElement, LinkKind};
use node::NodeElement;

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
}

#[cfg(test)]
mod test {
    use crate::link::{Conduit};

    use super::*;
    #[test]
    fn add_link() {
        let mut prj = Project::new();
        let l = Conduit{length: 2.0};
        prj.add_link(1, "L-1", l);

        #[cfg(feature="custom_links")]
        {
            let l2: Box<dyn link::Link> = Box::new(Conduit{length: 3.0});
            prj.add_link(2, "L-2", l2);
        }
    }
}

pub fn run() {
    use crate::link::{Conduit};

    let mut prj = Project::new();
    let l = Conduit{length: 2.0};
    prj.add_link(1, "L-1", l);

    #[cfg(feature="custom_links")]
    {
        let l2: Box<dyn link::Link> = Box::new(Conduit{length: 3.0});
        prj.add_link(2, "L-2", l2);
    }

    for link in prj.links {
        println!("{link:?}");
    }
}
