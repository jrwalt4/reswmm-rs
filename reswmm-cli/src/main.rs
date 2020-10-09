use reswmm_core::{
    units::{qnty::Qnty, si::Length},
    xsection,
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
    let rect = xsection::new_xs(xsection::Kind::Rectangle, width);
    println!("Rect area at {:?} is {:?}.", depth, rect.area(depth));
    let hello = "world";
    my_macro![{ hello }];
}
