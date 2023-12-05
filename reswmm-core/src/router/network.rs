//! Utilities for traversing through an hydraulic system

use bevy_ecs::{
    prelude::*,
    query::{ReadOnlyWorldQuery, WorldQuery},
    system::SystemParam,
};

use petgraph::{
    graphmap::*,
    prelude::*,
    visit::{Topo, Visitable, Walker, WalkerIter},
};

use std::iter::FusedIterator;

/// NetworkGraph Resource to hold connections between nodes and links.
/// This is just a facade over a Directed [`GraphMap`].
#[derive(Resource, Default)]
pub struct NetworkGraph {
    graph: DiGraphMap<Entity, Entity>,
}

impl NetworkGraph {
    pub fn add_connection(&mut self, us_node: Entity, ds_node: Entity, link: Entity) -> &mut Self {
        self.graph.add_edge(us_node, ds_node, link);
        self
    }

    pub fn all_links(&self) -> AllEdges<'_, Entity, Entity, Directed> {
        self.graph.all_edges()
    }

    pub fn links_out_of(&self, node: Entity) -> EdgesDirected<'_, Entity, Entity, Directed> {
        self.graph.edges_directed(node, Direction::Outgoing)
    }

    pub fn iter_topo(&self) -> NetworkWalker<'_> {
        Topo::new(&self.graph).iter(&self.graph)
    }
}

pub type NetworkWalker<'w> = WalkerIter<
    Topo<Entity, <DiGraphMap<Entity, Entity> as Visitable>::Map>,
    &'w DiGraphMap<Entity, Entity>,
>;

#[derive(SystemParam)]
pub struct Network<'w, 's, QNode, QLink, FNode = (), FLink = ()>
where
    QNode: WorldQuery + 'static,
    QLink: WorldQuery + 'static,
    FNode: ReadOnlyWorldQuery + 'static,
    FLink: ReadOnlyWorldQuery + 'static,
{
    node_query: Query<'w, 's, QNode, FNode>,
    link_query: Query<'w, 's, QLink, FLink>,
    graph: Res<'w, NetworkGraph>,
}

impl<'w, 's: 'w, QNode, QLink, FNode, FLink> IntoIterator
    for &'s Network<'w, 's, QNode, QLink, FNode, FLink>
where
    QNode: WorldQuery + 'static,
    QLink: WorldQuery + 'static,
    FNode: ReadOnlyWorldQuery + 'static,
    FLink: ReadOnlyWorldQuery + 'static,
{
    type IntoIter = NetworkIter<'w, 's, QNode, QLink, FNode, FLink>;

    type Item = <NetworkIter<'w, 's, QNode, QLink, FNode, FLink> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        NetworkIter {
            entities: NetworkEntityIter::new(&self.graph),
            node_query: &self.node_query,
            link_query: &self.link_query,
        }
    }
}

pub struct NetworkEntityIter<'w> {
    walker: NetworkWalker<'w>,
    current_node: Option<Entity>,
    edge_iter: Option<Edges<'w, Entity, Entity, Directed>>,
    //current_edge: Option<(Entity, Entity, &'w Entity)>,
}

impl<'a> NetworkEntityIter<'a> {
    pub fn new(graph: &'a NetworkGraph) -> Self {
        let mut walker = graph.iter_topo();
        // start the process so when `current_node` is None we know we're finished.
        let current_node = walker.next();
        Self {
            walker,
            current_node,
            edge_iter: None,
            //current_edge: None,
        }
    }
}

impl Iterator for NetworkEntityIter<'_> {
    type Item = NetworkIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        use NetworkIterItem::*;
        loop {
            if let Some(node) = self.current_node {
                match &mut self.edge_iter {
                    None => {
                        // firts check if iter is empty, meaning it's a standalone node with no edges.
                        let iter = self.edge_iter.insert(self.walker.context().edges(node));
                        match iter.next() {
                            Some((us_node, ds_node, &link)) => {
                                // iter is NOT empty, return the first item.
                                // already set `self.edge_iter` with `insert` above.
                                return Some(Link(Ok(link), (Ok(us_node), Ok(ds_node))));
                            }
                            None => {
                                // iter WAS emtpy, advance walker and return a `Node`.
                                // This shouldn't be common in a normal simulation.
                                self.edge_iter.take();
                                self.current_node = self.walker.next();
                                return Some(Node(Ok(node)));
                            }
                        }
                    }
                    Some(edges) => {
                        match edges.next() {
                            Some((us_node, ds_node, &link)) => {
                                // we know this is a link because if there were no edges
                                // we would have caught it above.
                                return Some(Link(Ok(link), (Ok(us_node), Ok(ds_node))));
                            }
                            None => {
                                // iter is done, clear it out
                                self.edge_iter.take();
                                // setup for next node
                                self.current_node = self.walker.next();
                            }
                        }
                    }
                }
            } else {
                // the current_node was set in constructor and managed above.
                // if it's empty, we're done and will alway return done.
                return None;
            }
        }
    }
}

impl FusedIterator for NetworkEntityIter<'_> {}

pub struct NetworkIter<'w, 's, QNode, QLink, FNode, FLink>
where
    QNode: WorldQuery + 'static,
    QLink: WorldQuery + 'static,
    FNode: ReadOnlyWorldQuery + 'static,
    FLink: ReadOnlyWorldQuery + 'static,
{
    entities: NetworkEntityIter<'w>,
    node_query: &'s Query<'w, 's, QNode, FNode>,
    link_query: &'s Query<'w, 's, QLink, FLink>,
}

impl<'w, 's, QNode, QLink, FNode, FLink> Iterator
    for NetworkIter<'w, 's, QNode, QLink, FNode, FLink>
where
    QNode: WorldQuery + 'static,
    //for <'a> <QNode::ReadOnly as WorldQuery>::Item<'a>: WorldQuery,
    QLink: WorldQuery + 'static,
    //for <'a> <QLink::ReadOnly as WorldQuery>::Item<'a>: WorldQuery,
    FNode: ReadOnlyWorldQuery + 'static,
    FLink: ReadOnlyWorldQuery + 'static,
{
    type Item = NetworkIterItem<
        <QNode::ReadOnly as WorldQuery>::Item<'s>,
        <QLink::ReadOnly as WorldQuery>::Item<'s>,
    >;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.entities.next().map(|item| {
            item.map(
                |node| self.node_query.get(node).map_err(|_| node),
                |link| self.link_query.get(link).map_err(|_| link),
            )
        })
    }
}

pub type NetworkQueryResult<Q> = Result<Q, Entity>;

pub enum NetworkIterItem<NodeItem = Entity, LinkItem = Entity> {
    Node(NetworkQueryResult<NodeItem>),
    Link(
        NetworkQueryResult<LinkItem>,
        (NetworkQueryResult<NodeItem>, NetworkQueryResult<NodeItem>),
    ),
}

impl<N, L> NetworkIterItem<N, L> {
    pub fn map<FNode, FLink, UNode, ULink>(
        self,
        mut f_node: FNode,
        f_link: FLink,
    ) -> NetworkIterItem<UNode, ULink>
    where
        FNode: FnMut(N) -> NetworkQueryResult<UNode>,
        FLink: FnOnce(L) -> NetworkQueryResult<ULink>,
    {
        use NetworkIterItem::*;
        match self {
            Node(node) => Node(node.and_then(f_node)),
            Link(link, (usnode, dsnode)) => Link(
                link.and_then(f_link),
                (usnode.and_then(&mut f_node), dsnode.and_then(&mut f_node)),
            ),
        }
    }
}
