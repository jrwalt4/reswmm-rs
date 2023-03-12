use bevy::{
    ecs::prelude::*
};

#[derive(Debug, Component)]
pub struct ExtInflow(f32);

pub fn inflow_router(mut inflows: Query<&mut ExtInflow>) {
    for mut q in &mut inflows {
        q.0 += 1.;
    }
}
