use std::{collections::{HashMap, HashSet, VecDeque}, io};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}

struct DAGNode {
    pos: (usize, usize),
    parents: HashSet<(usize, usize)>,
    children: HashSet<(usize, usize)>
}

// Assumes that input is a valid DAG
fn part1(grid: &Vec<Vec<char>>) -> u32 {
    // Build DAG
    let mut dag_node_map: HashMap<(usize, usize), DAGNode> = HashMap::new();

    //let mut visited = vec![vec![false; width]; height];
    let mut visit_stack = Vec::new();

    let start_col = grid[0].iter().position(|c| *c == '.').unwrap();
    let end_col = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
    //visited[0][start_col] = true;
    visit_stack.push((0, start_col));
    let start_dag_node = DAGNode { pos: (0, start_col), parents: HashSet::new(), children: HashSet::new()};
    dag_node_map.insert((0, start_col), start_dag_node);

    while visit_stack.len() > 0 {
        let (curr_row, curr_col) = visit_stack.pop().unwrap();
        let curr_node_parents = &dag_node_map.get(&(curr_row, curr_col)).unwrap().parents.clone(); // Inefficient, look at this later
        
        // Add current node as parent to all children (all outgoing connections which are not parent nodes)
        let new_node_children: HashSet<(usize, usize)> = get_node_children(grid, (curr_row, curr_col)).difference(curr_node_parents).map(|c| *c).collect();
        for (child_row, child_col) in new_node_children.iter() {
            match dag_node_map.get_mut(&(*child_row, *child_col)) {
                Some(dag) => { // Add current node as parent
                    dag.parents.insert((curr_row, curr_col));
                },
                None => { // Create new node and add current node as parent
                    let new_dag_node = DAGNode { pos: (*child_row, *child_col), parents: HashSet::from([(curr_row, curr_col)]), children: HashSet::new() };
                    dag_node_map.insert((*child_row, *child_col), new_dag_node);
                    visit_stack.push((*child_row, *child_col));
                }
            }
        }

        dag_node_map.get_mut(&(curr_row, curr_col)).unwrap().children = new_node_children;
    }

    // Perform topological sort
    let mut topo_visited_nodes = HashSet::new();
    let mut topo_visit_stack = Vec::new();
    let mut topo_sorted_nodes = VecDeque::new();
    
    topo_visit_stack.push((0, start_col));
    while topo_visit_stack.len() > 0 {
        let curr_coords = *topo_visit_stack.last().unwrap();
        let curr_node = dag_node_map.get(&curr_coords).unwrap();
        if curr_node.children.iter().all(|n| topo_visited_nodes.contains(n)) {
            topo_sorted_nodes.push_front(curr_coords);
            topo_visited_nodes.insert(curr_coords);
            topo_visit_stack.pop();
        }
        else {
            for child_node in curr_node.children.iter() {
                topo_visit_stack.push(*child_node);
            }
        }
    }
    //print!("{:?}", topo_sorted_nodes);
    
    let mut coords: Vec<(usize, usize)> = dag_node_map.keys().map(|c| *c).collect();
    coords.sort();
    for (node_row, node_col) in coords {
        let dag_node = dag_node_map.get(&(node_row, node_col)).unwrap();
        //println!("({}, {}) - parents: {:?}, children: {:?}", node_row, node_col, dag_node.parents, dag_node.children);
    }

    // Find distances by traversing topologically-sorted nodes
    let mut tentative_distances = HashMap::new();
    tentative_distances.insert((0, start_col), 0);
    for (node_row, node_col) in topo_sorted_nodes.into_iter().skip(1) {
        let max_parent_distance = *dag_node_map.get(&(node_row, node_col))
                                                    .unwrap()
                                                    .parents
                                                    .iter()
                                                    .map(|(p_row, p_col)| tentative_distances.get(&(*p_row, *p_col)).unwrap())
                                                    .max()
                                                    .unwrap();
        tentative_distances.insert((node_row, node_col), max_parent_distance + 1);
    }

    return *tentative_distances.get(&(grid.len() - 1, end_col)).unwrap();
}

fn get_node_children(grid: &Vec<Vec<char>>, coords: (usize, usize)) -> HashSet<(usize, usize)> {
    let (curr_row, curr_col) = coords;
    let mut children = HashSet::new();

    if curr_row > 0 {
        let north_neighbor = grid[curr_row - 1][curr_col];
        if (north_neighbor == '.') || (north_neighbor == '^') {
            children.insert((curr_row - 1, curr_col));
        }
    }
    if curr_row < grid.len() - 1 {
        let south_neighbor = grid[curr_row + 1][curr_col];
        if (south_neighbor == '.') || (south_neighbor == 'v') {
            children.insert((curr_row + 1, curr_col));
        }
    }
    let east_neighbor = grid[curr_row][curr_col + 1];
    if (east_neighbor == '.') || (east_neighbor == '>') {
        children.insert((curr_row, curr_col + 1));
    }
    let west_neighbor = grid[curr_row][curr_col - 1];
    if (west_neighbor == '.') || (west_neighbor == '<') {
        children.insert((curr_row, curr_col - 1));
    }
    return children;
}

// Assumes that input is a valid DAG
fn part2(grid: &Vec<Vec<char>>) -> u32 {
    // Build DAG
    let mut dag_node_map: HashMap<(usize, usize), DAGNode> = HashMap::new();

    //let mut visited = vec![vec![false; width]; height];
    let mut visit_stack = Vec::new();

    let start_col = grid[0].iter().position(|c| *c == '.').unwrap();
    let end_col = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
    //visited[0][start_col] = true;
    visit_stack.push((0, start_col));
    
    // GRAPH IS CYCLIC
    // ONLY USE POINTS WITH MULTIPLE POSSIBLE PATHS AS NODES

    return 0;
}