use hecs::Entity;

#[derive(Debug)]
pub struct Link {
    us_node: Entity,
    ds_node: Entity,
}

#[derive(Debug)]
pub struct Length(pub f32);

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Outlet {
    coeff: f64,
    expon: f64
}
