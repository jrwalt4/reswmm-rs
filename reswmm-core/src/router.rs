//! router

use hecs::{Query, QueryBorrow};

pub mod hydrology;
pub mod hydraulics;

pub trait Router<'a> {
    type SystemData: Query;

    fn run(&mut self, query: QueryBorrow<'a, Self::SystemData>);
}
