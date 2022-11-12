#[macro_use]
extern crate clap;
use clap::App;
use reswmm_core::{
    units::{Qnty, system::si::Meters as Length},
    xsection::{XSection, XS, RectangleXS},
    run
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let width_arg = matches.value_of("width").unwrap(); // required input
    let width = Qnty::<Length>::from(width_arg.parse::<f64>().expect("Invalid width"));

    let depth_arg = matches.value_of("depth").unwrap(); // required input
    let depth = Qnty::<Length, f64>::from(depth_arg.parse::<f64>().expect("Invalid depth"));

    let rect = XSection::from(RectangleXS::new(width));
    let area = rect.area(depth);

    let verbose = matches.is_present("verbose");
    if verbose {
        println!("Rect area at {:?} is {:?}.", depth, area);
    } else {
        println!("{:?}", area);
    }

    run();
}
