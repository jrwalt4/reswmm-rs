/// Project container for nodes, links, regions, etc.
use crate::{
    router::*
};

use hecs::*;

use std::{
    any::TypeId,
    collections::HashMap
};

pub struct Project {
    pub(crate) prop_model: World,
    pub(crate) prev_model: World,
    pub(crate) next_model: World,
    resource_entity: Entity,
    routers: HashMap<TypeId, Box<dyn Router>>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            prop_model: World::new(),
            prev_model: World::new(),
            next_model: World::new(),
            resource_entity: Entity::DANGLING,
            routers: Default::default(),
        }
    }

    pub fn add_router<M, R: IntoRouter<M>>(&mut self, router: R) -> &mut Self {
        self.routers.insert(TypeId::of::<R>(), Box::new(IntoRouter::into_router(router, self)));
        self
    }

    pub fn add_element<P: Component>(&mut self, params: P) -> &mut Self {
        self.prev_model.spawn((params,));
        self
    }

    pub(crate) fn query_props<Q: Query + QueryShared>(&self) -> QueryBorrow<'_, Q> {
        self.prop_model.query()
    }

    pub(crate) fn query_prev<Q: Query + QueryShared>(&self) -> QueryBorrow<'_, Q> {
        self.prev_model.query()
    }

    pub(crate) fn query_next<Q: Query + QueryShared>(&self) -> QueryBorrow<'_, Q> {
        self.next_model.query()
    }

    pub(crate) fn query_next_mut<Q: Query>(&self) -> QueryBorrow<'_, Q> {
        self.next_model.query()
    }
}
