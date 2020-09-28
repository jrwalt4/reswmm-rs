use reswmm_core::{xsection, units};

#[allow(unused)]
macro_rules! my_macro {
    ($var:block) => {
        println!("result of '{}' is '{}'", stringify!($var), $var);
    };
}

fn main() {
    let width = units::Qnty::<units::LengthUnit>::new(2.0);
    let depth = units::Qnty::<units::LengthUnit>::new(2.0);
    let rect = xsection::new_xs(xsection::Kind::Rectangle, width);
    println!("Rect area at {:?} is {:?}.", depth, rect.area(depth));
    let hello = "world";
    my_macro![{hello}];
}
