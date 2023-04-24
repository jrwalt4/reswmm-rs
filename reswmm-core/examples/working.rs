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

fn depth_router1(mut query: Query<(&Length, &mut Next<Depth>, &Prev<Depth>)>, time: Res<Clock>) {
    eprintln!("1 - Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), mut opt_depth, Depth(prev_depth)) in &mut query {
        eprintln!("1 - Length: {l}, Depth: {:?}", prev_depth);
        *opt_depth = Some(Depth(prev_depth + *l*0.1));
    }
}

fn depth_router2(query: Query<(&Length, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!("2 - Step: {:?}, Time: {:?}", time.step_count, time.simulation);
    for (Length(l), depth) in &query {
        eprintln!("2 - Length: {l}, Depth: {:?}", depth);
    }
}

fn main() {
    let mut prj = Project::new();
    prj.add_element(1, "J1", (Length(10.0), Param::initial(Depth(1.0))));
    prj.add_element(2, "J2", (Length(10.0), Param::initial(Depth(1.0))));

    // If depth_router2 were allowed to run concurrently with depth_router1 it would be UB. 
    // since depth_router1 has an exclusive ref to Next<Depth> but the WorldQuery impl marks it
    // as only a read access to allow concurrent access to the Prev<Depth>.
    // TODO: find a way to get the `System` out of an `IntoSystemConfig`.
    prj.add_router(depth_router1).add_system(depth_router2.after(depth_router1));
    prj.run();
    prj.run();
    prj.run();

}
