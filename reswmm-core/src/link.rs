use crate::element::Element;

use enum_dispatch::enum_dispatch;
use std::ops::Deref;
use std::fmt::Debug;

#[enum_dispatch]
pub trait Link: Debug {
    fn length(&self) -> f64;
}

impl<D: Deref + Debug> Link for D 
where <D as Deref>::Target: Link{
    fn length(&self) -> f64 {
        (**self).length()
    }
}

#[enum_dispatch(Link)]
#[derive(Debug)]
pub enum LinkKind {
    Conduit(Conduit),
    #[cfg(feature = "custom_links")]
    Extension(Box<dyn Link>)
}

pub type LinkElement = Element<LinkKind>;

#[derive(Debug)]
pub struct Conduit {
    pub length: f64
}

impl Link for Conduit {
    fn length(&self) -> f64 {
        self.length
    }
}
