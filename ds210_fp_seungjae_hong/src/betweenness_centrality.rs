use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub type Distance = i64;

/// Compute the betweenness centrality of each node.
pub fn calculate_betweenness_centrality(graph: &Graph<String, i32>, start_node: NodeIndex) -> Vec<f64> {
    let shortest_paths = find_shortest_paths(graph, start_node);
    let mut centrality_values = vec![0.0; graph.node_count()];

    for node in graph.node_indices() {
        let mut path_count = 0.0;

        if let Some(node_dist) = shortest_paths[node.index()] {
            for neighbor in graph.neighbors_directed(node, Direction::Incoming) {
                if let Some(neighbor_dist) = shortest_paths[neighbor.index()] {
                    let edge_weight = *graph.edge_weight(graph.find_edge(neighbor, node).unwrap()).unwrap();

                    if node_dist == neighbor_dist + edge_weight as i64 {
                        path_count += 1.0;
                        centrality_values[neighbor.index()] +=
                            (1.0 + centrality_values[node.index()]) / path_count;
                    }
                }
            }
        }

        if node != start_node {
            centrality_values[node.index()] *= 0.5;
        }
    }

    centrality_values
}

/// Find the shortest paths in a directed graph.
pub fn find_shortest_paths(graph: &Graph<String, i32>, start_node: NodeIndex) -> Vec<Option<Distance>> {
    let mut distances: Vec<Option<Distance>> = vec![None; graph.node_count()];
    distances[start_node.index()] = Some(0);

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start_node)));

    while let Some(Reverse((distance, current_node))) = priority_queue.pop() {
        if let Some(current_distance) = distances[current_node.index()] {
            if distance > current_distance {
                continue;
            }
        }

        for neighbor in graph.neighbors_directed(current_node, Direction::Outgoing) {
            let edge_weight = *graph.edge_weight(graph.find_edge(current_node, neighbor).unwrap()).unwrap();
            let new_distance = distance + edge_weight as i64;

            if distances[neighbor.index()].map_or(true, |d| new_distance < d) {
                distances[neighbor.index()] = Some(new_distance);
                priority_queue.push(Reverse((new_distance, neighbor)));
            }
        }
    }

    distances
}
