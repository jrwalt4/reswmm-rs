use reswmm_core::{
    units::{qnty::Qnty, system::si::Length},
    xsection::{XSection, XS, RectangleXS},
};

#[allow(unused)]
macro_rules! my_macro {
    ($var:block) => {
        println!("result of '{}' is '{}'", stringify!($var), $var);
    };
}

fn main() {
    let width = Qnty::<Length>::new(2.0);
    let depth = Qnty::<Length>::new(2.0);
    let rect = XSection::from(RectangleXS::new(width));
    println!("Rect area at {:?} is {:?}.", depth, rect.area(depth));
    let hello = "world";
    my_macro![{ hello }];
}
