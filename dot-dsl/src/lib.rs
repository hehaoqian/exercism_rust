pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug, Default)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        ..Self::default()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug, Default)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        ..Self::default()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    if self.attrs.contains_key(key) {
                        return Some(self.attrs.get(key).unwrap().as_str());
                    }
                    None
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.insert(k.to_string(), v.to_string());
            });
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.name == name {
                    return Some(node);
                }
            }
            None
        }
    }
}
