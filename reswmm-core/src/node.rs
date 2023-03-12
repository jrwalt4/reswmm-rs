use bevy::{
    ecs::prelude::*
};

#[derive(Debug, Component)]
pub struct Node {
    invert: f64,
    rim: f64
}

#[derive(Debug, Component)]
pub struct Junction {
    surch_depth: Option<f64>,
    pond_area: Option<f64>,
    init_depth: Option<f64>,
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
pub struct FunctionalStorage {
    evap: f64,
    f_const: f64,
    f_coeff: f64,
    f_expon: f64
}

// TODO: TabularStorage Component (do we define here or in table.rs?)
