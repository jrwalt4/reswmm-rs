//! Common units used throughout

pub use furlong::{Qnty,system::{self, si}};
use serde::{Serializer, Deserializer, de::Visitor};

pub type Length = Qnty<si::Meters>;

pub type Area = Qnty<system::Area<si::System>>;

// TODO: units of Length^5/3
pub type SectFact = f64;

pub fn serialize<S: Serializer, U>(q: &Qnty<U>, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_f64(*q.raw_value())
}

pub fn deserialize<'de, D: Deserializer<'de>, U>(d: D) -> Result<Qnty<U>, D::Error> {
    use std::marker::PhantomData;
    struct QuantityVisitor<U>(PhantomData<U>);
    impl<'de, U> Visitor<'de> for QuantityVisitor<U> {
        type Value = Qnty<U>;
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(Qnty::from_raw_value(v))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a quantity")
        }
    }
    d.deserialize_f64(QuantityVisitor::<U>(PhantomData))
}