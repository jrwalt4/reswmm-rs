use reswmm_core::{
    project::Project,
    router::*,
    time::Clock
};

use bevy_ecs::prelude::*;

#[derive(Component)]
struct Length(f32);

#[derive(Component, Clone, Debug)]
struct Depth(f32);

fn depth_router1(mut query: Query<(&Length, &Depth, &mut Next<Depth>)>, time: Res<Clock>) {
    eprintln!("1 - Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), prev,  mut next) in &mut query {
        eprintln!("1 - Length: {l}, Prev: {prev:?}, Next: {:?}", **next);
        **next = Depth(prev.0 + l * 0.1);
    }
}

fn depth_router2(query: Query<(&Length, &Depth, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!("2 - Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), prev, next) in &query {
        eprintln!("2 - Length: {l}, Prev: {prev:?}, Next: {:?}", *next);
    }
}

fn main() {
    let mut prj = Project::new();
    prj.add_element(1, "J1", (Length(10.0), (Depth(1.0))));
    prj.add_element(2, "J2", (Length(10.0), (Depth(4.0))));
    prj.register_variable::<Depth>();

    prj.add_router(depth_router1).add_system(depth_router2.after(depth_router1));
    //prj.add_router(depth_router2);
    prj.run();
    prj.run();
    //prj.run();

}
