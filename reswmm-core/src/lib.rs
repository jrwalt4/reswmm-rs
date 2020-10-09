pub extern crate furlong as units;
pub mod xsection;
#[cfg(test)]
mod tests {
    use super::*;
    use xsection::{Rectangle, XSection};
    use units::{qnty::Qnty, unit::{LengthUnit, AreaUnit}};
    #[test]
    fn xsection() {
        let xs = Rectangle::new(Qnty::<LengthUnit>::new(2.0));
        let depth = Qnty::<LengthUnit>::new(2.0);
        let area = Qnty::<AreaUnit>::new(4.0);
        assert_eq!(xs.area(depth), area);
    }
}
