//! router

pub mod hydrology;
pub mod hydraulics;

use crate::element::UID;

use std::{
    cell::{RefCell, Ref as CellRef},
    collections::HashMap,
    marker::PhantomData
};

use bevy_ecs::{
    archetype::{Archetype, ArchetypeComponentId},
    component::{ComponentId, Tick},
    query::{WorldQuery, QueryFetch, ReadOnlyWorldQuery, Access, FilteredAccess},
    prelude::*,
    system::IntoSystem,
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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct PreStepSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct StepSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct PostStepSet;

#[derive(Resource, Default)]
pub struct Nodes(HashMap<Entity, UID>);

impl Nodes {
    pub fn map<U, F: FnMut((Entity, UID))->U>(&self, mut f: F) -> HashMap::<Entity, U> {
        let mut result = HashMap::<Entity, U>::with_capacity(self.0.capacity());
        self.0.iter().for_each(|(id, uid)| {
            result.insert(*id, f((*id, *uid)));
        });
        result
    }
}

const PARAM_BUFFER_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Param<T>([RefCell<Option<T>>;PARAM_BUFFER_SIZE]);

impl<T: Component> Component for Param<T> {
    type Storage = <T as Component>::Storage;
}

/// SAFETY: The [`Param`] uses [`RefCell`]s, but exclusive access 
/// will be controlled through scheduling
unsafe impl<T: Send> Send for Param<T> {}
unsafe impl<T: Send + Sync> Sync for Param<T> {}

impl<T> Param<T> {
    pub fn new(value: T) -> Self {
        Param([RefCell::new(Some(value)), Default::default()])
    }
}

pub struct Next<T>(PhantomData<T>);

unsafe impl<T: Component> WorldQuery for &Next<T> {
    type Item<'a> = CellRef<'a, Option<T>>;

    type Fetch<'a> = QueryFetch<'a, &'a Param<T>>;

    type ReadOnly = Self;

    type State = (Tick, ComponentId);

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }

    unsafe fn init_fetch<'w>(
        world: &'w World,
        (_tick, component_id): &Self::State,
        last_change_tick: u32,
        change_tick: u32,
    ) -> Self::Fetch<'w> {
        <&'_ Param<T> as WorldQuery>::init_fetch(world, component_id, last_change_tick, change_tick)
    }

    unsafe fn clone_fetch<'w>(fetch: &Self::Fetch<'w>) -> Self::Fetch<'w> {
        <&'_ Param<T> as WorldQuery>::clone_fetch(fetch)
    }

    const IS_DENSE: bool = <&'_ Param<T> as WorldQuery>::IS_DENSE;

    const IS_ARCHETYPAL: bool = <&'_ Param<T> as WorldQuery>::IS_ARCHETYPAL;

    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        (_tick, component_id): &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ Param<T> as WorldQuery>::set_archetype(fetch, component_id, archetype, table);
    }

    unsafe fn set_table<'w>(
        fetch: &mut Self::Fetch<'w>, 
        (_tick, component_id): &Self::State, 
        table: &'w bevy_ecs::storage::Table
    ) {
        <&'_ Param<T> as WorldQuery>::set_table(fetch, component_id, table);
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        <&'_ Param<T> as WorldQuery>::fetch(fetch, entity, table_row).0[0].borrow()
    }

    fn update_component_access((_tick, component_id): &Self::State, access: &mut FilteredAccess<ComponentId>) {
        <&'_ Param<T> as WorldQuery>::update_component_access(component_id, access);
    }

    fn update_archetype_component_access(
        (_tick, component_id): &Self::State,
        archetype: &Archetype,
        access: &mut Access<ArchetypeComponentId>,
    ) {
        <&'_ Param<T> as WorldQuery>::update_archetype_component_access(component_id, archetype, access);
    }

    fn init_state(world: &mut World) -> Self::State {
        (Tick::new(world.change_tick()), <&'_ Param<T> as WorldQuery>::init_state(world))
    }

    fn matches_component_set(
        (_tick, component_id): &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <&'_ Param<T> as WorldQuery>::matches_component_set(component_id, set_contains_id)
    }
}

unsafe impl<T: Component> ReadOnlyWorldQuery for &Next<T> {}
