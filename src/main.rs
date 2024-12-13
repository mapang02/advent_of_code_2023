use std::{collections::HashMap, io};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars()).collect();
    let part1 = part1(&lines);
    println!("Part 1: {}", part1);
}

struct DAGNode {
    pos: (usize, usize),
    parents: Vec<(usize, usize)>,
    children: Vec<(usize, usize)>,
    distance: u32
}

fn part1(grid: &Vec<Vec<char>>) -> u32 {
    let width = grid[0].len();
    let height = grid.len();
    let start_col = grid[0].iter().position(|c| *c == '.').unwrap();
    let end_col = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
    let mut visited = vec![vec![false; width]; height];
    let mut dag_node_map = HashMap::new();

    let mut visit_stack = Vec::new();
    visit_stack.push((0, start_col));
    visited[0][start_col] = true;

    while visit_stack.len() > 0 {
        // Create new node
        let (curr_row, curr_col) = visit_stack.pop().unwrap();
        
        let new_node_parents = get_node_parents(grid, &dag_node_map, (curr_row, curr_col));
        let new_node_children = get_node_children(grid, &dag_node_map, (curr_row, curr_col));

        let parent_distances = new_node_parents.iter().map(|coords| dag_node_map.get(coords).unwrap().distance);
        let new_node_distance = parent_distances.max().unwrap() + 1;

        let new_dag_node = DAGNode { pos: (curr_row, curr_col), parents: new_node_parents, children: new_node_children, distance: new_node_distance };
        dag_node_map.insert((curr_row, curr_col), new_dag_node);

        // Find child nodes where all parents have been visited
        for (child_row, child_col) in dag_node_map.get(&(curr_row, curr_col)).unwrap().children.iter() {
            let child_node_parents = get_node_parents(grid, &dag_node_map, (*child_row, *child_col));
            if child_node_parents
        }
    }

    return 0;
}

fn get_node_parents(grid: &Vec<Vec<char>>, dag_node_map: &HashMap<(usize, usize), DAGNode>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (curr_row, curr_col) = coords;
    let mut parents = Vec::new();

    if curr_row > 0 {
        let north_neighbor = grid[curr_row - 1][curr_col];
        if (north_neighbor == '.' || north_neighbor == 'v') && dag_node_map.contains_key(&(curr_row - 1, curr_col)) {
            parents.push((curr_row - 1, curr_col));
        }
    }
    else if curr_row < grid.len() - 1 {
        let south_neighbor = grid[curr_row + 1][curr_col];
        if (south_neighbor == '.' || south_neighbor == '^') && dag_node_map.contains_key(&(curr_row + 1, curr_col)) {
            parents.push((curr_row - 1, curr_col));
        }
    }
    let east_neighbor = grid[curr_row][curr_col + 1];
    if (east_neighbor == '.' || east_neighbor == '<') && dag_node_map.contains_key(&(curr_row, curr_col + 1)) {
        parents.push((curr_row, curr_col + 1));
    }
    let west_neighbor = grid[curr_row][curr_col - 1];
    if (west_neighbor == '.' || west_neighbor == '>') && dag_node_map.contains_key(&(curr_row, curr_col - 1)) {
        parents.push((curr_row, curr_col - 1));
    }
    return parents;
}

fn get_node_children(grid: &Vec<Vec<char>>, dag_node_map: &HashMap<(usize, usize), DAGNode>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (curr_row, curr_col) = coords;
    let mut children = Vec::new();

    if curr_row > 0 {
        let north_neighbor = grid[curr_row - 1][curr_col];
        if (north_neighbor == '.' || north_neighbor == '^') && !dag_node_map.contains_key(&(curr_row - 1, curr_col)) {
            children.push((curr_row - 1, curr_col));
        }
    }
    else if curr_row < grid.len() - 1 {
        let south_neighbor = grid[curr_row + 1][curr_col];
        if (south_neighbor == '.' || south_neighbor == 'v') && !dag_node_map.contains_key(&(curr_row + 1, curr_col)) {
            children.push((curr_row - 1, curr_col));
        }
    }
    let east_neighbor = grid[curr_row][curr_col + 1];
    if (east_neighbor == '.' || east_neighbor == '>') && !dag_node_map.contains_key(&(curr_row, curr_col + 1)) {
        children.push((curr_row, curr_col + 1));
    }
    let west_neighbor = grid[curr_row][curr_col - 1];
    if (west_neighbor == '.' || west_neighbor == '<') && !dag_node_map.contains_key(&(curr_row, curr_col - 1)) {
        children.push((curr_row, curr_col - 1));
    }
    return children;
}