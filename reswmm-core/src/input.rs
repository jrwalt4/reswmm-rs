
use crate::element::Element;
use crate::node::NodeKind;

use rusqlite::{Connection, Statement};

use std::marker::PhantomData;

pub type Result<T> = std::result::Result<T, String>;

pub trait Input<K> {
    fn read(&self) -> Result<()>;
}

pub struct Inserter<K>(PhantomData<K>);

impl<K> Inserter<K> {
    pub fn new() -> Self {
        Inserter(PhantomData)
    }

}
