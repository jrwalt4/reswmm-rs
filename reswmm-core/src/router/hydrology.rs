use super::Router;

use hecs::QueryBorrow;

#[derive(Debug)]
pub struct ExtInflow(f32);

pub struct InflowRouter;

impl<'a> Router<'a> for InflowRouter {
    type SystemData = (&'a ExtInflow, );

    fn run(&mut self, mut query: QueryBorrow<'a, Self::SystemData>) {
        for (_id, _q) in query.iter() {
            
        }
    }

    
}
