use enum_dispatch::enum_dispatch;
use furlong::{
    qnty::Qnty,
    system::si,
};

type Length = Qnty<si::Length>;
type Area = Qnty<si::Area>;

#[enum_dispatch]
pub enum XSection {
    Circle(CircleXS),
    Rectangle(RectangleXS),
}

#[enum_dispatch(XSection)]
pub trait XS {
    fn area(&self, depth: Length) -> Area;
}

pub struct RectangleXS {
    width: Length,
}

impl RectangleXS {
    pub fn new(width: Length) -> RectangleXS {
        return RectangleXS { width };
    }
}

impl XS for RectangleXS {
    fn area(&self, depth: Length) -> Area {
        return self.width * depth;
    }
}

pub struct CircleXS {
    diameter: Length,
}

impl CircleXS {
    pub fn new(diameter: Length) -> CircleXS {
        return CircleXS { diameter };
    }
}

impl XS for CircleXS {
    fn area(&self, depth: Length) -> Area {
        return self.diameter * depth;
    }
}
