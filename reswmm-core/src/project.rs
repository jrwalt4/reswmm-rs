
use crate::{
    element::{Element, UID},
    router::Router
};

use bevy_ecs::{
    component::ComponentId,
    prelude::*, 
    world::EntityMut
};

use std::collections::HashMap;

/// Container for elements and routers that makeup a simulation.
/// Functions like a [`App`](bevy_app::app::App).
/// 
/// # Examples
/// ```
/// # use bevy_ecs::prelude::*;
/// # use reswmm_core::project::Project;
/// #[derive(Component, Debug)]
/// struct Flow(f32);
/// 
/// fn dummy_router(mut flows: Query<(Entity, &Flow)>) {
///     for (id, Flow(q)) in &flows {
///         println!("Flow for element '{id:?}' = {q}");
///     }
/// }
/// 
/// let mut prj = Project::new();
/// prj.add_router(dummy_router);
/// ```
#[derive(Default)]
pub struct Project {
    model: World,
    schedule: Schedule,
    graph: HashMap<ComponentId, Vec<ComponentId>>,
}

impl Project {
    /// Create a new Project using [`default`](std::default::Default::default).
    pub fn new() -> Self {
        Default::default()
    }

    /// Add an element to the model. Elements are [bevy Entities](bevy_ecs::entity::Entity) with a 
    /// [`UID`] and [`Name`](crate::element::Name) component. 
    /// 
    /// # Examples
    /// ```
    /// # use reswmm_core::{element::Name, project::Project};
    /// # use bevy_ecs::prelude::*;
    /// let mut prj = Project::new();
    /// let el = prj.add_element(1, "J1", ());
    /// 
    /// fn read_one(elements: Query<(Entity, &Name)>) {
    ///     let (_id, Name(name)) = elements.single();
    ///     assert_eq!(name, "J1");
    /// }
    /// 
    /// prj.add_router(read_one).run();
    /// 
    /// ```
    pub fn add_element<I: Into<UID>, S:ToString, P: Bundle>(&mut self, uid: I, name: S, props: P) -> EntityMut {
        self.model.spawn((Element::new(uid, name), props))
    }

    /// Add a [`Router`] to the [`Project`].
    pub fn add_router<M, R: Router<M>>(&mut self, router: R) -> &mut Self {
        let mut router = router.into_system();
        {
            router.initialize(&mut self.model);
        }
        let access = router.component_access();
        for dep in access.reads() {
            self.graph.entry(dep)
                .and_modify(|e| { e.push(dep)})
                .or_insert(vec![dep]);
        }
        for id in access.writes() {
            use std::collections::hash_map::Entry::*;
            match self.graph.entry(id) {
                Occupied(_) => {
                    let dup = self.model.components().get_name(id).unwrap_or("{unknown}");
                    panic!("Multiple ownership for component {}", dup);
                },
                Vacant(e) => {
                    e.insert(Vec::new());
                }                
            }
        }
        self.schedule.add_system(router);
        self
    }

    /// Run this [Project].
    pub fn run(&mut self) {
        self.schedule.run(&mut self.model);
    }
}
