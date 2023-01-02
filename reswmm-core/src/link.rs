use crate::element::Element;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use std::fmt::{self, Debug, Formatter};

#[enum_dispatch]
pub trait Link {
    fn length(&self) -> f64;
}

#[enum_dispatch(Link)]
#[derive(Debug, Serialize, Deserialize)]
pub enum LinkKind {
    Conduit(Conduit),
    #[cfg(feature = "custom_links")]
    #[serde(skip)]
    // TODO: de/serailization
    Custom(CustomLink)
}

pub type LinkElement = Element<LinkKind>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conduit {
    pub length: f64
}

impl Link for Conduit {
    fn length(&self) -> f64 {
        self.length
    }
}
#[cfg(feature="custom_links")]
mod custom {
    use super::*;
    pub struct CustomLink(Box<dyn Link>);

    impl CustomLink {
        pub fn new(custom: Box<dyn Link>) -> Self {
            CustomLink(custom)
        }
    }

    impl Link for CustomLink {
        fn length(&self) -> f64 {
            self.0.length()
        }
    }

    impl Debug for CustomLink {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomLink").finish_non_exhaustive()
        }
    }
}
#[cfg(feature="custom_links")]
pub use custom::*;
