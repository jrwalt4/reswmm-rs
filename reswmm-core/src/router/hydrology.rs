use bevy::{
    ecs::prelude::*
};
use reswmm_macros::IntoReal;

#[derive(Debug, Component, IntoReal)]
pub struct WetInflow(pub f32);

#[derive(Debug, Component, IntoReal)]
pub struct DryInflow(pub f32);

#[derive(Debug, Component, IntoReal)]
pub struct ExtInflow(pub f32);
