use bevy_ecs::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
pub struct UID(pub i32);

impl From<i32> for UID {
    fn from(value: i32) -> Self {
        UID(value)
    }
}

#[derive(Debug, Component)]
pub struct Name(pub String);

impl<S: ToString> From<S> for Name {
    fn from(s: S) -> Self {
        Name(s.to_string())
    }
}

#[derive(Bundle)]
pub struct Element {
    uid: UID,
    name: Name
}

impl Element {
    pub fn new<I: Into<UID>, S: ToString>(uid: I, name: S) -> Self {
        Element { uid: uid.into(), name: name.into() }
    }
}
