use std::{io, vec};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 1: {part2}");
}

const NO_UP: usize = 0;
const NO_DOWN: usize = 1;
const NO_LEFT: usize = 2;
const NO_RIGHT: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DistRecord {
    cost: usize,
    node: (usize, usize, usize)
}

impl Ord for DistRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node));
    }
}

impl PartialOrd for DistRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn part1(lines: &Vec<String>) -> usize {
    // Set up nodes and paths
    let costs: Vec<Vec<usize>> = lines.iter().map(|l| l.chars().map(|c| (c as usize) - 48).collect()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut neighbors: Vec<Vec<Vec<Vec<((usize, usize, usize), usize)>>>> = vec![vec![vec![Vec::new(); num_cols]; num_rows]; 4];
    for path_limit in 0..4 {
        for row_num in 0..num_rows {
            for col_num in 0..num_cols {
                // Up neighbors
                if path_limit != NO_UP && path_limit != NO_DOWN {
                    for offset in 1..4 {
                        if row_num >= offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num - o][col_num]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_UP, row_num - offset, col_num), path_cost));
                        }
                    }
                }
                // Down neighbors
                if path_limit != NO_UP && path_limit != NO_DOWN {
                    for offset in 1..4 {
                        if row_num < num_rows - offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num + o][col_num]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_DOWN, row_num + offset, col_num), path_cost));
                        }
                    }
                }
                // Left neighbors
                if path_limit != NO_LEFT && path_limit != NO_RIGHT {
                    for offset in 1..4 {
                        if col_num >= offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num][col_num - o]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_LEFT, row_num, col_num - offset), path_cost));
                        }
                    }
                }
                // Right neighbors
                if path_limit != NO_LEFT && path_limit != NO_RIGHT {
                    for offset in 1..4 {
                        if col_num < num_cols - offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num][col_num + o]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_RIGHT, row_num, col_num + offset), path_cost));
                        }
                    }
                }
            }
        }
    }
    // Debug printing
    for path_limit in 0..4 {
        //println!("Path Limit: {}", path_limit);
        for row_num in 0..num_rows {
            for col_num in 0..num_cols {
                //println!("({}, {}): {:?}", row_num, col_num, neighbors[path_limit][row_num][col_num]);
            }
        }
    }

    // Search for shortest path with Djikstra
    let mut tentative_costs = vec![vec![vec![std::usize::MAX; num_cols]; num_rows]; 4];
    let mut prev_nodes = vec![vec![vec![(std::usize::MAX, std::usize::MAX, std::usize::MAX); num_cols]; num_rows]; 4];
    let mut node_heap = BinaryHeap::new();
    tentative_costs[NO_UP][0][0] = 0;
    node_heap.push(DistRecord { cost: 0, node: (NO_UP, 0, 0) }); // Start at (NO_UP, 0, 0)
    while node_heap.len() > 0 {
        let dist_record = node_heap.pop().unwrap();
        let curr_node_cost = dist_record.cost;
        let curr_node = dist_record.node;
        //println!("Visiting {:?}, cost = {}", curr_node, curr_node_cost);
        for (neighbor, path_cost) in &neighbors[curr_node.0][curr_node.1][curr_node.2] {
            //println!("Distance to {:?} is {}", neighbor, curr_node_cost + path_cost);
            if curr_node_cost + path_cost < tentative_costs[neighbor.0][neighbor.1][neighbor.2] {
                tentative_costs[neighbor.0][neighbor.1][neighbor.2] = curr_node_cost + path_cost;
                prev_nodes[neighbor.0][neighbor.1][neighbor.2] = curr_node;
                node_heap.push(DistRecord { cost: curr_node_cost + path_cost, node: *neighbor});
            }
        }
    }

    // Debug print cost grid
    for row_num in 0..num_rows {
        for col_num in 0..num_cols {
            let min_cost = (0..4).map(|path_limit| tentative_costs[path_limit][row_num][col_num]).min().unwrap();
            //print!("{:4}", min_cost);
        }
        //println!("");
    }

    // Debug print path
    let mut min_cost_end_dir = NO_UP;
    for path_limit in 1..4 {
        if tentative_costs[path_limit][num_rows - 1][num_cols - 1] < tentative_costs[min_cost_end_dir][num_rows - 1][num_cols - 1] {
            min_cost_end_dir = path_limit
        }
    }
    let mut path = Vec::new();
    let mut path_trace = (min_cost_end_dir, num_rows - 1, num_cols - 1);
    while !(path_trace.1 == 0 && path_trace.2 == 0) {
        path.insert(0, path_trace);
        path_trace = prev_nodes[path_trace.0][path_trace.1][path_trace.2];
    }
    for node in path {
        println!("{:?}", node);
    }

    // Find shortest path to bottom right value (from any direction)
    return (0..4).map(|path_limit| tentative_costs[path_limit][num_rows - 1][num_cols - 1]).min().unwrap();
}

