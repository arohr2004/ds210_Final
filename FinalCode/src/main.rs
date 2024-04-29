// main.rs
mod graph;
mod breathfirstsearch;

use graph::Graph;
use breathfirstsearch::{
    average_distance,
    max_distance,
    mode_distance,
    distribution_percentage,
};

fn main() {
    let file_path = "euroroad.csv";
    let n = 1174;

    let graph = match Graph::read_csv(file_path, n) {
        Ok(graph) => graph,
        Err(e) => {
            eprintln!("Error creating graph: {}", e);
            return;
        },
    };
  //  graph.print(); // This will print the graph to the console
    for i in 1..=30 {
        println!("{} degrees of separation:", i);
        println!("Average distance: {}", average_distance(&graph, i));
        println!("Maximum distance: {}", max_distance(&graph, i));
        println!("Most commonly occurring distance: {}", mode_distance(&graph, i));
        println!("Percentage of vertex pairs with distances not 0 and below {i} degrees of separation: {}%", distribution_percentage(&graph, i));
        println!();
    }
}


