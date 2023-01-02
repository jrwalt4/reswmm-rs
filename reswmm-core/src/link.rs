use crate::element::Element;
#[cfg(feature="custom_links")]
pub use custom::CustomLink;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

#[enum_dispatch]
pub trait Link {
    fn length(&self) -> f64;
}

#[enum_dispatch(Link)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "link_kind")]
pub enum LinkKind {
    Conduit(Conduit),
    #[cfg(feature = "custom_links")]
    #[serde(skip)]
    // TODO: de/serailization
    Custom(CustomLink)
}

pub type LinkElement = Element<LinkKind>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Conduit {
    pub length: f64
}

impl Conduit {
    pub fn new(length: f64) -> Self {
        Self { length }
    }
}

impl Link for Conduit {
    fn length(&self) -> f64 {
        self.length
    }
}
#[cfg(feature="custom_links")]
mod custom {
    use super::*;

    use std::fmt::{self, Debug, Formatter};

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

    impl std::cmp::PartialEq for CustomLink {
        fn eq(&self, other: &Self) -> bool {
            self.0.length() == other.0.length()
        }
    }
}
