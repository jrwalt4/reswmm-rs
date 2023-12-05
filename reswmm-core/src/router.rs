//! router

pub mod hydraulics;
pub mod hydrology;
mod network;

pub use network::{Network, NetworkGraph, NetworkIterItem};

use bevy_ecs::{prelude::*, query::WorldQuery, schedule::ScheduleLabel};

use std::ops::{Deref, DerefMut};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, ScheduleLabel)]
pub struct InitSchedule;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct PreStepSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct StepSet;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemSet)]
pub struct PostStepSet;

/// A variable that will be solved for in a system.
///
/// In order to step from one timestep to another, the type must be able
/// to provide an initial [`Next`] value based on the previously calculed
/// value. The guess can be further refined, but a default is needed so
/// that the memory can be properly initialized.
pub trait Variable: Component + Clone {}

impl<T: Component + Clone> Variable for T {}

/// Wrapper around a `T` so that both `Prev` and `Next` values can be stored
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Next<T>(T);

impl<T> From<T> for Next<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Next<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Next<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Query the parameter value at the "next" time step
/// (the one being calculated).
#[derive(WorldQuery)]
pub struct NextRef<T: Variable> {
    next_ref: &'static Next<T>,
}

/// Query the parameter value at the "next" time step
/// (the one being calculated) for writing.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct NextMut<T: Variable> {
    next_mut: &'static mut Next<T>,
}

/// Query the parameter value at the "prev" time step
/// (the one previously calculated).
#[derive(WorldQuery)]
pub struct PrevRef<T: Variable> {
    prev_ref: &'static T,
}

pub(crate) fn variable_init_next<T: Variable>(vars: Query<(Entity, &T)>, mut commands: Commands) {
    eprintln!("Init variable {}", std::any::type_name::<T>());
    for (e, prev) in vars.iter() {
        commands.entity(e).insert(Next(prev.clone()));
    }
}

pub(crate) fn variable_advance_system<T: Variable>(mut vars: Query<(&mut T, &Next<T>)>) {
    eprintln!("Updating {}", std::any::type_name::<T>());
    for (mut prev, next) in &mut vars {
        // overwrite `prev`
        *prev = next.0.clone();
        // leave `next` as is for next step
    }
}
