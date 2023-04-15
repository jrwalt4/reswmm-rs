//! router

pub mod hydrology;
pub mod hydraulics;

use crate::element::UID;

use std::collections::HashMap;

use bevy_ecs::{
    prelude::*,
    system::IntoSystem
};

/// A router is really just a [bevy System](bevy_ecs::system::System) with an extra 
/// constraint that only one router can be responsible for writing to the [`Next`] 
/// value of a parameter component. Other changes to the parameter can be
/// [`deferred`](bevy_ecs::system::Deferred) to after the end of a step, but only one router
/// can be responsible for calculating the value. This relationship is used to schedule the routers
/// so that subsequent routers that depend on the output value can use it as an input. 
pub trait Router<M>: IntoSystem<(), (), M> + IntoSystemConfig<M> {

    /// Convenience for turning functions into [System](bevy_ecs::system::System)'s
    fn into_system(self) -> <Self as IntoSystem<(), (), M>>::System {
        IntoSystem::into_system(self)
    }
}

impl<M, F: IntoSystem<(), (), M> + IntoSystemConfig<M>> Router<M> for F {}

#[derive(Resource, Default)]
pub struct Nodes(HashMap<Entity, UID>);

impl Nodes {
    fn map<U, F: FnMut((Entity, UID))->U>(&self, mut f: F) -> HashMap::<Entity, U> {
        let mut result = HashMap::<Entity, U>::with_capacity(self.0.capacity());
        self.0.iter().for_each(|(id, uid)| {
            result.insert(*id, f((*id, *uid)));
        });
        result
    }
}
