use specs::{Component, DenseVecStorage, NullStorage};

#[derive(Default)]
pub struct Region;

impl Component for Region {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Component)]
pub struct SubBasin {
    pub area: f64
}
