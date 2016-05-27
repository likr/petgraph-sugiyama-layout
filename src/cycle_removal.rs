use std::collections::HashSet;
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::visit::{VisitMap, Visitable};
use fixedbitset::FixedBitSet;

fn dfs<N, E> (
    graph: &Graph<N, E, Directed>,
    map: &mut FixedBitSet,
    ancestors: &mut HashSet<NodeIndex>,
    result: &mut Vec<(NodeIndex, NodeIndex)>,
    u: NodeIndex
) {
    if map.is_visited(&u) {
        return;
    }
    map.visit(u);
    ancestors.insert(u);
    for v in graph.neighbors(u) {
        if ancestors.contains(&v) {
            result.push((u, v));
        } else {
            dfs(graph, map, ancestors, result, v)
        }
    }
    ancestors.remove(&u);
}

pub fn cycle_edges<N, E> (
    graph: &Graph<N, E, Directed>
) -> Vec<(NodeIndex, NodeIndex)> {
    let mut map = graph.visit_map();
    let mut ancestors = HashSet::new();
    let mut result = vec![];
    for u in graph.node_indices() {
        dfs(&graph, &mut map, &mut ancestors, &mut result, u)
    }
    result
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = Graph::<&str, &str>::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        graph.add_edge(a, b, "");
        graph.add_edge(b, c, "");
        graph.add_edge(c, a, "");
        assert_eq!(cycle_edges(&graph), vec![(c, a)]);
    }
}
