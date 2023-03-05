use crate::router::{Router, RouterStepWorking, RouterStepFinished, Result, ModelState};

use super::hydrology::NrcsRouter;

pub struct NodeHydraulicState {
    pub overflow: f64,
    pub outflow: f64,
    pub depth: f64,
    pub volume: f64,
}

#[derive(Debug)]
pub struct KinematicRouter {
    max_iterations: usize
}

impl Router for KinematicRouter {
    type Dependency = NrcsRouter;
    
    type NodeState = NodeHydraulicState;

    type LinkState = ();

    fn execute(&self, step: RouterStepWorking<Self>, dependency: &RouterStepFinished<Self::Dependency>) -> Result<ModelState<Self>> {
        todo!()
    }
    
}

