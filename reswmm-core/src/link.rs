use crate::xsection::XSection;

use bevy_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Link {
    pub us: Entity, 
    pub ds: Entity
}

#[derive(Bundle)]
pub struct LinkBundle {
    marker: Link,
    length: Length
}

#[derive(Debug, Component)]
pub struct Length(pub f32);

#[derive(Bundle)]
pub struct ConduitBundle {
    xs: XSection,
    km: MinorLoss,
}

#[derive(Debug, Default, Component, Clone, Copy)]
pub struct MinorLoss {
    pub ent: f32,
    pub avg: f32,
    pub ext: f32,
}

impl MinorLoss {
    pub fn new() {
        Default::default()
    }

    pub fn total(&self) -> f32 {
        self.ent + self.avg + self.ext
    }
}

/// Manning's roughness
#[derive(Debug, Component)]
pub struct RoughnessMN(f32);

/// Hazen-Williams C factor
#[derive(Debug, Component)]
pub struct RoughnessHW(f32);


#[derive(Debug, Component)]
pub struct Orifice {
    pub kind: OrificeKind,
    pub cd: f64,
    pub open_rate: f64
}

#[derive(Debug)]
pub enum OrificeKind {
    Side,
    Bottom
}

#[derive(Debug, Component)]
pub struct Weir {
    pub kind: WeirKind,
    pub cd_1: f64,
    pub cd_2: f64,
    pub contractions: u8
}

#[derive(Debug)]
pub enum WeirKind {
    Transverse,
    Sideflow,
    Notch,
    Trapezoid,
    Roadway
}

#[derive(Debug, Component)]
pub struct Outlet {
    pub coeff: f64,
    pub expon: f64
}
