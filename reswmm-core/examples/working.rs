use reswmm_core::{
    element::*,
    project::Project,
    router::*,
    time::Clock
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Invert(f32);

#[derive(Component)]
struct Link(Entity, Entity);

#[derive(Component)]
struct Diameter(f32);

#[derive(Component, Debug)]
struct Length(f32);

#[derive(Component)]
struct Slope(f32);

#[derive(Component, Clone, Debug)]
struct Depth(f32);

#[derive(Component, Clone, Debug)]
struct Inflow(f32);

#[derive(Component, Clone, Debug)]
struct Outflow(f32);

fn spawn_entities(mut commands: Commands) {
    let n1 = commands.spawn((Element::new(1, "J1"), Depth(1.0))).id();
    let n2 = commands.spawn((Element::new(2, "J2"), Depth(0.0))).id();
    let _l1 = commands.spawn((
        Element::new(3, "L1"),
        Length(100.0),
        Depth(0.0),
        Slope(0.0),
        Link(n1, n2),
        Diameter(2.0),
    ));
}

fn add_connections_system(mut graph: ResMut<NetworkGraph>, network: Query<(Entity, &Link)>) {
    for (link, Link(from, to)) in &network {
        graph.add_connection(*from, *to, link);
    }
}

fn calculate_slopes(graph: Res<NetworkGraph>, mut links: Query<(&Length, &mut Slope)>, nodes: Query<(Entity, &Invert)>) {
    for (node, invert) in &nodes {
        for (_from, to, link) in graph.links_out_of(node) {
            let &Length(length) = links.get_component::<Length>(*link).expect("Could not find Length");
            let mut slope = links.get_component_mut::<Slope>(*link).expect("Could not get mut Slope");
            let us_inv = invert;//nodes.get_component::<Invert>(from).expect("Could not get US Invert");
            let ds_inv = nodes.get_component::<Invert>(to).expect("Could not get DS Invert");
            *slope = Slope((ds_inv.0 - us_inv.0) / length);
        }
    }
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

fn depth_router2(network: Network<&Name, (&Length, &Depth, &Next<Depth>)>, time: Res<Clock>) {
    eprintln!(
        "2 - Step: {:?}, Time: {:?}",
        time.step_count, time.simulation
    );
    for item in &network {
        match item {
            NetworkIterItem::Node(name) => {
                eprintln!("2 - Solo node: {name:?}");
            }
            NetworkIterItem::Link(link, (node1, node2)) => {
                let link = link.unwrap();
                eprintln!("2 - Length: {length:?}, Prev: {prev:?}, Next: {next:?}\n\tNodes({node1:?}, {node2:?})", 
                    length = link.0,
                    prev = link.1,
                    next = link.2
                );
            }
        }
    }
}

fn main() {
    let mut prj = Project::new();
    prj.add_system(PreStartup, spawn_entities)
        .init_resource::<NetworkGraph>()
        .add_system(PostStartup, add_connections_system)
        .add_system(PostStartup, calculate_slopes)
        .register_variable::<Depth>()
        .add_router(depth_router1)
        .add_router(depth_router2);
    //prj.add_router(depth_router2);
    prj.run_one();
    prj.run_one();
    //prj.run();
}
