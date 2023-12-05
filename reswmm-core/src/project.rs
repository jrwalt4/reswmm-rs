use crate::{
    //element::{Element, UID},
    router::{
        variable_advance_system, variable_init_next, PostStepSet, PreStepSet,
        StepSet, Variable,
    },
    time::{clock_controller, Clock, ClockSettings, StepRequest},
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

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
pub struct Project {
    app: App,
    //graph: HashMap<ComponentId, Vec<ComponentId>>,
}

impl Project {
    /// Create a project with initialization for typical use
    pub fn new() -> Self {
        let mut app = App::default();
        app.configure_sets(
            Update,
            (
                PreStepSet,
                StepSet.after(PreStepSet),
                PostStepSet.after(StepSet),
            ),
        )
        .add_systems(PostUpdate, clock_controller)
        .init_resource::<Clock>()
        .init_resource::<ClockSettings>()
        .init_resource::<Events<StepRequest>>();
        Self { app }
    }

    pub fn register_variable<T: Variable>(&mut self) -> &mut Self {
        self.app.add_systems(
            PostStartup,
            IntoSystem::into_system(variable_init_next::<T>),
        );
        self.app.add_systems(
            PostUpdate,
            IntoSystem::into_system(variable_advance_system::<T>),
        );
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.app.init_resource::<R>();
        self
    }

    /// Add a [`Router`] to the [`Project`].
    pub fn add_router<M, R: IntoSystemConfigs<M>>(&mut self, router: R) -> &mut Self {
        self.app.add_systems(Update, router.in_set(StepSet));
        self
    }

    /// Add system directly, which can be added to any schedule
    /// (Startup, Update, PostUpdate, etc.).
    pub fn add_system<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        system: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.app.add_systems(schedule, system);
        self
    }

    /// Run this [`Project`].
    pub fn run_to_end(&mut self) {
        self.app.run();
    }

    pub fn run_one(&mut self) {
        self.app.update();
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
