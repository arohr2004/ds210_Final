use std::fs::File; //handling files 
use csv::ReaderBuilder; //reading csv files 
use std::error::Error; //handling errors 

pub type Vertex = usize; //vertex is identified by a unique index
pub type ListOfEdges = Vec<(Vertex, Vertex)>; // a list of edges represented as pairs of vertices
pub type AdjacencyLists = Vec<Vec<Vertex>>; // A graph represented as adjacency lists

#[derive(Debug)] // Enables automatic generation of Debug trait for Graph struct
pub struct Graph {
    pub n: usize, // # of vertices
    pub outedges: AdjacencyLists, // Shows outgoing edges from each vertex
}

impl Graph {
    // Read CSV and create undirected graph
    pub fn read_csv(file_path: &str, n: usize) -> Result<Self, Box<dyn Error>> {
        let mut edges = Vec::new(); // Vector for storing edges
        let file = File::open(file_path)?; // Open the CSV file, if not able to creates an error
        let mut rdr = ReaderBuilder::new().from_reader(file); // Initialize CSV reader

        for result in rdr.records() { // Iterate through each record in the CSV
            let record = result?; 
            let vertices: Vec<usize> = record.iter() // divide into a list of integers
                .map(|s| s.parse::<usize>().unwrap())  // converts each string into a usize
                .collect(); //collects the divided integers into a vector 
            edges.push((vertices[0] - 1, vertices[1] - 1)); // Convert to 0-based index
            edges.push((vertices[1] - 1, vertices[0] - 1)); // Add reverse edge for undirected graph
        }

        let graph = Graph::undirected(n, &edges); // Create undirected graph
        Ok(graph) // return the graph
    }

    // create undirected graph from the list of edges 
    pub fn undirected(n: usize, edges: &ListOfEdges) -> Self {
        let mut graph = Graph { n, outedges: vec![vec![]; n] }; // Initialize with empty adjacency lists

        // Add each edge to the graph
        for (u, v) in edges {
            graph.outedges[*u].push(*v); // adds an edge from u to v
            graph.outedges[*v].push(*u); // adds an edge from v to u (for undirected)
        }

        graph.sort(); // sort adjacency lists 
        graph // return the created graph
    }

    // Sort adjacency lists
    fn sort(&mut self) {
        for edges in &mut self.outedges {
            edges.sort(); // Sort the lists
        }
    }
}
