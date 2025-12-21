// lib.rs
pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Node {
                        id: id.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.id == id)
        }
    }
}