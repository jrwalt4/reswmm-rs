/// Test cases for de/serializing elements

#[cfg(test)]
mod io_test {
    use crate::node::{NodeElement, Junction};
    use crate::region::{RegionElement, SubBasin};
    use serde_json;

    macro_rules! test_ser_de {
        ($elem:expr) => {
            let elem = $elem;
            let elem_serialized = serde_json::to_string(&elem).unwrap();
            let elem_deserialized = serde_json::from_str(&elem_serialized).unwrap();
            assert_eq!(elem, elem_deserialized);
        };
    }

    #[test]
    fn serde_node() {
        test_ser_de!(NodeElement::new(1, "N1", Junction::new(101.0)));
    }

    #[test]
    fn serde_node_json() {
        let n2 = NodeElement::new(2, "N2", Junction::new(102.0));
        let n2_json = serde_json::json!({
            "kind": "Junction",
            "uid": 2,
            "name": "N2",
            "invert": 102.0
        }).to_string();
        let n2_de = serde_json::from_str(&n2_json).unwrap();
        assert_eq!(n2, n2_de);
    }

    #[test]
    fn serde_region() {
        test_ser_de!(RegionElement::new(1, "SB1", SubBasin::new(25.0)));
    }
}
