use crate::node::Node;

use super::{hydrology::{
    WetInflow,
    DryInflow,
    ExtInflow
}, Nodes};

use bevy::{
    ecs::prelude::*
};

pub struct Depth(pub f32);

pub struct Inflow(pub f32);

pub struct Outflow(pub f32);

pub fn kinematic_router(
    nodes: Res<Nodes>,
    q_wet: Query<(Entity, &WetInflow), With<Node>>, 
    q_dry: Query<(Entity, &DryInflow), With<Node>>,
    q_ext: Query<(Entity, &ExtInflow), With<Node>>
) {
    let node_flows = nodes.map(|(id, _uid)| {
        let mut q = 0.0;
        q += q_wet.get_component::<WetInflow>(id).map_or(0.0, Into::into);
        q += q_dry.get_component::<DryInflow>(id).map_or(0.0, Into::into);
        q += q_ext.get_component::<ExtInflow>(id).map_or(0.0, Into::into);
        q
    });
    for q in node_flows.values() {
        println!("Q = {q}");
    }
}
