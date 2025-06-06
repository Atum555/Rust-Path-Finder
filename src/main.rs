mod models;
mod parsers;

use parsers::*;
use std::env;
use std::fs::File;

use crate::models::{Distance, Graph, GraphError, Location};

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <locations_file> <distances_file>", args[0]);
        std::process::exit(1);
    }

    // Open locations files
    let locations_file = &args[1];
    let locations_file = File::open(locations_file).unwrap_or_else(|err| {
        eprintln!("Error opening locations file {}: {}", locations_file, err);
        std::process::exit(1);
    });

    // Open distances file
    let distances_file = &args[2];
    let distances_file = File::open(distances_file).unwrap_or_else(|err| {
        eprintln!("Error opening distances file {}: {}", distances_file, err);
        std::process::exit(1);
    });

    // Parse data files
    let locations = locations_parser::parse_file(&locations_file).expect("Error parsing locations");
    let distances = distances_parser::parse_file(&distances_file).expect("Error parsing distances");

    // Close files
    drop(locations_file);
    drop(distances_file);

    // Init graph
    let mut graph: Graph<Location, Distance> = Graph::new();

    // Insert Nodes
    for location in locations {
        graph.add_node(location);
    }

    // Insert Edges
    for distance in distances {
        let node_1_id = graph.get_node_id(&distance.0).expect(&format!(
            "Tried to connect nodes: '{}', '{}' but node '{}' does not exist",
            distance.0, distance.1, distance.0
        ));
        let node_2_id = graph.get_node_id(&distance.1).expect(&format!(
            "Tried to connect nodes: '{}', '{}' but node '{}' does not exist",
            distance.0, distance.1, distance.1
        ));
        match graph.add_bidirectional_edge(node_1_id, node_2_id, distance.2) {
            Err(GraphError::NodeNotFound(node_id)) => {
                panic!("Node does not exist: {:?}", node_id);
            }
            Err(GraphError::EdgeAlreadyExists {
                from_id,
                to_id,
                edge_id,
            }) => {
                panic!(
                    "Duplicate Edge found from {:?} to {:?}: {:?}",
                    from_id, to_id, edge_id
                );
            }
            Ok(_) => {}
        }
    }

    for (node_id, node) in graph.nodes() {
        println!("{}: {:?}", node_id, node);
    }
}
