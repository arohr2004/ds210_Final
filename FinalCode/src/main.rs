// main.rs
mod graph; // Declare the modules for Graph and BFS functions
mod breathfirstsearch;

use graph::Graph; // Import the graph struct and functions from the BFS module
use breathfirstsearch::{
    average_distance,
    max_distance,
    mode_distance,
    distribution_percentage,
};

fn main() {
    let file_path = "euroroad.csv"; // path to the CSV file 
    let n = 1174; //number of vertices in the graph

    //create graph from the CSV file
    let graph = match Graph::read_csv(file_path, n) { 
        Ok(graph) => graph, // graph creation successful
        Err(e) => { // handles error in reading the CSV file
            eprintln!("Error creating graph: {}", e);
            return; // if needed, exit early due to error
        },
    };
// Loop through degrees of separation from 1 to 30
    for i in 1..=30 {
        println!("{} degrees of separation:", i); // show current degree of separation
        // show various stat for the graph with i number degrees of separation
        println!("Average distance: {}", average_distance(&graph, i));
        println!("Maximum distance: {}", max_distance(&graph, i));
        println!("Most commonly occurring distance: {}", mode_distance(&graph, i));
        println!("Percentage of vertex pairs with distances not 0 and below {i} degrees of separation: {}%", distribution_percentage(&graph, i));
        println!(); // print a newline for readability
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*; // import the main module's symbols to use in tests
    use graph::Graph; // use the graph struct for test 
    use breathfirstsearch::{average_distance, max_distance, mode_distance, distribution_percentage};

    // Create a small sample graph for testing 
    fn sample_graph() -> Graph {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        Graph::undirected(5, &edges) // construct the graph from the edges
    }

    // test average distance calculation
    #[test]
    fn test_average_distance() {
        let graph = sample_graph();
        assert_eq!(average_distance(&graph, 1), 1.0); // validate the expected result
    }

    // test maximum distance calculation
    #[test]
    fn test_max_distance() {
        let graph = sample_graph();
        assert_eq!(max_distance(&graph, 2), 2); // validate if the maximum distance
    }

    // test the mode distance calculation
    #[test]
    fn test_mode_distance() {
        let graph = sample_graph();
        assert_eq!(mode_distance(&graph, 2), 1); // check that the mode distance is correct
    }

    // test the distribution percentage calculation
    #[test]
    fn test_distribution_percentage() {
        let graph = sample_graph();
        assert_eq!(distribution_percentage(&graph, 4), 80.0); //all nodes below 4 degrees of separation
    }
}


