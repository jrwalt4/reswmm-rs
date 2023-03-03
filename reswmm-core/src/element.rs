use serde::{Serialize, Deserialize};

use std::ops::Deref;
use std::marker::PhantomData;

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

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ElementRef<T> {
    uid: UID,
    
    /// use pointer so [Rust drop check doesn't look to drop a T][a]
    /// 
    /// [a]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html#ownership-and-the-drop-check
    #[serde(skip)]
    kind: PhantomData<*const T>
}

impl<K> ElementRef<K> {
    pub fn new(uid: UID) -> Self {
        Self {
            uid, 
            kind: PhantomData
        }
    }
    pub fn uid(&self) -> UID {
        self.uid
    }
}
