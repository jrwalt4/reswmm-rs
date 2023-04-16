use reswmm_core::{
    project::Project,
    router::*,
    time::Clock
};

use bevy_ecs::prelude::*;

#[derive(Component)]
struct Length(f32);

#[derive(Component, Debug)]
struct Depth(f32);

fn depth_router1(query: Query<(&Length, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!("Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), depth) in &query {
        eprintln!("1 - Length: {l}, Depth: {:?}", *depth);
    }
}

fn depth_router2(query: Query<(&Length, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!("Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), depth) in &query {
        eprintln!("2 - Length: {l}, Depth: {:?}", *depth);
    }
}

fn main() {
    let mut prj = Project::new();
    let e1 = prj.add_element(1, "J1", (Length(10.0), Param::new(Depth(1.0))));
    let e2 = prj.add_element(2, "J2", (Length(10.0), Param::new(Depth(1.0))));

    prj.add_router(depth_router1).add_router(depth_router2);
    prj.run();
    prj.run();
    prj.run();
}
