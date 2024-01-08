use std::{
    fmt::Display,
    fmt::Debug,
    collections::{ HashMap, VecDeque, HashSet, BinaryHeap },
    hash::Hash,
    path::Path,
};

pub struct SimpleGraphe<T: Display + Clone + PartialEq + Eq + Hash + Copy> {
    // Number of nodes
    size: usize,
    // Nodes connections
    edges: HashMap<T, Vec<T>>,
}

impl<T: Display + Clone + Copy + PartialEq + Eq + Hash> SimpleGraphe<T> {
    // Init a new graph from the list of directed connections
    // If a connection includes (a,b), we will create only the edge from a to b, not both.
    pub fn new(size: usize, connections: &Vec<(T, T)>, directed_graphe: bool) -> SimpleGraphe<T> {
        let mut edges: HashMap<T, Vec<T>> = HashMap::new();

        for (a, b) in connections {
            if edges.contains_key(a) {
                edges.get_mut(a).unwrap().push(*b);
            } else {
                edges.insert(*a, vec![*b]);
            }

            if !directed_graphe {
                if edges.contains_key(b) {
                    edges.get_mut(b).unwrap().push(*a);
                } else {
                    edges.insert(*b, vec![*a]);
                }
            }
        }

        return SimpleGraphe {
            size: size,
            edges: edges,
        };
    }

    pub fn display(&self) {
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        for current_node in self.edges.keys() {
            for n in self.edges.get(current_node).unwrap() {
                println!("{current_node} -> {n}");
            }
        }
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    }

    /**
     * Function will browse the whole graph and display the graph values (or indexes) as they are searched
     */
    pub fn depth_first_print(&self, start_node: T) {
        let mut node_stack: Vec<T> = vec![];
        node_stack.push(start_node);
        while node_stack.len() > 0 {
            let current_node = node_stack.pop().unwrap();
            print!("{} ", current_node);

            // Then stacking up the neighbours
            if let Some(neighbours) = self.edges.get(&current_node) {
                for neighbour in neighbours {
                    node_stack.push(*neighbour);
                }
            }
        }
        println!("");
    }

    pub fn breadth_first_print(&self, start_node: T) {
        let mut node_queue: VecDeque<T> = VecDeque::new();
        node_queue.push_back(start_node);
        while node_queue.len() > 0 {
            let current_node = node_queue.pop_front().unwrap();
            print!("{} ", current_node);

            if let Some(neighbours) = self.edges.get(&current_node) {
                for neighbour in neighbours {
                    node_queue.push_back(*neighbour);
                }
            }
        }
        println!("")
    }
}

pub struct DirectedWeightedGraphe<T: Eq + PartialEq + Hash + Copy + Display + Debug> {
    // number of edges
    size: usize,
    // Each edge has added a distance
    edges: HashMap<T, Vec<(T, i32)>>,
}

impl<T: PartialEq + Hash + Eq + Copy + Display + Debug> DirectedWeightedGraphe<T> {
    pub fn new(
        size: usize,
        connections: &Vec<(T, T, i32)>,
        addOtherDirection: bool
    ) -> DirectedWeightedGraphe<T> {
        let mut edges: HashMap<T, Vec<(T, i32)>> = HashMap::new();

        for (a, b, c) in connections {
            if edges.contains_key(a) {
                edges.get_mut(a).unwrap().push((*b, *c));
            } else {
                edges.insert(*a, vec![(*b, *c)]);
            }
            if addOtherDirection {
                if edges.contains_key(b) {
                    edges.get_mut(b).unwrap().push((*a, *c));
                } else {
                    edges.insert(*b, vec![(*a, *c)]);
                }
            }
        }

        return DirectedWeightedGraphe {
            size,
            edges: edges,
        };
    }

    pub fn display(&self) {
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        for source_node in self.edges.keys() {
            let current_edges = self.edges.get(source_node).unwrap();
            for (neighbour, distance) in current_edges {
                println!("{source_node} -=- ({distance}) -=-> {neighbour}");
            }
        }
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    }

    pub fn simple_shortest_path(&self, source: &T, destination: &T) {
        // will go breadth first traverse
        let mut queue: VecDeque<(&T, i32, Vec<&T>)> = VecDeque::new();
        queue.push_back((source, 0, vec![]));
        let mut visited: HashSet<&T> = HashSet::new();

        while queue.len() > 0 {
            let (current_node, current_distance, current_path) = queue.pop_front().unwrap();
            if *current_node == *destination {
                // Found the shortest path
                println!("Shortest path is {} long, through {:?}", current_distance, current_path);
                return;
            }
            visited.insert(current_node);

            if let Some(edge_list) = self.edges.get(current_node) {
                for (neighbour, distance) in edge_list {
                    if !visited.contains(neighbour) {
                        let mut new_path = current_path.clone();
                        new_path.push(neighbour);
                        queue.push_back((neighbour, current_distance + *distance, new_path));
                    }
                }
            }
        }
        println!("Shortest distance not found!");
    }
}

// Structure internal to this module, created to store path information for Djikstra's algorithm
// It is necessary to create a structure so that we can adapt the BinaryHeap to the algorithm's needs

#[derive(Eq, PartialEq, Debug)]
struct PathData<T> {
    node: T,
    cost: i32,
}

impl<T: PartialEq + Ord> PartialOrd for PathData<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(other.cmp(self));
    }
}

impl<T: Eq + Ord> Ord for PathData<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.cost.cmp(&other.cost);
    }
}

impl<T: PartialEq + Hash + Eq + Copy + Display + Debug + Ord> DirectedWeightedGraphe<T> {
    /**
     * Djikstra's shortest path algorithm
     * Returns the HashMap containing the min distance for all the nodes
     */
    pub fn djikstra_shortest_path(&self, source: &T) -> HashMap<T, i32> {
        let mut visited: HashSet<T> = HashSet::new();
        let mut min_distance: HashMap<T, i32> = HashMap::new();

        let mut min_heap: BinaryHeap<PathData<T>> = BinaryHeap::new();

        min_heap.push(PathData { node: *source, cost: 0 });

        while min_heap.len() > 0 {            
            let current_path_data = min_heap.pop().unwrap();
            if !visited.contains(&current_path_data.node) {
                visited.insert(current_path_data.node);
                min_distance.insert(current_path_data.node, current_path_data.cost);
    
                if let Some(neighbors_list) = self.edges.get(&current_path_data.node) {
                    for (neighbor, distance) in neighbors_list {
                        if !visited.contains(neighbor) {
                            min_heap.push(PathData {
                                node: *neighbor,
                                cost: current_path_data.cost + *distance,
                            });
                        }
                    }
                }
            }
        }
        return min_distance;
    }
}
