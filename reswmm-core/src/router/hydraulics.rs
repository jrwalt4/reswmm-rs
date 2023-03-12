use super::hydrology::ExtInflow;

use bevy::{
    ecs::prelude::*
};

#[derive(Component)]
pub struct Depth(pub f32);

#[derive(Component)]
pub struct Inflow(pub f32);

pub fn kinematic_router(_query: Query<&ExtInflow>) {
    todo!()
}
