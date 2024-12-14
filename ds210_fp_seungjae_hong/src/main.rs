mod betweenness_centrality;
mod graph_construction;
use betweenness_centrality::calculate_betweenness_centrality;
use petgraph::prelude::*;

fn main() {
    let file_path = "data/EPL_standings_2000-2022.csv";
    let graph = graph_construction::construct_graph(file_path);

    // Extract team names and node count
    let teams = graph
        .node_indices()
        .map(|node| graph.node_weight(node).unwrap().clone())
        .collect::<Vec<_>>();
    let count = graph.node_count();

    // Compute betweenness centrality
    let mut centrality_scores: Vec<(String, f64)> = Vec::with_capacity(count);
    for v_index in 0..count {
        let centrality = calculate_betweenness_centrality(&graph, NodeIndex::new(v_index))[v_index];
        centrality_scores.push((teams[v_index].clone(), centrality));
    }

    // Sort centrality scores in descending order
    centrality_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print betweenness centrality scores
    println!("\nBetweenness Centrality Scores:");
    for (team, score) in &centrality_scores {
        println!("Team: {:<30}, Centrality: {:.4}", team, score);
    }

    // Print graph nodes
    println!("\nGraph Nodes:");
    for node_index in graph.node_indices() {
        if let Some(node_weight) = graph.node_weight(node_index) {
            println!("Node Index: {:<3}, Team: {}", node_index.index(), node_weight);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_graph_construction_and_centrality() {
        // Create mock EPL dataset
        let csv_data = "Team,W,L,GF,Season\n\
                        TeamA,10,5,30,2022\n\
                        TeamB,8,7,25,2022\n\
                        TeamC,12,3,35,2022\n\
                        TeamD,7,8,20,2022\n";

        // Write the mock dataset to a temporary file
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("mock_data.csv");
        fs::write(&file_path, csv_data).unwrap();

        // Construct the graph from the mock dataset
        let graph = graph_construction::construct_graph(file_path.to_str().unwrap());

        // Log graph nodes and edges for debugging
        println!("\nDebug: Graph Nodes:");
        for node_index in graph.node_indices() {
            println!(
                "Node Index: {:<3} | Team: {}",
                node_index.index(),
                graph.node_weight(node_index).unwrap()
            );
        }

        println!("\nDebug: Graph Edges:");
        for edge in graph.edge_indices() {
            let (source, target) = graph.edge_endpoints(edge).unwrap();
            let edge_weight = graph.edge_weight(edge).unwrap();
            println!(
                "Edge from Node {:<3} to Node {:<3} | Weight: {}",
                source.index(),
                target.index(),
                edge_weight
            );
        }

        // Check that the correct number of nodes and edges were created
        assert_eq!(graph.node_count(), 4, "Node count mismatch"); 
        assert_eq!(graph.edge_count(), 12, "Edge count mismatch");

        // Compute betweenness centrality
        let teams = graph
            .node_indices()
            .map(|node| graph.node_weight(node).unwrap().clone())
            .collect::<Vec<_>>();
        let count = graph.node_count();

        let mut centrality_scores: Vec<(String, f64)> = Vec::with_capacity(count);
        for v_index in 0..count {
            let centrality = calculate_betweenness_centrality(&graph, NodeIndex::new(v_index))[v_index];
            centrality_scores.push((teams[v_index].clone(), centrality));
        }

        // Log centrality scores for debugging
        println!("\nDebug: Centrality Scores Before Sorting: {:?}", centrality_scores);

        centrality_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("Debug: Centrality Scores After Sorting: {:?}", centrality_scores);

        // Verify that centrality scores are non-negative and sorted correctly
        for (i, (_, score)) in centrality_scores.iter().enumerate() {
            assert!(*score >= 0.0, "Centrality score is negative for index {}", i);
            if i > 0 {
                assert!(
                    centrality_scores[i - 1].1 >= *score,
                    "Centrality scores are not sorted correctly at index {}",
                    i
                );
            }
        }

        // Validate that the number of centrality scores matches node count
        assert_eq!(
            centrality_scores.len(),
            4,
            "Centrality scores length mismatch with node count"
        );
    }
}