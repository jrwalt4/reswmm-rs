//! Series of values (flowrate, depth, rainfall, etc.)

use crate::time::Time;

use std::collections::{BTreeMap, btree_map};

pub struct Series<T> {
    values: BTreeMap<Time, SeriesItem<T>>
}

impl<T> Series<T> {
    pub fn new() -> Self {
        Series {
            values: BTreeMap::new()
        }
    }

    pub fn add<U: Into<SeriesItem<T>>>(&mut self, item: U) -> &mut Self {
        let item = item.into();
        self.values.insert(item.time, item);
        self
    }

    pub fn get<Q>(&self, time: &Q) -> Option<&T> 
    where
        Time: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized
    {
        self.values.get(time).map(|item| &item.value)
    }

}

impl<T> IntoIterator for Series<T> {
    type IntoIter = btree_map::IntoValues<Time, SeriesItem<T>>;
    type Item = SeriesItem<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_values()
    }
}

pub struct SeriesItem<T> {
    time: Time,
    value: T
}

impl<T> SeriesItem<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> From<(Time, T)> for SeriesItem<T> {
    fn from(val: (Time, T)) -> Self {
        SeriesItem {
            time: val.0,
            value: val.1
        }
    }
}
