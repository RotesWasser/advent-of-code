use std::{error::Error, collections::{HashSet, HashMap, hash_map::Entry}, hash::Hash, string, vec};

use clap::{App, Arg};
use petgraph::{Graph, Undirected, graph::NodeIndex};

struct Cave {
    name: String,
    is_small: bool
}

impl TryFrom<&str> for Cave {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.chars().fold(true, |acc, c| acc && c.is_ascii_alphabetic()) {
            return Err("Node name not ascii-alphabetic!".into());
        }

        Ok(Self {
            name: value.into(),
            is_small: value.chars().next().unwrap().is_ascii_lowercase()
        })
    }
}

struct CaveSystem {
    graph: Graph<Cave, (), Undirected>,
    start_index: NodeIndex,
    end_index: NodeIndex,
}

impl CaveSystem {
    fn calculate_task_one_paths(&self) -> Vec<String> {
        self.dfs_task_one(self.start_index, &mut HashSet::new())
    }

    fn dfs_task_one(&self, current_idx: NodeIndex, visited_small_caves: &mut HashSet<NodeIndex>) -> Vec<String> {
        let mut found_paths: Vec<String> = vec![];
        let current_data = self.graph.node_weight(current_idx).unwrap();
        let neighbors = self.graph.neighbors(current_idx);

        // we reached the end node, return its name.
        if current_idx == self.end_index {
            return vec![current_data.name.clone()]
        }

        if current_data.is_small {
            visited_small_caves.insert(current_idx);
        }

        // we haven't reached the end node, dfs search!
        for neighbor_idx in neighbors {
            if !visited_small_caves.contains(&neighbor_idx) {
                found_paths.extend(self.dfs_task_one(neighbor_idx, visited_small_caves));
            }
        }

        // we're done visiting, remove our cave again
        if current_data.is_small {
            visited_small_caves.remove(&current_idx);
        }

        // prefix all found paths with our node and return them.
        found_paths.into_iter().map(|suffix| format!("{},{}", current_data.name, suffix).into()).collect()
    }

    fn calculate_task_two_paths(&self) -> Vec<String> {
        self.dfs_task_two(self.start_index, HashSet::new(), None, &mut vec![])
    }

    fn dfs_task_two(&self, 
        current_idx: NodeIndex, 
        visited_small_caves: HashSet<NodeIndex>,
        visited_twice: Option<NodeIndex>,
        current_path: &mut Vec<String>
    ) -> Vec<String> {
        let mut found_paths: Vec<String> = vec![];
        let current_data = self.graph.node_weight(current_idx).unwrap();
        let neighbors = self.graph.neighbors(current_idx);
        

        let mut visited_small_with_current = visited_small_caves.clone();

        current_path.push(current_data.name.clone());
        let path_string: String = current_path.join(",");

        // we reached the end node, return its name.
        if current_idx == self.end_index {
            current_path.pop();
            return vec![path_string]
        }

        if current_data.is_small {
            visited_small_with_current.insert(current_idx);
        }

        // we haven't reached the end node, dfs search!
        for neighbor_idx in neighbors {
            let neighbor_data = self.graph.node_weight(neighbor_idx).unwrap();
            if neighbor_data.is_small {
                // proceed into small cave if it has not yet been visited
                if !visited_small_with_current.contains(&neighbor_idx) {
                    found_paths.extend(self.dfs_task_two(neighbor_idx, visited_small_with_current.clone(), visited_twice, current_path));
                } else {
                    // enter the small cave a second time, but only once.
                    if visited_twice.is_none() && neighbor_idx != self.start_index && neighbor_idx != self.end_index {
                        found_paths.extend(self.dfs_task_two(neighbor_idx, visited_small_with_current.clone(), Some(neighbor_idx), current_path));
                    }
                }
            } else {
                // if the neighbour is a large cave, enter it
                found_paths.extend(self.dfs_task_two(neighbor_idx, visited_small_with_current.clone(), visited_twice, current_path));
            }
        }

        current_path.pop();
        // prefix all found paths with our node and return them.
        found_paths
    }
}

impl TryFrom<&str> for CaveSystem {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut name_to_idx: HashMap<&str, NodeIndex> = HashMap::new();
        let mut graph = Graph::new_undirected();

        for (line_no, line) in value.lines().enumerate() {
            let (a, b) = match line.split_once("-") {
                Some(x) => x,
                None => return Err(format!("Failed to parse line {}", line_no).into()),
            };

            let data_a = Cave::try_from(a)?;
            let data_b = Cave::try_from(b)?;

            let idx_a = match name_to_idx.entry(a) {
                Entry::Occupied(occupied) => {
                    occupied.get().clone()
                },
                Entry::Vacant(vacant) => {
                    vacant.insert(graph.add_node(data_a)).clone()
                },
            };

            let idx_b = match name_to_idx.entry(b) {
                Entry::Occupied(occupied) => {
                    occupied.get().clone()
                },
                Entry::Vacant(vacant) => {
                    vacant.insert(graph.add_node(data_b)).clone()
                },
            };

            graph.update_edge(idx_a, idx_b, ());
        }

        let start_index = match name_to_idx.get("start") {
            Some(x) => x.clone(),
            None => return Err("Failed to find start node!".into())
        };

        let end_index = match name_to_idx.get("end") {
            Some(x) => x.clone(),
            None => return Err("Failed to find end node!".into())
        };

        Ok(Self {
            graph,
            start_index,
            end_index,
        })
    }
}


fn main() {
    let matches = App::new("Advent of Code Day 12")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the input file");

        
    // Parse input into graph
    let cave_system: CaveSystem = input.trim_end().try_into().unwrap();

    // DFS search from start, marking nodes that are lower-case as visited.
    // If end node is reached, emit a path, else backtrack.

    println!("Number of paths through the cave system not visiting small caves twice: {}", 
        cave_system.calculate_task_one_paths().len());
    
    println!("Number of paths through the cave system if one small cave is allowed to be visited twice: {}",
        cave_system.calculate_task_two_paths().len());
}

#[test]
fn test_loading() {
    let cs: CaveSystem = EXAMPLE_SMALL.try_into().unwrap();

    assert_eq!(cs.graph.node_count(), 6);
    assert_eq!(cs.graph.edge_count(), 7);
}

#[test]
fn test_task_one_simple_paths() {
    let cs: CaveSystem = EXAMPLE_SMALL.try_into().unwrap();

    let paths: HashSet<String> = HashSet::from_iter(cs.calculate_task_one_paths());
    let expected_paths: HashSet<String> = HashSet::from_iter(EXAMPLE_SMALL_PATHS_TASK_ONE.lines().map(|x| x.into()));

    assert_eq!(paths.len(), 10);
    assert_eq!(paths, expected_paths);
}

#[test]
fn test_task_two_simple_paths() {
    let cs: CaveSystem = EXAMPLE_SMALL.try_into().unwrap();

    let paths: HashSet<String> = HashSet::from_iter(cs.calculate_task_two_paths());
    let expected_paths: HashSet<String> = HashSet::from_iter(EXAMPLE_SMALL_PATHS_TASK_TWO.lines().map(|x| x.into()));

    assert_eq!(paths.len(), 36);
    assert_eq!(paths, expected_paths);
}

#[cfg(test)]
const EXAMPLE_SMALL: &str = 
r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[cfg(test)]
const EXAMPLE_SMALL_PATHS_TASK_ONE: &str = 
r"start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end";

#[cfg(test)]
const EXAMPLE_SMALL_PATHS_TASK_TWO: &str =
r"start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end";