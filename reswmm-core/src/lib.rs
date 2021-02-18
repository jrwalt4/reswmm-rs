extern crate enum_dispatch;
pub extern crate furlong as units;
pub mod xsection;
#[cfg(test)]
mod tests {
    use super::*;
    use xsection::{XSection, XS, RectangleXS};
    use units::{qnty::Qnty, system::si::{Length, Area}};
    #[test]
    fn xsection() {
        let xs = XSection::from(RectangleXS::new(Qnty::<Length>::new(2.0)));
        let depth = Qnty::<Length>::new(2.0);
        let area = Qnty::<Area>::new(4.0);
        assert_eq!(xs.area(depth), area);
    }
}
