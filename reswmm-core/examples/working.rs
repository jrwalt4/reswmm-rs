use reswmm_core::{element::*, project::Project, router::*, time::Clock};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use petgraph::{
    graphmap::*,
    prelude::*
};

#[derive(Component)]
struct Invert(f32);

#[derive(Component)]
struct Link(Entity, Entity);

// #[derive(Component)]
// struct UpstreamElement(Entity);

#[derive(Component)]
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
    let _l1 = commands.spawn((Element::new(3, "L1"), Length(100.0), Depth(0.0), Slope(0.0), Link(n1, n2)));
}

fn calculate_slopes(graph: Res<NetworkGraph>, mut links: Query<(&Length, &mut Slope)>, nodes: Query<(Entity, &Invert)>) {
    for (node, invert) in &nodes {
        for (from, to, link) in graph.links_from(node) {
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
        .init_resource::<NetworkGraph>()
        .add_system(PostStartup, NetworkGraph::add_connections_system)
        .add_system(PostStartup, calculate_slopes)
        .register_variable::<Depth>()
        .add_router(depth_router1)
        .add_router(depth_router2);
    //prj.add_router(depth_router2);
    prj.run_one();
    prj.run_one();
    //prj.run();
}

#[derive(Resource, Default)]
pub struct NetworkGraph {
    graph: DiGraphMap<Entity, Entity>,
}

impl NetworkGraph {
    pub fn add_connection(&mut self, us_node: Entity, ds_node: Entity, link: Entity) -> &mut Self {
        self.graph.add_edge(us_node, ds_node, link);
        self
    }

    pub fn add_connections_system(mut graph: ResMut<Self>, network: Query<(Entity, &Link)>) {
        for (link, Link(from, to)) in &network {
            graph.add_connection(*from, *to, link);
        }
    }

    pub fn all_links(&self) -> AllEdges<'_, Entity, Entity, Directed> {
        self.graph.all_edges()
    }

    pub fn links_from(&self, node: Entity) -> EdgesDirected<'_, Entity, Entity, Directed> {
        self.graph.edges_directed(node, Direction::Outgoing)
    }
}
