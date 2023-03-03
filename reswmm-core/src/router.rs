//! router

use crate::element::UID;
use crate::error::{Result, Error};

use chrono::{NaiveDateTime, Duration};
use crossbeam_channel::{unbounded, Receiver, Sender};
use itertools::Itertools;
use rusqlite as rs;

use std::collections::{HashMap, BTreeMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

pub struct RouterHandle<R: Router> {
    pub ruid: UID,
    router: R,
    timestamp: Duration,

    receiver: Receiver<Arc<RouterStep<R::Output>>>,
    current: Option<Arc<RouterStep<R::Output>>>,
    next: Option<Arc<RouterStep<R::Output>>>
}

impl<R: Router> RouterHandle<R> {
    pub fn simulation_time(&self) -> Duration {
        self.timestamp
    }

    pub fn calendar_time(&self) -> NaiveDateTime {
        __private::get_project().project_start + self.timestamp
    }

    pub fn advance(&mut self, by: Duration) -> Result<Duration> {
        match &self.current {
            Some(current) => {

                match &self.next {
                    Some(_next) => {
                        self.current = self.next.take();
                        self.next = self.receiver.recv().ok();
                    },
                    None => {

                    }
                }
            },
            None => {
                assert!(by.is_zero());

                self.current = self.receiver.recv().ok();
            }
        }
        Ok(by)
    }

    fn solve(&self, prev: &RouterStep<R::Output>, next: &mut RouterStep<R::Output>) -> Result<Solution> {
        self.router.solve(prev, next)
    }

    fn interpolate(&self, a: &RouterStep<R::Output>, b: &RouterStep<R::Output>, c: &mut RouterStep<R::Output>) -> Result<()> {
        self.router.interpolate(a, b, c)
    }
}

pub struct RouterStep<T> {
    timestamp: Duration,
    data: HashMap<UID, T>
}

impl<T> RouterStep<T> {
    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }
}

pub struct StateWriter {
    pub timestamp: Duration
}

pub struct Solution {
    pub step: Duration
}

pub trait Router: Send {
    type Output;

    fn solve(&self, prev: &RouterStep<Self::Output>, next: &mut RouterStep<Self::Output>) -> Result<Solution>;

    fn interpolate(&self, a: &RouterStep<Self::Output>, b: &RouterStep<Self::Output>, c: &mut RouterStep<Self::Output>) -> Result<()>;
}

pub struct ScenarioRouter<Ho, Ha, Ql> {
    hydrology: Ho,
    hydraulics: Ha,
    quality: Ql
}

impl<Ho, Ha, Ql> ScenarioRouter<Ho, Ha, Ql> {
    
}

mod __private {
    use chrono::NaiveDateTime;

    use std::sync::Once;

    static mut PRJ_ROUTER: ProjectRouter = ProjectRouter::const_new();
    static PRJ_INIT: Once = Once::new();

    pub fn init_project(start: NaiveDateTime) -> Result<(), String> {
        if PRJ_INIT.is_completed() {
            return Err("Project already initialized".to_string());
        }
        unsafe {
            PRJ_INIT.call_once(|| {
                PRJ_ROUTER = ProjectRouter {
                    project_start: start
                };
            });
        }
        Ok(())
    }

    pub(crate) fn get_project() -> &'static ProjectRouter {
        assert!(PRJ_INIT.is_completed());
        unsafe {&PRJ_ROUTER}
    }

    pub(crate) struct ProjectRouter {
        pub project_start: NaiveDateTime
    }

    impl ProjectRouter {
        const fn const_new() -> Self {
            Self {
                project_start: NaiveDateTime::MIN
            }
        }
    }
}
