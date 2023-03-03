use serde::{Serialize, Deserialize};

use std::ops::Deref;
use std::marker::PhantomData;
use std::sync::Arc;

pub type UID = i32;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Element<K> {
    pub uid: UID,
    pub name: String,
    #[serde(flatten)]
    pub kind: K
}

impl<K> Element<K> {
    pub fn new<S: ToString, U: Into<K>>(uid: UID, name: S, kind: U) -> Self {
        Element {uid, name: name.to_string(), kind: kind.into()}
    }
}

impl<K> Deref for Element<K> {
    type Target = K;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

/// Create an Element from a tuple of (UID, String, Kind)
impl<K, U: Into<K>, S: ToString> From<(UID, S, U)> for Element<K> {
    fn from((uid, name, kind): (UID, S, U)) -> Self {
        Element{uid, name: name.to_string(), kind: kind.into()}
    }
}

#[derive(Debug)]
pub struct Ref<K>(Arc<Element<K>>);

impl<K> Ref<K> {
    pub fn uid(&self) -> UID {
        self.0.uid
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }
}

impl<K> Clone for Ref<K> {
    fn clone(&self) -> Self {
        Ref(Arc::clone(&self.0))
    }
}

pub mod ir {
    use super::*;

    use std::collections::HashMap;

    #[repr(transparent)]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct RefIR<T> {
        uid: UID,
        #[serde(skip)]
        kind: PhantomData<T>
    }

    impl<T> RefIR<T> {
        pub fn new(uid: UID) -> Self {
            Self {
                uid,
                kind: PhantomData
            }
        }

        pub fn resolve(self, ref_map: &HashMap<UID, Ref<T>>) -> Option<Ref<T>> {
            ref_map.get(&self.uid).cloned()
        }
    }

    pub trait Resolver<T>: Fn(RefIR<T>) -> Option<Ref<T>> {}

    pub trait ResolveWith<F> {
        type Output;
        fn resolve(self, f: F) -> Option<Self::Output>;
    }
}
