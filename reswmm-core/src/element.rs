use specs::{Component, DenseVecStorage};

#[derive(Debug, Component)]
pub struct UID(pub i32);

#[derive(Debug, Component)]
pub struct Name(pub String);
