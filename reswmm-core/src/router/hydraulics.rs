use super::{Router, hydrology::ExtInflow};

use hecs::QueryBorrow;

pub struct Depth(pub f32);

pub struct Inflow(pub f32);

pub struct KinematicRouter {
    max_iterations: usize
}

impl<'a> Router<'a> for KinematicRouter {
    type SystemData = (&'a ExtInflow,);

    fn run(&mut self, query: QueryBorrow<'_, Self::SystemData>) {
        todo!()
    }
}

