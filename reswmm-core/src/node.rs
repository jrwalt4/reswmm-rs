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
    #[cfg(feature = "custom_nodes")]
    Extension(Box<dyn Node>)
}

pub type NodeElement = Element<NodeKind>;

pub struct Junction {
    invert: f64
}

impl Node for Junction {
    fn invert(&self) -> f64 {
        self.invert
    }
}
