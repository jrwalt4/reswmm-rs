//! A Result that can represent both:
//!  * a successful calculation
//!  * a calculation with a warning about accuracy or suggestions for the user
//!  * an error preventing further computations

pub use std::result::Result as StdResult;

pub enum Warning {
    MaxIterations,
    MinTimeStep,
    MinSlope,
    MinOffset,
    Custom(String)
}

#[derive(Debug)]
pub enum Error {
    Input(String),
    InvalidInput,
    MaxIterations,
    DivZero,
    Router,
    Custom(String)
}

pub type Result<T> = StdResult<T, Error>;
