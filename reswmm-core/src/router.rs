//! router

pub mod hydrology;
pub mod hydraulics;

use crate::{element::UID, time::Clock};

use std::{
    collections::HashMap,
    marker::PhantomData
};

use bevy_ecs::{
    archetype::{Archetype, ArchetypeComponentId},
    component::ComponentId,
    query::{WorldQuery, QueryItem, QueryFetch, ReadOnlyWorldQuery, Access, FilteredAccess},
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
pub struct Param<T>([Option<T>;PARAM_BUFFER_SIZE]);

impl<T: Component> Component for Param<T> {
    type Storage = <T as Component>::Storage;
}

impl<T> Param<T> {
    pub fn initial(value: T) -> Self {
        Param([Some(value), None])
    }
}

/// Query the parameter value at the "next" time step 
/// (the one being calculated). For shared references,
/// this is the value calculated by another router.
/// for mutable references this is the "output" of the 
/// current router. No two routers can have mutable access 
/// to the same [`Next`] value. 
pub struct Next<T>(PhantomData<T>);

/// Query the parameter value at the "prev" time step 
/// (the one previously calculated).
pub struct Prev<T>(PhantomData<T>);

#[doc(hidden)]
pub struct ParamFetch<'w, T: Component> {
    index: usize,
    inner: QueryFetch<'w, &'w Param<T>>
}

pub struct ParamWriteFetch<'w, T: Component> {
    index: usize,
    inner: QueryFetch<'w, &'w mut Param<T>>
}

/// System to populate [`Param<T>`]s from an initial T component.
pub fn init_param_from_component<T: Component + Clone>(init_state: Query<(Entity, &T)>, mut commands: Commands) {
    for (id, val) in init_state.iter() {
        commands.entity(id).insert(Param::initial(Clone::clone(val)));
    }
}

unsafe impl<T: Component> WorldQuery for &Next<T> {
    type Item<'a> = QueryItem<'a, Option<&'a T>>;
    type Fetch<'a> = ParamFetch<'a, T>;
    type ReadOnly = Self;
    type State = ComponentId;
    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }
    unsafe fn init_fetch<'w>(
        world: &'w World,
        component_id: &Self::State,
        last_change_tick: u32,
        change_tick: u32,
    ) -> Self::Fetch<'w> {
        ParamFetch {
            index: (world.resource::<Clock>().step_count as usize + 1) % PARAM_BUFFER_SIZE,
            inner: <&'_ Param<T> as WorldQuery>::init_fetch(
                world,
                component_id,
                last_change_tick,
                change_tick,
            ),
        }
    }
    unsafe fn clone_fetch<'w>(ParamFetch { index, inner }: &Self::Fetch<'w>) -> Self::Fetch<'w> {
        ParamFetch {
            index: *index,
            inner: <&'_ Param<T> as WorldQuery>::clone_fetch(inner),
        }
    }
    const IS_DENSE: bool = <&'_ Param<T> as WorldQuery>::IS_DENSE;
    const IS_ARCHETYPAL: bool = <&'_ Param<T> as WorldQuery>::IS_ARCHETYPAL;
    unsafe fn set_archetype<'w>(
        ParamFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ Param<T> as WorldQuery>::set_archetype(inner, component_id, archetype, table);
    }
    unsafe fn set_table<'w>(
        ParamFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ Param<T> as WorldQuery>::set_table(inner, component_id, table);
    }
    unsafe fn fetch<'w>(
        ParamFetch { index, inner }: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        let param = <&'_ Param<T> as WorldQuery>::fetch(inner, entity, table_row);
        param.0[*index].as_ref()
    }
    fn update_component_access(
        component_id: &Self::State,
        access: &mut FilteredAccess<ComponentId>,
    ) {
        <&'_ Param<T> as WorldQuery>::update_component_access(component_id, access);
    }
    fn update_archetype_component_access(
        component_id: &Self::State,
        archetype: &Archetype,
        access: &mut Access<ArchetypeComponentId>,
    ) {
        <&'_ Param<T> as WorldQuery>::update_archetype_component_access(
            component_id,
            archetype,
            access,
        );
    }
    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<Clock>();
        <&'_ Param<T> as WorldQuery>::init_state(world)
    }
    fn matches_component_set(
        component_id: &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <&'_ Param<T> as WorldQuery>::matches_component_set(component_id, set_contains_id)
    }
}

unsafe impl<T: Component> ReadOnlyWorldQuery for &Next<T> {}

