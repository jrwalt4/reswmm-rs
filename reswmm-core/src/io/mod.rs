/// Test cases for de/serializing elements

#[cfg(test)]
mod io_test {
    use crate::node::{NodeElement, Junction};
    use serde_json;

    #[test]
    fn serde_node() {
        let n1 = NodeElement::new(1, "N1", Junction::new(101.0));
        let n1_json = serde_json::to_string(&n1).unwrap();
        
        let n1_de: NodeElement = serde_json::from_str(&n1_json).unwrap();
        assert_eq!(n1, n1_de);
        
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
}
