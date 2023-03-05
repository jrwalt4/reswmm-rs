//! router

pub mod hydrology;
pub mod hydraulics;

use crate::element::{UID, Element};
use crate::node::NodeBase;
use crate::project::Model;
use crate::time::{Interval, Time, Duration};

use std::collections::HashMap;
use std::sync::Arc;

pub trait Router: Sized {

    type Dependency: Router;
    
    type NodeState;
    
    type LinkState;

    // type RegionState;

    fn execute(&self, step: RouterStepWorking<Self>, dependency: &RouterStepFinished<Self::Dependency>) -> Result<ModelState<Self>>;
}

impl Router for () {
    type Dependency = ();
    type NodeState = ();
    type LinkState = ();
    fn execute(&self, step: RouterStepWorking<Self>, _dependency: &RouterStepFinished<Self::Dependency>) -> Result<ModelState<Self>> {
        step.commit()
    }
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
    model: Arc<Model>,
    messages: Vec<Message>
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

    pub fn warning(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn error(self, err: Error) -> Result<ModelState<R>> {
        Err(err)
    }

    pub fn commit(self) -> Result<ModelState<R>> {
        // TODO: report warnings
        Ok(self.state.next)
    }
}

pub enum Message {
    MaxIterations,
    Tolerance,
    Custom(String),
}

pub enum Error {
    BadIndex(UID, usize),
    Convergence(f64),
    Other(String)
}

pub type Result<T> = std::result::Result<T, Error>;
