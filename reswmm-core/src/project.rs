/// Project container for nodes, links, regions, etc.

use hecs::World;

pub struct Project {
    model: World
}

impl Project {
    pub fn new() -> Self {
        Project { model: World::new() }
    }
}
