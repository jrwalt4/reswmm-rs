
use crate::{
    element::{Element, UID},
    router::{InitSchedule, PreStepSet, StepSet, PostStepSet, Variable, variable_advance_system, variable_init_next},
    time::{Clock, ClockSettings, StepRequest, clock_controller}
};

use bevy_ecs::{
    prelude::*,
    world::EntityMut
};

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
    //graph: HashMap<ComponentId, Vec<ComponentId>>,
    init_schedule: Option<Schedule>,
    step_schedule: Schedule,
}

impl Project {
    /// Create a project with initialization for typical use
    pub fn new() -> Self {
        let mut prj = Self::default();
        prj.schedule
            .configure_sets(PreStepSet)
            .configure_sets(StepSet.after(PreStepSet))
            .configure_sets(PostStepSet.after(StepSet))
            .add_systems(clock_controller.in_set(PostStepSet));
        prj.model.init_resource::<Clock>();
        prj.model.init_resource::<ClockSettings>();
        prj.model.init_resource::<Events<StepRequest>>();
        prj
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
        self.model.spawn((Element::new(uid, name), props)).into()
    }

    pub fn register_variable<T: Variable>(&mut self) -> &mut Self {

        self.init_schedule.get_or_insert_with(|| Schedule::new(InitSchedule)).add_systems(IntoSystem::into_system(variable_init_next::<T>));
        self.step_schedule.add_systems(IntoSystem::into_system(variable_advance_system::<T>));
        self
    }

    /// Add a [`Router`] to the [`Project`].
    pub fn add_router<M, R: IntoSystem<(), (), M>>(&mut self, router: R) -> &mut Self {
        /* 
        let mut router = IntoSystem::into_system(router);
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
        // */
        self.schedule.add_systems(router.in_set(StepSet));
        self
    }

    pub fn add_system<M, S: IntoSystemConfigs<M>>(&mut self, system: S) -> &mut Self {
        self.schedule.add_systems(system);
        self
    }

    /// Run this [Project].
    pub fn run(&mut self) {
        // only run the init_schedule once, leave None in its place. 
        if let Some(mut init) = self.init_schedule.take() {
            self.schedule.initialize(&mut self.model).unwrap();
            init.run(&mut self.model);
            init.apply_deferred(&mut self.model);
        }
        self.schedule.run(&mut self.model);
        self.step_schedule.run(&mut self.model);
    }
}
