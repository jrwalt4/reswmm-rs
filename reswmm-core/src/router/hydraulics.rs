use super::hydrology::ExtInflow;

use specs::{Component, prelude::*};

#[derive(Component)]
pub struct Depth(pub f32);

#[derive(Component)]
pub struct Inflow(pub f32);

pub struct KinematicRouter {
    max_iterations: usize
}

impl<'a> System<'a> for KinematicRouter {
    type SystemData = ReadStorage<'a, ExtInflow>;

    fn run(&mut self, ext_inflow: Self::SystemData) {
        todo!()
    }
}

