use uom::si::f64::{Length, Area};


pub trait XSection {
    fn area(&self, depth: Length) -> Area;
}

pub struct Rectangle {
    width: Length
}

impl Rectangle {
    fn new(width: Length) -> Rectangle {
        return Rectangle {
            width
        };
    }
}

impl XSection for Rectangle {
    fn area(&self, depth: Length) -> Area {
        return self.width * depth;
    }
}

pub struct Circle {
    diameter: Length
}

impl Circle {
    fn new(diameter: Length) -> Circle {
        return Circle {
            diameter
        };
    }
}

impl XSection for Circle {
    fn area(&self, depth: Length) -> Area {
        return self.diameter * depth;
    }
}

pub enum Kind {
    Circle,
    Rectangle
}

pub fn new_xs(kind: Kind, prop: Length) -> Box<dyn XSection> {
    return match kind {
        Kind::Circle => Box::new(Circle::new(prop)),
        Kind::Rectangle => Box::new(Rectangle::new(prop))
    };
}
