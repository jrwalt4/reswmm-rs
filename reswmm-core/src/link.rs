use bevy_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Link {
    us_node: Entity,
    ds_node: Entity,
}

#[derive(Debug, Component)]
pub struct Length(pub f32);

#[derive(Debug, Component)]
pub struct Conduit {
    flow_init: f32,
    flow_limit: f32,
    
    k_entrance: f32,
    k_avg: f32,
    k_exit: f32,
    
    flap_gate: bool,

    roughness: f64,
    barrels: u8,
}

#[derive(Debug, Component)]
pub struct Orifice {
    kind: OrificeKind,
    cd: f64,
    open_rate: f64
}

#[derive(Debug)]
pub enum OrificeKind {
    Side,
    Bottom
}

#[derive(Debug, Component)]
pub struct Weir {
    kind: WeirKind,
    cd_1: f64,
    cd_2: f64,
    contractions: u8
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
    coeff: f64,
    expon: f64
}
