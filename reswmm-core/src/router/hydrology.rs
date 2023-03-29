use specs::{prelude::*, Component, DenseVecStorage};

#[derive(Debug, Component)]
pub struct ExtInflow(f32);

pub struct InflowRouter;

impl<'a> System<'a> for InflowRouter {
    type SystemData = (Entities<'a>, WriteStorage<'a, ExtInflow>);

    fn run(&mut self, (entities, mut inflows): Self::SystemData) {
        for (id, q) in (&entities, &mut inflows).join() {
            q.0 += 1.;
        }
    }

    
}
