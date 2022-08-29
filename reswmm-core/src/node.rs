use crate::element::Element;
use enum_dispatch::enum_dispatch;
use std::ops::Deref;

#[enum_dispatch]
pub trait Node {
    fn invert(&self) -> f64;
}

impl<D: Deref> Node for D
where <D as Deref>::Target: Node {
    fn invert(&self) -> f64 {
        (**self).invert()
    }
}

#[enum_dispatch(Node)]
pub enum NodeKind {
    Junction(Junction),
    Extension(Box<dyn Node>)
}

/*
impl Node for NodeKind {
    fn invert(&self) -> f64 {
        match self {
            Self::Junction(j) => j.invert(),
            Self::Extension(b) => b.invert()
        }
    }
}
// */

pub type NodeElement = Element<NodeKind>;

pub struct Junction {
    invert: f64
}

impl Node for Junction {
    fn invert(&self) -> f64 {
        self.invert
    }
}
