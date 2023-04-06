use bevy::{
    ecs::prelude::*
};

#[derive(Debug, Component)]
pub struct WetInflow(pub f32);

#[derive(Debug, Component)]
pub struct DryInflow(pub f32);

#[derive(Debug, Component)]
pub struct ExtInflow(pub f32);
