#![allow(unused, non_upper_case_globals)]
/*
#[macro_use]
extern crate typenum;
// */
use typenum::{Integer as Int, Unsigned, Length, Sum, consts::*}; // 1.12.0
use std::marker::PhantomData as PD;
use std::fmt::{self, Debug};
use std::ops::{Add, Mul};

type Real = f64;
type Abbr = &'static str;

pub trait Dimension {
    type Length;
    type Time;    
}

#[derive(Debug, Copy, Clone)]
pub struct MakeDimension<L,T> {
    length_dim:PD<L>,
    time_dim:PD<T>
}

impl<L, T> MakeDimension<L,T> {
    fn new() -> MakeDimension<L,T> {
        MakeDimension {
            length_dim: PD,
            time_dim: PD
        }
    }
}
impl<L,T> Dimension for MakeDimension<L,T> {
    type Length = L;
    type Time = T;
}
impl<Ll, Tl, Lr, Tr> Mul<MakeDimension<Lr, Tr>> for MakeDimension<Ll,Tl> 
where Ll: Int+Add<Lr>, Tl: Int+Add<Tr>, Lr: Int, Tr: Int {
    type Output = MakeDimension<Sum<Ll, Lr>,Sum<Tl, Tr>>;
    fn mul(self, rhs: MakeDimension<Lr,Tr>) -> Self::Output {
        MakeDimension::new()
    }
}

type ProdDimension<Dl, Dr> = <Dl as Mul<Dr>>::Output;

pub type LengthDimension = MakeDimension<P1,Z0>;
pub type TimeDimension = MakeDimension<Z0,P1>;

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

pub trait UnitSystem {
    type Length: BaseUnit;//+BaseUnitInfo;
    type Time: BaseUnit;//+BaseUnitInfo;
}

#[derive(Debug, Copy, Clone)]
pub struct System<LB:BaseUnit,TB:BaseUnit> {
    length_base:PD<LB>,
    time_base:PD<TB>
}

impl<LB:BaseUnit,TB:BaseUnit> UnitSystem for System<LB,TB> {
    type Length = LB;
    type Time = TB;
}

pub type SI = System<MeterBaseUnit, SecondBaseUnit>;

pub trait Unit: Sized {
    type System: UnitSystem;
    type Dim: Dimension;
    fn from_value(value: Real) -> Qnty<Self> {
        Qnty::<Self>::new(value)
    }
}

pub trait UnitInfo: Unit {
    fn abbr() -> String;
}

#[derive(Debug, Copy, Clone)]
pub struct MakeUnit<S:UnitSystem, D: Dimension> {
    system: PD<S>,
    dimension: PD<D>
}
impl<S: UnitSystem, D: Dimension> MakeUnit<S,D> {
    fn new() -> MakeUnit<S,D> {
        MakeUnit {
            system: PD,
            dimension: PD
        }
    }
}
impl<S:UnitSystem, D:Dimension> Unit for MakeUnit<S,D> {
    type System = S;
    type Dim = D;
}

impl<S, D> UnitInfo for MakeUnit<S,D>
where 
    S:UnitSystem,
    <S as UnitSystem>::Length:BaseUnitInfo,
    <S as UnitSystem>::Time:BaseUnitInfo,
    D:Dimension,
    <D as Dimension>::Length: Int,
    <D as Dimension>::Time: Int
{
    fn abbr() -> String {
        let length_abbr = <<S as UnitSystem>::Length as BaseUnitInfo>::abbr;
        let length_pwr = <<D as Dimension>::Length as Int>::I8;

        let time_abbr = <<S as UnitSystem>::Time as BaseUnitInfo>::abbr;
        let time_pwr = <<D as Dimension>::Time as Int>::I8;
        
        let length_part = match length_pwr {
            0 => String::from(""),
            1 => String::from(length_abbr),
            _ => format!("{}^{}", length_abbr, length_pwr)
        };
        let time_part = match time_pwr {
            0 => String::from(""),
            1 => String::from(time_abbr),
            _ => format!("{}^{}", time_abbr, time_pwr)
        };
        
        format!("{}{}", length_part.as_str(), time_part.as_str())
    }
}

impl<S, Dl, Dr> Mul<MakeUnit<S, Dr>> for MakeUnit<S, Dl> 
where
    S: UnitSystem,
    Dl: Dimension+Mul<Dr>,
    <Dl as Mul<Dr>>::Output: Dimension,
    Dr: Dimension
    {
    type Output = MakeUnit<S, ProdDimension<Dl, Dr>>;
    fn mul(self, rhs: MakeUnit<S, Dr>) -> Self::Output {
        MakeUnit::new()
    }
}

type ProdUnit<Ul, Ur> = <Ul as Mul<Ur>>::Output;

#[derive(Copy, Clone)]
pub struct Qnty<U:Unit> {
    value: Real,
    unit: PD<U>,
}

impl<U:Unit> Qnty<U> {
    pub fn new(value: Real) -> Qnty<U> {
        Qnty {
            value,
            unit: PD
        }
    }
}

impl<Ul,Ur> Add<Qnty<Ur>> for Qnty<Ul>
where Ul:Unit,
      Ur:Unit<Dim = <Ul as Unit>::Dim> {
    type Output = Qnty<Ul>;
    fn add(self, rhs: Qnty<Ur>) -> Self::Output { //<Self as Add<Qnty<UR>>>
        Qnty::<Ul>::new(self.value + rhs.value)
    }
}

impl<Ul, Ur> Mul<Qnty<Ur>> for Qnty<Ul>
where
    Ul:Unit+Mul<Ur>,
    <Ul as Mul<Ur>>::Output: Unit,
    Ur:Unit<System=<Ul as Unit>::System>
{
    type Output = Qnty<ProdUnit<Ul,Ur>>;
    fn mul(self, rhs: Qnty<Ur>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<U:UnitInfo> Debug for Qnty<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.value, <U as UnitInfo>::abbr())
    }
}

pub type LengthUnit = MakeUnit<SI, LengthDimension>;
pub type AreaUnit = MakeUnit<SI, MakeDimension<P2,Z0>>;
pub type TimeUnit = MakeUnit<SI, TimeDimension>;

#[cfg(test)]
mod unit_test {
    use super::{Qnty, Unit, LengthUnit, TimeUnit};
    #[test]
    fn length() {
        let l1 = Qnty::<LengthUnit>::new(2.0);
    let l2 = <LengthUnit as Unit>::from_value(1.5);
    dbg!(l1);
    dbg!(l2);
    dbg!(l1+l2);
    dbg!(l1*l2);
    
    let t1 = Qnty::<TimeUnit>::new(1.0);
    dbg!(t1);
    
    dbg!(l1*t1);

    }
}
