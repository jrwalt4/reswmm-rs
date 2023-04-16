//! Components and Routers for calculating hydraulics

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Depth(pub f32);

#[derive(Component)]
pub struct Inflow(pub f32);

#[derive(Component)]
pub struct Outflow(pub f32);
