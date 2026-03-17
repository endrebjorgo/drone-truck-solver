type Node = usize;

#[derive(Clone, Debug, Default)]
pub struct Flight {
    pub start: Node,
    pub goal:  Node,
    pub end:   Node,
}

#[derive(Clone, Debug, Default)]
pub struct Solution {
    pub truck_path: Vec<Node>,
    pub flights1: Vec<Flight>,
    pub flights2: Vec<Flight>,
}

