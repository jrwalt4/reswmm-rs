use bevy_ecs::prelude::*;

#[derive(Default, Component)]
pub struct Region;

#[derive(Debug, Component)]
pub struct SubBasin {
    pub area: f64
}
