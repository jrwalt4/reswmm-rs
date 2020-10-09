use crate::types::*;

pub trait BaseUnit {
    const conv: Real;
}

pub trait BaseUnitInfo {
    const abbr: Abbr;
}

#[derive(Debug, Copy, Clone)]
pub struct MeterBaseUnit;
impl BaseUnit for MeterBaseUnit {
    const conv: Real = 1.0;
}
impl BaseUnitInfo for MeterBaseUnit {
    const abbr: Abbr = "m";
}

#[derive(Debug, Copy, Clone)]
pub struct SecondBaseUnit;
impl BaseUnit for SecondBaseUnit {
    const conv: Real = 1.0;
}
impl BaseUnitInfo for SecondBaseUnit {
    const abbr: Abbr = "s";
}
