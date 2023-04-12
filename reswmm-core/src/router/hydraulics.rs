use super::{
    Next, NextMut,
    hydrology::ExtInflow
};

pub struct Depth(pub f32);

pub struct Inflow(pub f32);

pub fn inflow_router(inflow: NextMut<&mut Inflow>, ext_inflow: Next<&ExtInflow>) {
    
}
