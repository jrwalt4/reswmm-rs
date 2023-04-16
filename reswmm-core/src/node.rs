use bevy_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Node;

#[derive(Debug, Component)]
pub struct Invert(f32);

#[derive(Debug, Component)]
pub struct Rim(f32);

#[derive(Bundle)]
pub struct NodeBundle {
    marker: Node,
    invert: Invert,
    rim: Rim
}

impl NodeBundle {
    pub fn new(inv: f32, rim: f32) -> Self {
        Self {
            marker: Node,
            invert: Invert(inv),
            rim: Rim(rim),
        }
    }
}

#[derive(Debug, Component)]
pub struct SurchargeDepth(f32);

#[derive(Debug, Component)]
pub struct PondedArea(f32);

#[derive(Bundle)]
pub struct JunctionBundle {
    surch_depth: SurchargeDepth,
    pond_area: PondedArea,
}

#[derive(Debug, Component)]
pub enum Outfall {
    /// critical depth outfall condition
    Free,

    /// normal depth outfall condition
    Normal,

    /// fixed depth outfall condition
    Fixed(f64)
}

#[derive(Debug, Component)]
pub enum Storage {
    Functional {
        f_const: f64,
        f_coeff: f64,
        f_expon: f64
    },
    Tabular(Entity)
}
