//! router

pub mod hydraulics;

use crate::element::{UID, Element};
use crate::node::NodeBase;
use crate::project::Model;
use crate::time::Interval;

use std::collections::HashMap;
use std::sync::Arc;

pub trait Router {
    
    type NodeState;
    
    type LinkState;

    // type RegionState;
}

pub struct ModelState<R: Router> {
    nodes: HashMap<UID, R::NodeState>,
    links: HashMap<UID, R::LinkState>
}

pub struct ElementState<'e, K, S> {
    element: &'e Element<K>,
    state: S
}

pub type ElementStep<'e, K, S> = ElementState<'e, K, (&'e S, &'e S)>;

pub struct RouterStep<R: Router> {
    interval: Interval,
    prev: Arc<ModelState<R>>,
    next: ModelState<R>,
    model: Arc<Model>
}

impl<R: Router> RouterStep<R> {
    pub fn get_node(&self, uid: UID) -> Option<ElementStep<'_, NodeBase, R::NodeState>> {
        let element = self.model.get_node(&uid);
        let prev_state = self.prev.nodes.get(&uid);
        let next_state = self.next.nodes.get(&uid);
        let state = prev_state.zip(next_state);
        element.zip(state).map(|(element, state)| {
            ElementState {
                element,
                state
            }
        })
    }
}


