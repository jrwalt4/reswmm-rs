//! World datastructure to hold entities, components, and resources.

use std::collections::HashMap;

use crate::{
    component::{Archetype, ArchetypeId},
    entity::{EntityId, EntityManager}
};

#[derive(Default)]
pub struct World {
    entities: EntityManager,
    entity_index: HashMap<EntityId, ArchetypeId>,
    archetypes: HashMap<ArchetypeId, Archetype>,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}
