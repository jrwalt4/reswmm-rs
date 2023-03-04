//! router

pub mod hydraulics;

use crate::element::{UID, Element};
use crate::node::NodeBase;
use crate::project::Model;
use crate::time::{Interval, Time, Duration};
use crate::error::{Result};

use std::collections::HashMap;
use std::sync::Arc;

pub trait Router {
    
    type NodeState;
    
    type LinkState;

    // type RegionState;
}

pub struct ModelState<R: Router> {
    pub time: Time,
    nodes: HashMap<UID, R::NodeState>,
    links: HashMap<UID, R::LinkState>
}

impl<R: Router> ModelState<R> {
    pub fn empty_from<'m>(other: &'m ModelState<R>, time: Time) -> Self {
        Self {
            time,
            nodes: HashMap::with_capacity_and_hasher(other.nodes.capacity(), other.nodes.hasher().clone()),
            links: HashMap::with_capacity_and_hasher(other.links.capacity(), other.links.hasher().clone())
        }
    }
}

pub struct ElementState<'e, K, S> {
    pub element: &'e Element<K>,
    pub state: S
}

/// Holds a previous state and a next state. 
/// The two states can have different lifetime and/or 
/// mutability requirements (generally `prev` will be readonly
/// while `next` can be mutable during routing)
pub struct StateStep<P, N> {
    pub prev: P,
    pub next: N
}

pub type ElementStep<'e, K, S> = ElementState<'e, K, StateStep<&'e S, &'e S>>;

pub type ElementStepMut<'e, K, S> = ElementState<'e, K, StateStep<&'e S, &'e mut S>>;

pub struct RouterStep<P, N> {
    interval: Interval,
    state: StateStep<P, N>,
    model: Arc<Model>
}

/// A finished calculation that can be used in downstream routers
pub type RouterStepFinished<R> = RouterStep<Arc<ModelState<R>>, Arc<ModelState<R>>>;

/// A working router computation used by a single working `Router`
pub type RouterStepWorking<R> = RouterStep<Arc<ModelState<R>>, ModelState<R>>;

impl<R: Router> RouterStepFinished<R> {
    pub fn get_node(&self, uid: UID) -> Option<ElementStep<'_, NodeBase, R::NodeState>> {
        let element = self.model.get_node(&uid);
        let prev_state = self.state.prev.nodes.get(&uid);
        let next_state = self.state.next.nodes.get(&uid);
        let state = prev_state.zip(next_state);
        element.zip(state).map(|(element, (prev, next))| {
            ElementState {
                element,
                state: StateStep { prev, next }
            }
        })
    }
}

impl<R: Router> RouterStepWorking<R> {
    pub fn advance(&mut self, by: Duration) -> Result<Arc<ModelState<R>>> {
        self.interval = self.interval.advance(by);// std::mem::replace(&mut self.interval.0, self.interval.1);
        self.interval.1 += by;
        let (_prev_time, next_time) = self.interval.range();
        let new_next = ModelState::empty_from(&self.state.next, next_time);
        // 'commit' `next` as the new `prev`
        let new_prev = Arc::new(std::mem::replace(&mut self.state.next, new_next));
        // drop `old_prev`
        self.state.prev = Arc::clone(&new_prev);
        // TODO: check for convergence/valid results
        return Ok(new_prev)
    }
}
