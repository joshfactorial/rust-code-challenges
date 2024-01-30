use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

// from the solution. A new struct to help track the path
#[derive(Clone, Eq, PartialEq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Step {
    fn new(position: Node, cost: Cost, history: Vec<Node>) -> Self {
        Step {
            cost,
            position,
            history,
        }
    }

    fn from_start(start: Node) -> Self {
        Step {
            cost: 0,
            position: start,
            history: vec![],
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

    // Here they are building the default dist map. Similar to my distance_from_source
    let mut dist: HashMap<Node, Cost> = g.nodes
        .iter()
        .map(|&x| if x == start { (x, 0) } else { (x, usize::MAX) })
        .collect();

    // Not sure what a binary heap does
    let mut priority_queue = BinaryHeap::new();

    // looks like this will track the steps to the goal
    priority_queue.push(Step::from_start(start));

    while let Some(Step {cost, position, mut history}) = priority_queue.pop() {
        // So this stops if the nodes connect directly, which could be the right answer, but idk. What if that particular
        // path has a very high cost, then don't we risk a higher-node count direct path? But maybe that's okay.
        if position == goal {
            history.push(goal);
            return Some((history, cost));
        }

        // this looks similar to my finding neighbors u of v below
        if let Some(destinations) = &g.edges.get(&position) {
            for &(next_destination, next_cost) in destinations.iter() {
                if next_cost < dist[&next_destination] {
                    let mut next = Step::new(next_destination, cost+next_cost, history.clone());
                    next.history.push(position);
                    priority_queue.push(next);

                    if let Some(old_cost) = dist.get_mut(&next_destination) {
                        *old_cost = next_cost
                    }
                }
            }
        }
    }

    None

    // // If neither the start nor the goal are in the Nodes list, no need to process.
    // if !g.nodes.contains(&start) || !(g.nodes.contains(&goal)) {
    //     return None;
    // }
    //
    // // initialize with distance from source to source = 0
    // let mut distance_from_source: HashMap<Node, Cost> = HashMap::from([(start, 0)]);
    // let mut nodes_traversed: Vec<Node> = vec![];
    // let mut nodes_of_interest: HashSet<Node> = HashSet::new();
    // let mut removed_nodes: HashSet<Node> = HashSet::new();
    //
    // // pseudocode:
    // // for each vertex v in Graph:
    // //    if v != source
    // //        dist[v] := infinity
    // //    add v to Q
    //
    // // note that Q is nodes_of_interest. dist[x] is distance_from_source
    // for &vertex in &g.nodes {
    //     if vertex != start {
    //         // initialize to infinity
    //         distance_from_source.entry(vertex).or_insert(usize::MAX);
    //     }
    //     nodes_of_interest.insert(vertex);
    // }
    //
    // // Initialize the check_node parameter. I'm still confused on if this gets updated
    // let mut check_node: Node = start;
    //
    // // psuedocode:
    // // while Q [nodes_of_interest] is not empty:
    // //     v:= vertex in Q with min dist[v]
    // //     remove v from Q
    // //     for each neighbor u of v [such that u has not been removed from Q]:
    // //          alt := dist[v] + length(v, u)
    // //          if alt < dist[u]:
    // //              dist[u] := alt
    // while !nodes_of_interest.is_empty() {
    //
    //     // v := vertex in Q with min dist[v]
    //     // first, we need to identify which node has min dist[v]
    //
    //     // search for the node with the minimum distance to the current source
    //     // Note that in the first iteration, this will be the start node.
    //     let mut temp_min = usize::MAX;
    //     for &vertex in &nodes_of_interest {
    //         if distance_from_source[&vertex] < temp_min {
    //             temp_min = distance_from_source[&vertex];
    //             check_node = vertex;
    //         }
    //     }
    //     // note: v = check_node
    //
    //     // remove v from Q
    //     nodes_of_interest.remove(&check_node);
    //     // trying to see if I'm doing the next part right
    //     removed_nodes.insert(check_node);
    //
    //
    //     //     for each neighbor u of v [such that u has not been removed from Q]:
    //     // first determine neighbors of check_node that have not been removed from Q.
    //     let mut neighbors_of_v: HashSet<Node> = HashSet::new();
    //     {
    //         for &(neighbor, _) in &g.edges[&check_node] {
    //             if removed_nodes.contains(&neighbor) {
    //                 // skip any we have removed. Note sure this is necessary, but maybe there's a subtlty I'm missing
    //                 continue;
    //             } else {
    //                 neighbors_of_v.insert(neighbor);
    //             }
    //         }
    //     }
    //
    //     // Now we examine all remaining neighbors, looking for the shortest distance
    //     for &node in &neighbors_of_v {
    //         // gather the edges to find the minimum
    //         let mut length_node_to_check: usize = usize::MAX;
    //         for (key, value) in &g.edges[&check_node] {
    //             if key == &node {
    //                 if value < &length_node_to_check {
    //                     length_node_to_check = *value;
    //                 }
    //                 break
    //             }
    //         }
    //
    //         let alt = distance_from_source[&check_node] + length_node_to_check;
    //         if alt < distance_from_source[&node] {
    //             distance_from_source.entry(node).and_modify(|x| *x = alt);
    //         }
    //     }
    // }

}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(
            &g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    // let edge_list = include!("large_graph.in");
    let edge_list = vec!((1,2,1), (1,3,2), (2,1,1), (2,4,5), (3,1,2), (3,4,2), (4,2,5), (4,3,2));
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1, 4);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 4);
}
