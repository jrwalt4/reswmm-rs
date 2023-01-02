/// Stormwater Management Model (SWMM) version 6

pub extern crate furlong as units;
pub mod xsection;
pub mod element;
pub mod node;
pub mod link;
pub mod region;
pub mod io;
pub mod project;

pub use project::Project;

#[cfg(test)]
mod test {
    use crate::link::{Conduit};

    use super::*;
    #[test]
    fn add_link() {
        let mut prj = Project::new();
        let l = Conduit{length: 2.0};
        prj.add_link(1, "L-1", l);
    }

    #[cfg(feature="custom_links")]
    #[test]
    fn add_custom_link() {
        let mut prj = Project::new();
        let l2: link::CustomLink = link::CustomLink::new(Box::new(Conduit{length: 3.0}));
        let old_link = prj.add_link(2, "L-2", link::LinkKind::Custom(l2));
        assert!(old_link.is_none())
    }

}

pub fn run() {
    use crate::link::{Conduit};

    let mut prj = Project::new();
    let l = Conduit{length: 2.0};
    prj.add_link(1, "L-1", l);

    #[cfg(feature="custom_links")]
    {
        let l2: link::CustomLink = link::CustomLink::new(Box::new(Conduit{length: 3.0}));
        prj.add_link(2, "L-2", link::LinkKind::Custom(l2));
    }

    for link in prj.links() {
        println!("{link:?}");
    }
}
