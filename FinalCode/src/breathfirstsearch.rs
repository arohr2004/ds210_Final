// breathfirstsearch.rs
use crate::graph::{Graph, Vertex}; // importing Graph and Vertex from the graph.rs
use std::collections::{VecDeque, HashMap}; // importing data structures for queue and hashmap operations

pub fn bfs_distance(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distance = vec![None; graph.n]; // creates a vector to store distances from start vertex
    let mut queue = VecDeque::new(); // double queue for breathfirstsearch queue

    distance[start] = Some(0); // distance from the start vertex to the same is 0
    queue.push_back(start); // start the BFS from the stated 'start' vertex

    while let Some(current) = queue.pop_front() { // continue BFS until the queue is empty
        if let Some(current_distance) = distance[current] { // find the distance to the current vertex
            for &neighbor in &graph.outedges[current] { // check all adjacent vertices
                if distance[neighbor].is_none() { // use the neighbor hasn't been visited
                    distance[neighbor] = Some(current_distance + 1); // update the distance
                    queue.push_back(neighbor); // add to the queue
                }
            }
        }
    }

    distance // return the vector of distances from the start vertex
}

// For each vertex, calculate distances using BFS, and then filter by the degree number
fn filtered_distances(graph: &Graph, degree: u32) -> Vec<Vec<Option<u32>>> {
    graph.outedges.iter().enumerate().map(|(i, _)| {
        bfs_distance(i, graph)// Get distances from vertex 'i' 
            .into_iter() //iterate over distances
            .map(|d| if let Some(val) = d { // Apply a filter
                if val <= degree && val > 0 { // keep distances only within 'degree' and > 0 (legit or valid distances)
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            })
            .collect() // put filtered results into a list
    }).collect() // collect results for each vertex into a vector of vectors
}

pub fn average_distance(graph: &Graph, degree: u32) -> f64 {
    let distances = filtered_distances(graph, degree); // get filtered distances
    let (mut total_distance, mut valid_pairs) = (0, 0); // variables for sum and count of legit distances

    for row in distances { // go through the filtered legit distances
        for d in row {
            if let Some(val) = d { //if the distance is valid 
                total_distance += val; //add to the total distance 
                valid_pairs += 1; //add to the total count of valid pairs 
            }
        }
    }

    if valid_pairs > 0 { // ff there are valid distances
        (total_distance as f64) / (valid_pairs as f64) // calculate the average
    } else {
        0.0 // if there are no valid pairs, return 0 as average
    }
}

pub fn max_distance(graph: &Graph, degree: u32) -> u32 {
    let distances = filtered_distances(graph, degree); //get the filtered distances 
    let mut max_distance = 0; //tracking the max distance

    for row in distances { // iterate through the filtered distances
        for d in row {
            if let Some(val) = d { // if there's a valid distance
                if val > max_distance { // check if it's more than current max
                    max_distance = val; //then update the max distance 
                }
            }
        }
    }

    max_distance //return max distance found 
}

pub fn mode_distance(graph: &Graph, degree: u32) -> u32 {
    let distances = filtered_distances(graph, degree); // Get filtered distances
    let mut dist_frequency = HashMap::new(); // HashMap for tracking frequency of distances

    for row in distances { // iterate through filtered distances
        for d in row {
            if let Some(val) = d { // if there's a valid distance
                *dist_frequency.entry(val).or_insert(0) += 1; //increase the frequency
            }
        }
    }

    *dist_frequency.iter().max_by_key(|(_, &count)| count).unwrap().0 // Find the most frequent distance
}

pub fn distribution_percentage(graph: &Graph, degree: u32) -> f64 {
    let distances = filtered_distances(graph, degree); // get filtered distances
    let (mut total_pairs, mut valid_pairs) = (0, 0); // variables for tracking pairs

    for row in distances { // iterate through filtered distances
        for d in row {
            total_pairs += 1; // increase the count of total pairs by one
            if d.is_some() { // if there's a valid distance, increase the count of valid pairs 
                valid_pairs += 1;
            }
        }
    }

    if total_pairs > 0 { // find the percentage if there are pairs
        (valid_pairs as f64 / total_pairs as f64) * 100.0 // return the %
    } else {
        0.0 // If no pairs, return 0
    }
}
