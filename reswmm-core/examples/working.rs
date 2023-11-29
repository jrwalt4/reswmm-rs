use reswmm_core::{element::*, project::Project, router::*, time::Clock};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Length(f32);

#[derive(Component, Clone, Debug)]
struct Depth(f32);

fn spawn_entities(mut commands: Commands) {
    commands.spawn_batch(vec![
        (Element::new(1, "J1"), Length(10.0), Depth(1.0)),
        (Element::new(2, "J2"), Length(5.0), Depth(4.0)),
    ]);
}

fn depth_router1(mut query: Query<(&Length, &Depth, &mut Next<Depth>)>, time: Res<Clock>) {
    eprintln!(
        "1 - Step: {:?}, Time: {:?}",
        time.step_count, time.simulation
    );
    for (Length(l), prev, mut next) in &mut query {
        eprintln!("1 - Length: {l}, Prev: {prev:?}, Next: {:?}", **next);
        **next = Depth(prev.0 + l * 0.1);
    }
}

fn depth_router2(query: Query<(&Length, &Depth, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!(
        "2 - Step: {:?}, Time: {:?}",
        time.step_count, time.simulation
    );
    for (Length(l), prev, next) in &query {
        eprintln!("2 - Length: {l}, Prev: {prev:?}, Next: {:?}", *next);
    }
}

fn main() {
    let mut prj = Project::new();
    prj.add_system(PreStartup, spawn_entities)
        .register_variable::<Depth>()
        .add_router(depth_router1)
        .add_router(depth_router2);
    //prj.add_router(depth_router2);
    prj.run_one();
    prj.run_one();
    //prj.run();
}
