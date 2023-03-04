//! Time utilities and types

pub use chrono::Duration;

use std::ops::{Add, Sub, AddAssign};
use std::time::Duration as StdDuration;

/// Simulation Time
/// 
/// Number of milliseconds since the beginning of a simulation. 
#[derive(Debug, Clone, Copy)]
pub struct Time(u64);

impl Add<StdDuration> for Time {
    type Output = Self;
    fn add(self, rhs: StdDuration) -> Self::Output {
        Time(self.0 + rhs.as_millis() as u64)
    }
}

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        Time(self.0 + rhs.num_milliseconds() as u64)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.num_milliseconds() as u64;
    }
}

impl Sub<StdDuration> for Time {
    type Output = Self;
    fn sub(self, rhs: StdDuration) -> Self::Output {
        Time(self.0 - rhs.as_millis() as u64)
    }
}


impl Sub<Duration> for Time {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self::Output {
        assert!(rhs.num_milliseconds().ge(&0));
        assert!(self.0 > rhs.num_milliseconds() as u64);
        Time(self.0 - rhs.num_milliseconds() as u64)
    }
}

/// Interval of (start, end)
#[derive(Debug, Copy, Clone)]
pub struct Interval(pub Time, pub Time);

impl Interval {
    pub fn start(&self) -> Time {
        self.0
    }

    pub fn end(&self) -> Time {
        self.1
    }

    pub fn range(&self) -> (Time, Time) {
        (self.0, self.1)
    }

    pub fn advance(self, by: Duration) -> Self {
        Interval(self.1, self.1 + by)
    }
}
