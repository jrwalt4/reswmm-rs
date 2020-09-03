use reswmm_core::{xsection, units};

fn main() {
    let width = units::Length::new::<units::meter>(2.0);
    let depth = units::Length::new::<units::meter>(2.0);
    let rect = xsection::new_xs(xsection::Kind::Rectangle, width);
    println!("Rect area at {:?} is {:?}.", depth, rect.area(depth));
}
