use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use super::super::graph::{Node, Edge};

fn segment(
    graph: &Graph<Node, Edge, Directed>,
    h1: &Vec<NodeIndex>
) -> (Vec<(NodeIndex, NodeIndex)>, Vec<(NodeIndex, NodeIndex)>){
    let mut inner = vec![];
    let mut outer = vec![];
    for u in h1 {
        for v in graph.neighbors(*u) {
            if graph[*u].dummy && graph[v].dummy {
                inner.push((*u, v));
            } else {
                outer.push((*u, v));
            }
        }
    }
    (inner, outer)
}

pub fn mark_conflicts(
    graph: &mut Graph<Node, Edge, Directed>,
    layers: &Vec<Vec<NodeIndex>>
) {
    for i in 1..(layers.len() - 1) {
        let h1 = layers.get(i).unwrap();
        let (inner, outer) = segment(graph, &h1);
        for (u1, v1) in inner {
            for &(u2, v2) in &outer {
                let ou1 = graph[u1].order;
                let ou2 = graph[u2].order;
                let ov1 = graph[v1].order;
                let ov2 = graph[v2].order;
                if (ou1 < ou2 && ov1 > ov2) || (ou1 > ou2 && ov1 < ov2) {
                    let index = graph.find_edge(u2, v2).unwrap();
                    graph[index].conflict = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use super::*;
    use super::super::super::graph::{Node, Edge};

    #[test]
    fn test_mark_conflicts() {
        let mut graph = Graph::new();
        let a = graph.add_node(Node::new());
        let b = graph.add_node(Node::new());
        let c = graph.add_node(Node::new());
        graph.add_edge(a, b, Edge::new());
        graph.add_edge(b, c, Edge::new());
        graph.add_edge(c, a, Edge::new());
        let layers = vec![
            vec![a],
            vec![b],
            vec![c],
        ];
        mark_conflicts(&mut graph, &layers);
    }
}
