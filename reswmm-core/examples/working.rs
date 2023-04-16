use reswmm_core::{
    project::Project,
    router::*
};

use bevy_ecs::prelude::*;

#[derive(Component)]
struct Length(f32);

#[derive(Component, Debug)]
struct Depth(f32);

fn depth_router(query: Query<(&Length, &Next<Depth>)>) {
    for (Length(l), depth) in &query {
        eprintln!("Length: {l}, Depth: {:?}", *depth);
    }
}

fn main() {
    let mut prj = Project::new();
    let e1 = prj.add_element(1, "J1", (Length(10.0), Param::new(Depth(1.0))));
    let e2 = prj.add_element(2, "J2", (Length(10.0), Param::new(Depth(1.0))));

    prj.add_router(depth_router);
    prj.run();
}
