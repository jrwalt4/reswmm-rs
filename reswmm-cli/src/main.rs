#[macro_use]
extern crate clap;
use clap::App;
use reswmm_core::{
    units::Length,
    xsection::{XSection, XS, RectangleXS}
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let width_arg = matches.value_of("width").unwrap(); // required input
    let width = Length::from_raw_value(width_arg.parse().expect("Invalid width"));

    let depth_arg = matches.value_of("depth").unwrap(); // required input
    let depth = Length::from_raw_value(depth_arg.parse().expect("Invalid depth"));

    let rect = XSection::from(RectangleXS::new(width));
    let area = rect.a_of_y(depth);

    let verbose = matches.is_present("verbose");
    if verbose {
        println!("Rect area at {:?} is {:?}.", depth, area);
    } else {
        println!("{:?}", area);
    }
}
