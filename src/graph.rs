use petgraph::graph::NodeIndex;

pub struct Node {
    pub layer: usize,
    pub order: usize,
    pub width: usize,
    pub height: usize,
    pub x: i32,
    pub y: i32,
    pub dummy: bool,
    pub align: Option<NodeIndex>,
    pub root: Option<NodeIndex>,
    pub sink: Option<NodeIndex>,
    pub shift: i32,
}

impl Node {
    pub fn new() -> Node {
        Node {
            layer: 0,
            order: 0,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            dummy: false,
            align: None,
            root: None,
            sink: None,
            shift: i32::min_value(),
        }
    }
}

pub struct Edge {
    pub conflict: bool,
}

impl Edge {
    pub fn new() -> Edge {
        Edge {
            conflict: false,
        }
    }
}
