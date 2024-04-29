// breathfirstsearch.rs
use crate::graph::{Graph, Vertex};
use std::collections::{VecDeque, HashMap};

pub fn bfs_distance(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distance = vec![None; graph.n];
    let mut queue = VecDeque::new();

    distance[start] = Some(0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(current_distance) = distance[current] {
            for &neighbor in &graph.outedges[current] {
                if distance[neighbor].is_none() {
                    distance[neighbor] = Some(current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distance
}

fn filtered_distances(graph: &Graph, degree: u32) -> Vec<Vec<Option<u32>>> {
    graph.outedges.iter().enumerate().map(|(i, _)| {
        bfs_distance(i, graph)
            .into_iter()
            .map(|d| if let Some(val) = d {
                if val <= degree && val > 0 {
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            })
            .collect()
    }).collect()
}

pub fn average_distance(graph: &Graph, degree: u32) -> f64 {
    let distances = filtered_distances(graph, degree);
    let (mut total_distance, mut valid_pairs) = (0, 0);

    for row in distances {
        for d in row {
            if let Some(val) = d {
                total_distance += val;
                valid_pairs += 1;
            }
        }
    }

    if valid_pairs > 0 {
        (total_distance as f64) / (valid_pairs as f64)
    } else {
        0.0
    }
}

pub fn max_distance(graph: &Graph, degree: u32) -> u32 {
    let distances = filtered_distances(graph, degree);
    let mut max_distance = 0;

    for row in distances {
        for d in row {
            if let Some(val) = d {
                if val > max_distance {
                    max_distance = val;
                }
            }
        }
    }

    max_distance
}

pub fn mode_distance(graph: &Graph, degree: u32) -> u32 {
    let distances = filtered_distances(graph, degree);
    let mut dist_frequency = HashMap::new();

    for row in distances {
        for d in row {
            if let Some(val) = d {
                *dist_frequency.entry(val).or_insert(0) += 1;
            }
        }
    }

    *dist_frequency.iter().max_by_key(|(_, &count)| count).unwrap().0
}

pub fn distribution_percentage(graph: &Graph, degree: u32) -> f64 {
    let distances = filtered_distances(graph, degree);
    let (mut total_pairs, mut valid_pairs) = (0, 0);

    for row in distances {
        for d in row {
            total_pairs += 1;
            if d.is_some() {
                valid_pairs += 1;
            }
        }
    }

    if total_pairs > 0 {
        (valid_pairs as f64 / total_pairs as f64) * 100.0
    } else {
        0.0
    }
}
