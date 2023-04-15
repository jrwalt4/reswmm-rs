//! router

pub mod hydrology;
pub mod hydraulics;

use crate::element::UID;

use std::collections::HashMap;

use bevy_ecs::prelude::*;

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
