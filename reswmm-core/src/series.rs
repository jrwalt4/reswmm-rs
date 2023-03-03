//! Series of values (flowrate, depth, rainfall, etc.)

use chrono::Duration;

pub struct Series<T> {
    values: Vec<SeriesItem<T>>
}

impl<T> Series<T> {
    pub fn new() -> Self {
        Series {
            values: Vec::new()
        }
    }

    pub fn push<U: Into<SeriesItem<T>>>(&mut self, item: U) -> &mut Self {
        self.values.push(item.into());
        self
    }

    pub fn push_after(&mut self, duration: Duration, value: T) -> &mut Self {
        let prev = self.values.last().map(|item| item.timestamp).unwrap_or(Duration::zero());
        self.values.push(SeriesItem {
            timestamp: prev + duration,
            value
        });
        self
    }
}

pub struct SeriesItem<T> {
    timestamp: Duration,
    value: T
}

impl<T> From<(Duration, T)> for SeriesItem<T> {
    fn from(val: (Duration, T)) -> Self {
        SeriesItem {
            timestamp: val.0,
            value: val.1
        }
    }
}
