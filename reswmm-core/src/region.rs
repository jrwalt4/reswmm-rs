use crate::element::Element;

use std::fmt::Debug;

pub trait Region: Debug {
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub enum RegionKind {
    SubBasin(SubBasin),
    #[cfg(feature = "custom_regions")]
    Extension(Box<dyn Region>)
}

pub type RegionElement = Element<RegionKind>;

impl Region for RegionKind {
    fn area(&self) -> f64 {
        use RegionKind::*;
        match self {
            SubBasin(sb) => sb.area(),
            #[cfg(feature = "custom_regions")]
            Extension(region) => region.area()
        }
    }
}

#[derive(Debug)]
pub struct SubBasin {
    pub area: f64
}

impl Region for SubBasin {
    fn area(&self) -> f64 {
        self.area
    }
}
