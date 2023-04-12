//! router
//! 
//! This is based on bevy_ecs System and SystemParam

pub mod hydrology;
pub mod hydraulics;

use crate::project::Project;

use hecs::*;

use std::marker::PhantomData;

pub trait Router: 'static {
    fn execute(&mut self, project: &Project);
}

pub trait RouterParam: Sized {
    type State;//: Send + Sync + 'static;
    type Item<'p, 's>: RouterParam<State = Self::State> where 'p: 's;
    fn get_param<'p:'s, 's>(project: &'p Project, state: &'s mut Self::State) -> Self::Item<'p, 's>;
    fn init_state(prj: &Project) -> Self::State;
}

pub type RouterParamItem<'p, 's, P> = <P as RouterParam>::Item<'p, 's>;

pub trait RouterParamFunction<Marker>: Send + Sync + 'static {
    type Param: RouterParam;
    fn call(&mut self, param: RouterParamItem<Self::Param>  );
}

pub struct FunctionRouter<Func, Marker>
where
    Func: RouterParamFunction<Marker>
{
    router_fn: Func,
    state: <Func::Param as RouterParam>::State,
    // no idea why this works...
    marker: PhantomData<fn() -> Marker>
}

impl<Func, Marker> Router for FunctionRouter<Func, Marker>
where
    Marker: 'static,
    Func: RouterParamFunction<Marker> {
    fn execute(&mut self, project: &Project) {
        let params = Func::Param::get_param(project, &mut self.state);
        self.router_fn.call(params);
    }
}

pub trait IntoRouter<Marker>: Sized + 'static {
    type Into: Router;
    fn into_router(this: Self, prj: &Project) -> Self::Into;
}

/// IntoRouter is reflexive
impl<R: Router> IntoRouter<()> for R {
    type Into = Self;
    fn into_router(this: Self, _prj: &Project) -> Self::Into {
       this 
    }
}

pub struct FunctionRouterMarker;

impl<Func, Marker> IntoRouter<(FunctionRouterMarker, Marker)> for Func
where
    Marker: 'static, 
    Func: RouterParamFunction<Marker>
{
    type Into = FunctionRouter<Self, Marker>;
    fn into_router(this: Self, prj: &Project) -> Self::Into {
        FunctionRouter {router_fn: this, state: Func::Param::init_state(prj), marker: PhantomData }
    }
}

pub struct Prop<'q, Q: Query + QueryShared>(PreparedQueryBorrow<'q, Q>);

impl<Q: Query + QueryShared + Send + Sync + 'static> RouterParam for Prop<'_, Q> {
    type State = PreparedQuery<Q>;
    type Item<'p, 's> = Prop<'s, Q> where 'p:'s;
    fn get_param<'p: 's, 's>(project: &'p Project, state: &'s mut Self::State) -> Self::Item<'p, 's> {
        Prop(state.query(&project.prop_model))
    }
    fn init_state(_prj: &Project) -> Self::State {
        PreparedQuery::new()
    }
}

pub struct Prev<'q, Q: Query + QueryShared>(PreparedQueryBorrow<'q, Q>);

impl<Q: Query + QueryShared> Prev<'_, Q> {
    pub fn iter(&mut self) -> impl Iterator<Item = (Entity, <Q as Query>::Item<'_>)> {
        self.0.iter()
    }
}

impl<Q: Query + QueryShared + Send + Sync + 'static> RouterParam for Prev<'_, Q> {
    type State = PreparedQuery<Q>;
    type Item<'p, 's> = Prev<'s, Q> where 'p:'s;
    fn get_param<'p: 's, 's>(project: &'p Project, state: &'s mut Self::State) -> Self::Item<'p, 's> {
        Prev(state.query(&project.prev_model))
    }
    fn init_state(_prj: &Project) -> Self::State {
        PreparedQuery::new()
    }
}

pub struct Next<'q, Q: Query + QueryShared>(PreparedQueryBorrow<'q, Q>);

impl<Q: Query + QueryShared + Send + Sync + 'static> RouterParam for Next<'_, Q> {
    type State = PreparedQuery<Q>;
    type Item<'p, 's> = Next<'s, Q> where 'p:'s;
    fn get_param<'p:'s, 's>(project: &'p Project, state: &'s mut Self::State) -> Self::Item<'p, 's> {
        Next(state.query(&project.next_model))
    }
    fn init_state(_prj: &Project) -> Self::State {
        PreparedQuery::new()
    }
}

pub struct NextMut<'q, Q: Query>(PreparedQueryBorrow<'q, Q>);

impl<Q: Query + Send + Sync + 'static> RouterParam for NextMut<'_, Q> {
    type State = PreparedQuery<Q>;
    type Item<'p, 's> = NextMut<'s, Q> where 'p:'s;
    fn get_param<'p:'s, 's>(project: &'p Project, state: &'s mut Self::State) -> Self::Item<'p, 's> {
        NextMut(state.query(&project.next_model))
    }
    fn init_state(_prj: &Project) -> Self::State {
        PreparedQuery::new()
    }
}