unsafe impl<'__w, T: Component> WorldQuery for &'__w mut Next<T> {
    type Item<'a> = Mut<'a,  Option<T>>;
    type Fetch<'a> = ParamWriteFetch<'a, T>;
    type ReadOnly = &'__w Next<T>;
    type State = ComponentId;
    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }

    unsafe fn init_fetch<'w>(
        world: &'w World,
        component_id: &Self::State,
        last_change_tick: u32,
        change_tick: u32,
    ) -> Self::Fetch<'w> {
        ParamWriteFetch {
            index: (world.resource::<Clock>().step_count as usize + 1) % PARAM_BUFFER_SIZE,
            inner: <&'_ mut Param<T> as WorldQuery>::init_fetch(
                world,
                component_id,
                last_change_tick,
                change_tick,
            ),
        }
    }

    unsafe fn clone_fetch<'w>(ParamWriteFetch { index, inner }: &Self::Fetch<'w>) -> Self::Fetch<'w> {
        ParamWriteFetch {
            index: *index,
            inner: <&'_ mut Param<T> as WorldQuery>::clone_fetch(inner),
        }
    }

    const IS_DENSE: bool = <&'_ Param<T> as WorldQuery>::IS_DENSE;
    const IS_ARCHETYPAL: bool = <&'_ Param<T> as WorldQuery>::IS_ARCHETYPAL;
    unsafe fn set_archetype<'w>(
        ParamWriteFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ mut Param<T> as WorldQuery>::set_archetype(inner, component_id, archetype, table);
    }

    unsafe fn set_table<'w>(
        ParamWriteFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ mut Param<T> as WorldQuery>::set_table(inner, component_id, table);
    }

    unsafe fn fetch<'w>(
        ParamWriteFetch { index, inner }: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        <&'_ mut Param<T> as WorldQuery>::fetch(inner, entity, table_row)
            .map_unchanged(|param_mut| &mut param_mut.0[*index] )
    }

    fn update_component_access(
        component_id: &Self::State,
        access: &mut FilteredAccess<ComponentId>,
    ) {
        // SAFETY; We're pretending we aren't writing to this component to allow
        // multiple access to the [`Prev`] portion of the component
        <&'_ Param<T> as WorldQuery>::update_component_access(component_id, access);
    }

    fn update_archetype_component_access(
        component_id: &Self::State,
        archetype: &Archetype,
        access: &mut Access<ArchetypeComponentId>,
    ) {
        // SAFETY; We're pretending we aren't writing to this component to allow
        // multiple access to the [`Prev`] portion of the component
        <&'_ Param<T> as WorldQuery>::update_archetype_component_access(
            component_id,
            archetype,
            access,
        );
    }

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<Clock>();
        <&'_ mut Param<T> as WorldQuery>::init_state(world)
    }

    fn matches_component_set(
        component_id: &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <&'_ Param<T> as WorldQuery>::matches_component_set(component_id, set_contains_id)
    }
}

unsafe impl<T: Component> WorldQuery for &Prev<T> {
    type Item<'a> = QueryItem<'a, &'a T>;
    type Fetch<'a> = ParamFetch<'a, T>;
    type ReadOnly = Self;
    type State = ComponentId;
    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }
    unsafe fn init_fetch<'w>(
        world: &'w World,
        component_id: &Self::State,
        last_change_tick: u32,
        change_tick: u32,
    ) -> Self::Fetch<'w> {
        ParamFetch {
            index: world.resource::<Clock>().step_count as usize % PARAM_BUFFER_SIZE,
            inner: <&'_ Param<T> as WorldQuery>::init_fetch(
                world,
                component_id,
                last_change_tick,
                change_tick,
            ),
        }
    }
    unsafe fn clone_fetch<'w>(ParamFetch { index, inner }: &Self::Fetch<'w>) -> Self::Fetch<'w> {
        ParamFetch {
            index: *index,
            inner: <&'_ Param<T> as WorldQuery>::clone_fetch(inner),
        }
    }
    const IS_DENSE: bool = <&'_ Param<T> as WorldQuery>::IS_DENSE;
    const IS_ARCHETYPAL: bool = <&'_ Param<T> as WorldQuery>::IS_ARCHETYPAL;
    unsafe fn set_archetype<'w>(
        ParamFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ Param<T> as WorldQuery>::set_archetype(inner, component_id, archetype, table);
    }
    unsafe fn set_table<'w>(
        ParamFetch { inner, .. }: &mut Self::Fetch<'w>,
        component_id: &Self::State,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <&'_ Param<T> as WorldQuery>::set_table(inner, component_id, table);
    }
    unsafe fn fetch<'w>(
        ParamFetch { index, inner }: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        let param = <&'_ Param<T> as WorldQuery>::fetch(inner, entity, table_row);
        param.0[*index].as_ref().unwrap()
    }
    fn update_component_access(
        component_id: &Self::State,
        access: &mut FilteredAccess<ComponentId>,
    ) {
        <&'_ Param<T> as WorldQuery>::update_component_access(component_id, access);
    }
    fn update_archetype_component_access(
        component_id: &Self::State,
        archetype: &Archetype,
        access: &mut Access<ArchetypeComponentId>,
    ) {
        <&'_ Param<T> as WorldQuery>::update_archetype_component_access(
            component_id,
            archetype,
            access,
        );
    }
    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<Clock>();
        <&'_ Param<T> as WorldQuery>::init_state(world)
    }
    fn matches_component_set(
        component_id: &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <&'_ Param<T> as WorldQuery>::matches_component_set(component_id, set_contains_id)
    }
}

unsafe impl<T: Component> ReadOnlyWorldQuery for &Prev<T> {}
