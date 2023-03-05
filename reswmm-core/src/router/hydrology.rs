use crate::router::{Router, RouterStepWorking, RouterStepFinished, Result, ModelState};

pub struct NodeHydrologicState {
    pub dry_inflow: f64,
    pub wet_inflow: f64,
}

pub struct NrcsRouter;

impl Router for NrcsRouter {
    type Dependency = ();

    type LinkState = ();

    type NodeState = NodeHydrologicState;

    fn execute(&self, step: RouterStepWorking<Self>, _dependency: &RouterStepFinished<Self::Dependency>) -> Result<ModelState<Self>> {
        step.commit()
    }
}
