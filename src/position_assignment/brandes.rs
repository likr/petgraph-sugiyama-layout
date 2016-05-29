use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use super::super::graph::{Node, Edge};
use super::mark_conflicts::mark_conflicts;

pub fn brandes(
    graph: &mut Graph<Node, Edge, Directed>,
    layers: &Vec<Vec<NodeIndex>>
) {
    mark_conflicts(graph, layers);
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use super::*;
    use super::super::super::graph::{Node, Edge};

    #[test]
    fn test_brandes() {
        let mut graph = Graph::new();
        let a1 = graph.add_node(Node { width: 10, order: 1, dummy: false, .. Node::new() });
        let a2 = graph.add_node(Node { width: 10, order: 2, dummy: false, .. Node::new() });
        let b1 = graph.add_node(Node { width: 10, order: 1, dummy: false, .. Node::new() });
        let b2 = graph.add_node(Node { width: 10, order: 2, dummy: false, .. Node::new() });
        let b3 = graph.add_node(Node { width: 10, order: 3, dummy: true, .. Node::new() });
        let b4 = graph.add_node(Node { width: 10, order: 4, dummy: false, .. Node::new() });
        let b5 = graph.add_node(Node { width: 10, order: 5, dummy: true, .. Node::new() });
        let b6 = graph.add_node(Node { width: 10, order: 6, dummy: true, .. Node::new() });
        let b7 = graph.add_node(Node { width: 10, order: 7, dummy: false, .. Node::new() });
        let b8 = graph.add_node(Node { width: 10, order: 8, dummy: false, .. Node::new() });
        let c1 = graph.add_node(Node { width: 10, order: 1, dummy: false, .. Node::new() });
        let c2 = graph.add_node(Node { width: 10, order: 2, dummy: false, .. Node::new() });
        let c3 = graph.add_node(Node { width: 10, order: 3, dummy: true, .. Node::new() });
        let c4 = graph.add_node(Node { width: 10, order: 4, dummy: true, .. Node::new() });
        let c5 = graph.add_node(Node { width: 10, order: 5, dummy: true, .. Node::new() });
        let c6 = graph.add_node(Node { width: 10, order: 6, dummy: false, .. Node::new() });
        let d1 = graph.add_node(Node { width: 10, order: 1, dummy: false, .. Node::new() });
        let d2 = graph.add_node(Node { width: 10, order: 2, dummy: false, .. Node::new() });
        let d3 = graph.add_node(Node { width: 10, order: 3, dummy: true, .. Node::new() });
        let d4 = graph.add_node(Node { width: 10, order: 4, dummy: true, .. Node::new() });
        let d5 = graph.add_node(Node { width: 10, order: 5, dummy: true, .. Node::new() });
        let d6 = graph.add_node(Node { width: 10, order: 6, dummy: false, .. Node::new() });
        let d7 = graph.add_node(Node { width: 10, order: 7, dummy: true, .. Node::new() });
        let e1 = graph.add_node(Node { width: 10, order: 1, dummy: false, .. Node::new() });
        let e2 = graph.add_node(Node { width: 10, order: 2, dummy: false, .. Node::new() });
        let e3 = graph.add_node(Node { width: 10, order: 3, dummy: false, .. Node::new() });
        graph.add_edge(a1, b1, Edge::new());
        graph.add_edge(a1, b6, Edge::new());
        graph.add_edge(a1, b8, Edge::new());
        graph.add_edge(a2, b3, Edge::new());
        graph.add_edge(a2, b5, Edge::new());
        graph.add_edge(b2, c2, Edge::new());
        graph.add_edge(b3, c2, Edge::new());
        graph.add_edge(b4, c2, Edge::new());
        graph.add_edge(b5, c3, Edge::new());
        graph.add_edge(b6, c4, Edge::new());
        graph.add_edge(b7, c2, Edge::new());
        graph.add_edge(b7, c6, Edge::new());
        graph.add_edge(b8, c2, Edge::new());
        graph.add_edge(b8, c5, Edge::new());
        graph.add_edge(c1, d1, Edge::new());
        graph.add_edge(c1, d2, Edge::new());
        graph.add_edge(c1, d6, Edge::new());
        graph.add_edge(c3, d4, Edge::new());
        graph.add_edge(c4, d5, Edge::new());
        graph.add_edge(c5, d6, Edge::new());
        graph.add_edge(c6, d3, Edge::new());
        graph.add_edge(c6, d7, Edge::new());
        graph.add_edge(d1, e1, Edge::new());
        graph.add_edge(d1, e2, Edge::new());
        graph.add_edge(d2, e2, Edge::new());
        graph.add_edge(d3, e1, Edge::new());
        graph.add_edge(d4, e3, Edge::new());
        graph.add_edge(d5, e3, Edge::new());
        graph.add_edge(d6, e3, Edge::new());
        graph.add_edge(d7, e3, Edge::new());
        let layers = vec![
            vec![a1, a2],
            vec![b1, b2, b3, b4, b5, b6, b7, b8],
            vec![c1, c2, c3, c4, c5, c6],
            vec![d1, d2, d3, d4, d5, d6, d7],
            vec![e1, e2, e3],
        ];
        brandes(&mut graph, &layers);
    }
}
