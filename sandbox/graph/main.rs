use petgraph::graph::UnGraph;

struct Node {
    pub token: Cow<>
}

fn main() {
    let g = UnGraph::<i32, ()>::from_edges(&[
        (1, 2), (2, 3), (3, 4),
        (1, 4)]);
}