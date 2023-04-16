//! Time utilities and types

use bevy_ecs::prelude::*;
pub use chrono::{Duration, NaiveDateTime};

use std::ops::{Add, Sub, AddAssign};
use std::time::Duration as StdDuration;

/// Simulation Time
/// 
/// Number of milliseconds since the beginning of a simulation. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u64);

impl Time {
    pub fn zero() -> Self {
        Self(0)
    }
}

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

#[derive(Resource)]
pub struct Clock {
    pub simulation: Duration,
    pub calendar: NaiveDateTime,
    pub step_count: u64,
    end: Option<NaiveDateTime>,
}

impl Clock {
    pub fn with_start(start: NaiveDateTime) -> Self {
        Self {
            simulation: Duration::zero(),
            calendar: start,
            end: None,
            step_count: 0
        }
    }

    pub fn with_start_and_end(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self {
            end: Some(end),
            ..Self::with_start(start)
        }
    }

    pub fn with_start_and_duration(start: NaiveDateTime, duration: Duration) -> Self {
        Self {
            end: Some(start + duration),
            ..Self::with_start(start)
        }
    }

    pub(crate) fn advance(&mut self, by: Duration) {
        let by = match self.end {
            Some(end) if end > self.calendar + by => end - self.calendar,
            _ => by,
        };
        self.simulation = self.simulation + by;
        self.calendar += by;
        self.step_count += 1;
    }
}

impl Default for Clock {
    fn default() -> Self {
        use std::time::SystemTime;

        Clock::with_start(
            NaiveDateTime::default()
                + Duration::from_std(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default(),
                )
                .unwrap_or(Duration::zero()),
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct StepRequest(Duration);

impl StepRequest {
    pub fn seconds(seconds: i64) -> Self {
        Self(Duration::seconds(seconds))
    }

    pub fn minutes(minutes: i64) -> Self {
        Self(Duration::minutes(minutes))
    }

    pub fn hours(hours: i64) -> Self {
        Self(Duration::hours(hours))
    }
}

#[derive(Resource)]
pub struct ClockSettings {
    min_step: Duration,
    max_step: Duration,
}

impl Default for ClockSettings {
    fn default() -> Self {
        ClockSettings {
            min_step: Duration::seconds(30),
            max_step: Duration::minutes(60),
        }
    }
}

/// System to control advancing the simulation clock
/// 
/// Clock advances either by the default amount in the [`ClockSettings`] or
/// by requesting a specific step by sending a [`StepRequest`] event.
pub(crate) fn clock_controller(
    settings: Res<ClockSettings>,
    mut clock: ResMut<Clock>,
    mut requests: EventReader<StepRequest>,
) {
    let next_step = requests
        .iter()
        .map(|StepRequest(step)| step)
        .min()
        .copied()
        .clamp(Some(settings.min_step), Some(settings.max_step))
        .unwrap();
    clock.advance(next_step);
}
