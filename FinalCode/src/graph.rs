use std::fs::File;
use csv::ReaderBuilder;
use std::error::Error;
//use std::fmt;

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    // Read CSV and create undirected graph
    pub fn read_csv(file_path: &str, n: usize) -> Result<Self, Box<dyn Error>> {
        let mut edges = Vec::new(); // Vector for storing edges
        let file = File::open(file_path)?; // Open the CSV file
        let mut rdr = ReaderBuilder::new().from_reader(file); // Initialize CSV reader

        for result in rdr.records() { // Iterate through each record in the CSV
            let record = result?; // Handle the record
            let vertices: Vec<usize> = record.iter() // Parse into integers
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            edges.push((vertices[0] - 1, vertices[1] - 1)); // Convert to 0-based index
            edges.push((vertices[1] - 1, vertices[0] - 1)); // Add reverse edge for undirected graph
        }

        let graph = Graph::undirected(n, &edges); // Create undirected graph
        Ok(graph) // Return the graph
    }

    // Create undirected graph
    pub fn undirected(n: usize, edges: &ListOfEdges) -> Self {
        let mut graph = Graph { n, outedges: vec![vec![]; n] }; // Initialize with empty adjacency lists

        // Add edges to create the undirected graph
        for (u, v) in edges {
            graph.outedges[*u].push(*v); // Add first direction
            graph.outedges[*v].push(*u); // Add reverse direction
        }

        graph.sort(); // Sort adjacency lists for consistency
        graph // Return the constructed graph
    }

    // Sort adjacency lists
    fn sort(&mut self) {
        for edges in &mut self.outedges {
            edges.sort(); // Sort the lists
        }
    }

    // Custom print method to display the graph
  //  pub fn print(&self) {
  //      for (vertex, neighbors) in self.outedges.iter().enumerate() {
  //          println!("Vertex {}: {:?}", vertex, neighbors); // Output vertex and its neighbors
   //     }
 //   }
}
