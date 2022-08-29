use std::ops::Deref;

pub type UID = i32;

#[derive(Debug)]
pub struct Element<K> {
    pub uid: UID,
    pub name: String,
    pub kind: K
}

impl<K> Element<K> {
    pub fn new<U: Into<K>>(uid: UID, name: String, kind: U) -> Self {
        Element {uid, name, kind: kind.into()}
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
