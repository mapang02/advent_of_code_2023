use std::io;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i64 {
    // Convert nodes to HashMap
    let mut node_map = HashMap::new();
    for l in &lines[2..] {
        let source_node = &l[..(l.find(" ").unwrap())];
        let left_node = &l[(l.find("(").unwrap() + 1)..(l.find(",").unwrap())];
        let right_node = &l[(l.find(",").unwrap() + 2)..(l.find(")").unwrap())];
        node_map.insert(source_node, (left_node, right_node));
    }

    // Traverse nodes
    let move_sequence = &lines[0];
    let mut curr_node = "AAA";
    let mut num_steps = 0;
    'step_loop: loop {
        for dir in move_sequence.chars() {
            curr_node = match dir {
                'L' => node_map.get(curr_node).unwrap().0,
                'R' => node_map.get(curr_node).unwrap().1,
                _ => unreachable!()
            };
            num_steps += 1;
            if curr_node == "ZZZ" {
                break 'step_loop;
            }
        }
    }

    return num_steps;
}

fn part2(lines: &Vec<String>) -> i64 {
    // Convert nodes to HashMap
    let mut node_map = HashMap::new();
    for l in &lines[2..] {
        let source_node = &l[..(l.find(" ").unwrap())];
        let left_node = &l[(l.find("(").unwrap() + 1)..(l.find(",").unwrap())];
        let right_node = &l[(l.find(",").unwrap() + 2)..(l.find(")").unwrap())];
        node_map.insert(source_node, (left_node, right_node));
    }

    // Traverse nodes, calculate path length for each start node
    let move_sequence = &lines[0];
    let mut path_lengths = HashMap::new();
    for start_node in node_map.keys().filter(|n| n.ends_with("A")) {
        let mut curr_node = *start_node;
        let mut num_steps = 0;
        'step_loop: loop {
            for dir in move_sequence.chars() {
                curr_node = match dir {
                    'L' => node_map.get(curr_node).unwrap().0,
                    'R' => node_map.get(curr_node).unwrap().1,
                    _ => unreachable!()
                };
                num_steps += 1;
                if curr_node.ends_with("Z") {
                    break 'step_loop;
                }
            }
        }
        path_lengths.insert(start_node, num_steps);
    }
    dbg!(&path_lengths);

    // Calculate LCM of all path lengths, required to make all paths reach __Z simultaneously
    let mut lcm = 1;
    for step_count in path_lengths.values() {
        lcm = step_count * lcm / gcd(*step_count, lcm);
    }
    return lcm;
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a = a - b;
        }
        else {
            b = b - a;
        }
    }
    return a;
}