fn part2(lines: &Vec<String>) -> usize {
    // Set up nodes and paths
    let costs: Vec<Vec<usize>> = lines.iter().map(|l| l.chars().map(|c| (c as usize) - 48).collect()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut neighbors: Vec<Vec<Vec<Vec<((usize, usize, usize), usize)>>>> = vec![vec![vec![Vec::new(); num_cols]; num_rows]; 4];
    for path_limit in 0..4 {
        for row_num in 0..num_rows {
            for col_num in 0..num_cols {
                // Up neighbors
                if path_limit != NO_UP && path_limit != NO_DOWN {
                    for offset in 4..11 {
                        if row_num >= offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num - o][col_num]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_UP, row_num - offset, col_num), path_cost));
                        }
                    }
                }
                // Down neighbors
                if path_limit != NO_UP && path_limit != NO_DOWN {
                    for offset in 4..11 {
                        if num_rows > offset && row_num < num_rows - offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num + o][col_num]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_DOWN, row_num + offset, col_num), path_cost));
                        }
                    }
                }
                // Left neighbors
                if path_limit != NO_LEFT && path_limit != NO_RIGHT {
                    for offset in 4..11 {
                        if col_num >= offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num][col_num - o]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_LEFT, row_num, col_num - offset), path_cost));
                        }
                    }
                }
                // Right neighbors
                if path_limit != NO_LEFT && path_limit != NO_RIGHT {
                    for offset in 4..11 {
                        if num_cols > offset && col_num < num_cols - offset {
                            let path_cost: usize = (1..(offset + 1)).map(|o| costs[row_num][col_num + o]).sum();
                            neighbors[path_limit][row_num][col_num].push(((NO_RIGHT, row_num, col_num + offset), path_cost));
                        }
                    }
                }
            }
        }
    }
    // Debug printing
    for path_limit in 0..4 {
        //println!("Path Limit: {}", path_limit);
        for row_num in 0..num_rows {
            for col_num in 0..num_cols {
                //println!("({}, {}): {:?}", row_num, col_num, neighbors[path_limit][row_num][col_num]);
            }
        }
    }

    // Search for shortest path with Djikstra
    let mut tentative_costs = vec![vec![vec![std::usize::MAX; num_cols]; num_rows]; 4];
    let mut prev_nodes = vec![vec![vec![(std::usize::MAX, std::usize::MAX, std::usize::MAX); num_cols]; num_rows]; 4];
    let mut node_heap = BinaryHeap::new();
    tentative_costs[NO_UP][0][0] = 0;
    node_heap.push(DistRecord { cost: 0, node: (NO_UP, 0, 0) }); // Start at (NO_UP, 0, 0)
    while node_heap.len() > 0 {
        let dist_record = node_heap.pop().unwrap();
        let curr_node_cost = dist_record.cost;
        let curr_node = dist_record.node;
        //println!("Visiting {:?}, cost = {}", curr_node, curr_node_cost);
        for (neighbor, path_cost) in &neighbors[curr_node.0][curr_node.1][curr_node.2] {
            //println!("Distance to {:?} is {}", neighbor, curr_node_cost + path_cost);
            if curr_node_cost + path_cost < tentative_costs[neighbor.0][neighbor.1][neighbor.2] {
                tentative_costs[neighbor.0][neighbor.1][neighbor.2] = curr_node_cost + path_cost;
                prev_nodes[neighbor.0][neighbor.1][neighbor.2] = curr_node;
                node_heap.push(DistRecord { cost: curr_node_cost + path_cost, node: *neighbor});
            }
        }
    }

    // Debug print cost grid
    for row_num in 0..num_rows {
        for col_num in 0..num_cols {
            let min_cost = (0..4).map(|path_limit| tentative_costs[path_limit][row_num][col_num]).min().unwrap();
            //print!("{:4}", min_cost);
        }
        //println!("");
    }

    // Debug print path
    let mut min_cost_end_dir = NO_UP;
    for path_limit in 1..4 {
        if tentative_costs[path_limit][num_rows - 1][num_cols - 1] < tentative_costs[min_cost_end_dir][num_rows - 1][num_cols - 1] {
            min_cost_end_dir = path_limit
        }
    }
    let mut path = Vec::new();
    let mut path_trace = (min_cost_end_dir, num_rows - 1, num_cols - 1);
    while !(path_trace.1 == 0 && path_trace.2 == 0) {
        path.insert(0, path_trace);
        path_trace = prev_nodes[path_trace.0][path_trace.1][path_trace.2];
    }
    for node in path {
        println!("{:?}", node);
    }

    // Find shortest path to bottom right value (from any direction)
    return (0..4).map(|path_limit| tentative_costs[path_limit][num_rows - 1][num_cols - 1]).min().unwrap();
}
