use crate::element::Element;
#[cfg(feature = "custom_regions")]
use self::custom::CustomRegion;

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

#[enum_dispatch]
pub trait Region {
    fn area(&self) -> f64;
}

#[enum_dispatch(Region)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "region_kind")]
pub enum RegionKind {
    SubBasin(SubBasin),
    #[cfg(feature = "custom_regions")]
    #[serde(skip)]
    // TODO: custom de/serialization
    Custom(CustomRegion)
}

pub type RegionElement = Element<RegionKind>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SubBasin {
    pub area: f64
}

impl SubBasin {
    pub fn new(area: f64) -> Self {
        Self { area }
    }
}

impl Region for SubBasin {
    fn area(&self) -> f64 {
        self.area
    }
}

#[cfg(feature = "custom_regions")]
mod custom {
    use super::*;

    use std::fmt::{self, Debug, Formatter};

    pub struct CustomRegion(Box<dyn Region>);

    impl CustomRegion {
        pub fn new(region: Box<dyn Region>) -> Self {
            CustomRegion(region)
        }
    }

    impl Region for CustomRegion {
        fn area(&self) -> f64 {
            self.0.area()
        }
    }

    impl Debug for CustomRegion {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomRegion").finish_non_exhaustive()
        }
    }

    impl std::cmp::PartialEq for CustomRegion {
        fn eq(&self, other: &Self) -> bool {
            self.0.area() == other.0.area()
        }
    }
}
