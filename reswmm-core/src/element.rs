use bevy::{
    ecs::prelude::*
};

#[derive(Debug, Component, Copy, Clone)]
pub struct UID(pub i32);

#[derive(Debug, Component)]
pub struct Name(pub String);
