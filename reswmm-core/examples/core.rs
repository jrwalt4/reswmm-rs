use reswmm_core::{
    element::{Element, UID},
    node::*,
    link::*, series::Series
};

use chrono::{NaiveDateTime, Duration};

use std::{collections::HashMap, iter::FromIterator};

fn main() {

    fn into_tuple<K>(el: Element<K>) -> (UID, Element<K>) {
        (el.uid, el)
    }

    let nodes = HashMap::<UID, NodeElement>::from_iter(
        (vec![
            NodeElement::new(1, "J1", Junction::new(101.0)),
            NodeElement::new(2, "J2", Junction::new(100.0)),
            NodeElement::new(3, "J3", Junction::new(99.0))
        ]).into_iter().map(into_tuple)
    );

    let links = HashMap::<UID, LinkElement>::from_iter(
        (vec![
            LinkElement::new(1, "C1", Conduit::new(100.0)),
            LinkElement::new(2, "C2", Conduit::new(100.0)),
        ]).into_iter().map(into_tuple)
    );

    let mut flows = Series::<HashMap<UID, f64>>::new();
    flows
        .push((Duration::zero(), HashMap::from([(1, 5.0)])))
        .push_after(Duration::minutes(30), HashMap::from([(1, 2.5)]))
        .push_after(Duration::minutes(30), HashMap::from([(1, 0.0)]));
    
}
