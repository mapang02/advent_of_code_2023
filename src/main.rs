use std::{collections::{HashMap, HashSet, VecDeque}, io};

fn main() {
    let lines = io::stdin().lines().map(|ln| ln.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let mut nodes: HashMap<String, Vec<String>> = lines.iter().map(|ln| (String::from(&ln[0..3]), Vec::new())).collect();
    for ln in lines {
        let (n, dest) = ln.split_once(": ").unwrap();
        for d in dest.split(" ") {
            if !nodes.contains_key(n) {
                nodes.insert(n.to_string(), Vec::new());
            }
            if !nodes.contains_key(d) {
                nodes.insert(d.to_string(), Vec::new());
            }
            nodes.get_mut(n).unwrap().push(String::from(d));
            nodes.get_mut(d).unwrap().push(String::from(n));
        }
    }

    // Pick one start node, loop over all other nodes as the end node under one is found with a cut set of 3
    let mut node_keys = nodes.keys();
    let start_node = node_keys.next().unwrap();
    for end_node in node_keys {
        // Find a maximum flow
        let max_flow = max_flow(&nodes, &start_node, &end_node);
    
        // Remove all saturated edges, determine which nodes are reachable from start node using BFS
        let mut unsaturated_graph = nodes.clone();
        for (source, dest) in max_flow {
            let source_edges = unsaturated_graph.get_mut(source).unwrap();
            for i in 0..source_edges.len() {
                if source_edges[i] == *dest {
                    source_edges.remove(i);
                    break;
                }
            }
            let dest_edges = unsaturated_graph.get_mut(dest).unwrap();
            for i in 0..dest_edges.len() {
                if dest_edges[i] == *source {
                    dest_edges.remove(i);
                    break;
                }
            }
        }

        let mut reachable: HashMap<&String, bool> = nodes.keys().map(|n| (n, false)).collect();
        reachable.insert(&start_node, true);
        let mut bfs_queue = VecDeque::from([start_node]);
        while let Some(curr_node) = bfs_queue.pop_front() {
            for dest in unsaturated_graph.get(curr_node).unwrap() {
                if !reachable.get(dest).unwrap() {
                    reachable.insert(dest, true);
                    bfs_queue.push_back(dest);
                }
            }
        }
    
        // Edges going from nodes in reachable group to unreachable group are cut edges
        let mut num_cut_edges = 0;
        for (source, outgoing_edges) in nodes.iter() {
            if *reachable.get(source).unwrap() {
                for dest in outgoing_edges {
                    if !(*reachable.get(dest).unwrap()) {
                        //println!("({source}, {dest}) is a cut edge");
                        num_cut_edges += 1;
                    }
                }
            }
        }

        // If there are exactly 3 edges in the cut, return the product of group sizes
        if num_cut_edges == 3 {
            let num_reachable = reachable.values().filter(|r| **r).count();
            let num_unreachable = reachable.values().filter(|r| !(**r)).count();
            return (num_reachable * num_unreachable) as u32;
        }
    }

    return 0;
}

// Determine max flow using Edmonds-Karp algorithm
fn max_flow<'a>(graph: &'a HashMap<String, Vec<String>>, start_node: &'a String, end_node: &'a String) -> HashSet<(&'a String, &'a String)> {
    let mut flow: HashSet<(&String, &String)> = HashSet::new();

    // Repeatedly add shortest path using BFS
    loop {
        // Find shortest path using BFS
        let mut predecessors = HashMap::from([(start_node, start_node)]); // key is the node, value is the predecessor in current BFS iteration
        let mut bfs_queue = VecDeque::from([start_node]);
        while let Some(curr_node) = bfs_queue.pop_front() {
            if curr_node == end_node {
                break;
            }
            let outgoing_edges = graph.get(curr_node).unwrap();
            for o in outgoing_edges {
                if !flow.contains(&(curr_node, o)) && !predecessors.contains_key(o) {
                    predecessors.insert(o, curr_node);
                    bfs_queue.push_back(o);
                }
            }
        }

        // Modify flow based on new path
        if predecessors.contains_key(end_node) {
            let mut cursor = end_node;
            while cursor != start_node {
                let pred = *predecessors.get(cursor).unwrap();
                if flow.contains(&(cursor, pred)) { // Residual flow, reverse direction of edge
                    flow.remove(&(cursor, pred));
                }
                else { // Add edge to flow normally
                    flow.insert((pred, cursor));
                }
                cursor = pred;
            }
        }
        else { // No path to end node, max flow found
            break;
        }
    }
    return flow;
}

fn part2(lines: &Vec<String>) -> u32 {
    return 0;
}
