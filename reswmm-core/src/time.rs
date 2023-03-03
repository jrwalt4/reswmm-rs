//! Time utilities and types

use chrono::Duration;

use std::ops::{Add, Sub};
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
pub struct Interval(pub Time, pub Time);

impl Interval {
    pub fn start(&self) -> Time {
        self.0
    }

    pub fn end(&self) -> Time {
        self.1
    }
}
