//! Cross Section shapes

use crate::units::{self, Length, Area, SectFact};

use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};
use bevy_ecs::prelude::*;

#[enum_dispatch]
#[derive(Debug, Component)]
pub enum XSection {
    Circle(CircleXS),
    Rectangle(RectangleXS),
}


/// Cross Section
///     y = flow depth
///     a = flow area
///     r = hyd. radius
///     s = section factor = A*R^(2/3)
#[enum_dispatch(XSection)]
pub trait XS {

    /// Area at depth
    fn a_of_y(&self, depth: Length) -> Area;
    
    /// Area with section factor
    fn a_of_s(&self, sf: SectFact) -> Area;
    
    /// Top width at depth
    fn w_of_y(&self, depth: Length) -> Length;

    /// Hydraulic radius at depth
    fn r_of_y(&self, depth: Length) -> Length;
    
    /// Depth with given area
    fn y_of_a(&self, area: Area) -> Length;
    
    /// Hydraulic radius with given area
    fn r_of_a(&self, area: Area) -> Length {
        self.r_of_y(self.y_of_a(area))
    }
    
    /// Section factor with given area
    fn s_of_a(&self, area: Area) -> SectFact {
        // TODO: compare with units
        if area.raw_value() == &0.0 {
            return 0.0;
        }
        let r = self.r_of_a(area);
        if r.raw_value() < &0.01 {
            return 0.0;
        }
        area.raw_value() * r.raw_value().powf(2.0/3.0)
    }
    
    /// Derivative of section factor w.r.t. area at given area
    fn ds_da(&self, area: Area) -> SectFact;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RectangleXS {
    #[serde(with="units")]
    width: Length,
}

impl RectangleXS {
    pub fn new(width: Length) -> RectangleXS {
        return RectangleXS { width };
    }
}

impl XS for RectangleXS {
    fn a_of_y(&self, depth: Length) -> Area {
        return self.width * depth;
    }

    fn a_of_s(&self,sf:SectFact) -> Area {
        todo!()
    }

    fn w_of_y(&self,depth:Length) -> Length {
        todo!()
    }

    fn r_of_y(&self,depth:Length) -> Length {
        todo!()
    }

    fn y_of_a(&self,area:Area) -> Length {
        todo!()
    }

    fn ds_da(&self,area:Area) -> SectFact {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CircleXS {
    #[serde(with="units")]
    diameter: Length,
}

impl CircleXS {
    pub fn new(diameter: Length) -> CircleXS {
        return CircleXS { diameter };
    }
}

impl XS for CircleXS {
    fn a_of_y(&self, depth: Length) -> Area {
        return self.diameter * depth;
    }

    fn a_of_s(&self,sf:SectFact) -> Area {
        todo!()
    }

    fn w_of_y(&self,depth:Length) -> Length {
        todo!()
    }

    fn r_of_y(&self,depth:Length) -> Length {
        todo!()
    }

    fn y_of_a(&self,area:Area) -> Length {
        todo!()
    }

    fn ds_da(&self,area:Area) -> SectFact {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xsection() {
        let xs = XSection::from(RectangleXS::new(Length::from(2.0)));
        let depth = Length::from(2.0);
        let area = Area::from(4.0);
        assert_eq!(xs.a_of_y(depth), area);
    }
}
