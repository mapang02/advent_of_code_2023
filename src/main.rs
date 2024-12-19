use std::{collections::{HashMap, HashSet, VecDeque}, io, path};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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

struct Node {
    pos: (usize, usize),
    neighbors: Vec<((usize, usize), usize)>
}

// Assumes that input is a valid DAG
fn part2(grid: &Vec<Vec<char>>) -> usize {
    // Build graph
    let width = grid[0].len();
    let height = grid.len();
    let start_col = grid[0].iter().position(|c| *c == '.').unwrap();
    let end_col = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut nodes = HashMap::new();
    nodes.insert((0, start_col), Node { pos: (0, start_col), neighbors: Vec::new() });

    let mut node_visit_queue = VecDeque::from([(0, start_col)]);
    while node_visit_queue.len() > 0 {
        // Find directions of outgoing paths
        let (curr_row, curr_col) = node_visit_queue.pop_front().unwrap();
        let unvisited_neighbors: Vec<(usize, usize)> = get_neighbors(grid, (curr_row, curr_col))
                                            .into_iter()
                                            .filter(|(n_row, n_col)| !visited[*n_row][*n_col])
                                            .collect();

        // Follow path to neighboring node
        for nbr_coords in unvisited_neighbors {
            let mut ptr_coords = nbr_coords;
            let mut prev_ptr_coords = (curr_row, curr_col);

            let mut distance = 1;
            loop {
                let ptr_neighbors = get_neighbors(grid, ptr_coords);
                if ptr_neighbors.len() == 2 { // Is a path
                    let (next_ptr_row, next_ptr_col) = if ptr_neighbors[0] == prev_ptr_coords { ptr_neighbors[1] } else { ptr_neighbors[0] };
                    prev_ptr_coords = ptr_coords;
                    ptr_coords = (next_ptr_row, next_ptr_col);
                }
                else { // Tiles with exactly 1 neighbor or 3+ neighbors are nodes
                    if !nodes.contains_key(&ptr_coords) {
                        let new_node = Node { pos: ptr_coords, neighbors: Vec::new() };
                        nodes.insert(ptr_coords, new_node);
                        node_visit_queue.push_back(ptr_coords);
                    }
                    nodes.get_mut(&ptr_coords).unwrap().neighbors.push(((curr_row, curr_col), distance));
                    nodes.get_mut(&(curr_row, curr_col)).unwrap().neighbors.push((ptr_coords, distance));
                    break;
                }
                visited[ptr_coords.0][ptr_coords.1] = true;
                distance += 1;
            }
        }
    }
    print_nodes(&nodes);

    // Iterate through all possible paths
    let mut tentative_max_length = 0;
    let mut curr_path = Vec::new(); // Current path nodes in order
    let mut curr_path_edges = HashMap::new();   // Key is path node, value is prev node and distance
    let mut explore_stack = Vec::new(); // Edges in the format of (source node, dest node, distance)

    let start_node = nodes.get(&(0, start_col)).unwrap();
    let end_node_coords = (height - 1, end_col);
    curr_path.push(start_node.pos);
    curr_path_edges.insert(start_node.pos, (start_node.pos, 0));
    for (n, dist) in start_node.neighbors.iter() {
        explore_stack.push((start_node.pos, *n, *dist));
    }
    while explore_stack.len() > 0 {
        let (prev_coords, curr_coords, dist) = explore_stack.pop().unwrap();
        while prev_coords != *curr_path.last().unwrap() {
            let last_node = curr_path.pop().unwrap();
            curr_path_edges.remove(&last_node);
        }
        //println!("Exploring {:?} from {:?}", curr_coords, prev_coords);
        //println!("{:?}", explore_stack);
        //println!("{:?}", curr_path);
        if !curr_path_edges.contains_key(&curr_coords) {
            // If path completed, calculate total length
            if curr_coords == end_node_coords {
                curr_path_edges.insert(curr_coords, (prev_coords, dist));
                let path_length = curr_path_edges.values().map(|(_, dist)| *dist).sum::<usize>();
                //print_path(&curr_path_edges, (height - 1, end_col));
                //println!("Length: {}", path_length);
                if path_length > tentative_max_length {
                    tentative_max_length = path_length;
                }
                curr_path_edges.remove(&curr_coords);
                continue;
            }

            // Add current node to path, continue exploring
            let unvisited_neighbors: Vec<&((usize, usize), usize)> = nodes.get(&curr_coords)
                                        .unwrap()
                                        .neighbors
                                        .iter()
                                        .filter(|(n, _)| !curr_path_edges.contains_key(n))
                                        .collect();
            if unvisited_neighbors.len() > 0 { // Add next node's neighbors to explore stack
                curr_path.push(curr_coords);
                curr_path_edges.insert(curr_coords, (prev_coords, dist));
                for (next_coords, next_dist) in unvisited_neighbors {
                    explore_stack.push((curr_coords, *next_coords, *next_dist));
                }
            }
        }
    }
    return tentative_max_length;
}

fn get_neighbors(grid: &Vec<Vec<char>>, tile_coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (curr_row, curr_col) = tile_coords;
    let mut output = Vec::new();
    if (curr_row > 0) && grid[curr_row - 1][curr_col] != '#' {
        output.push((curr_row - 1, curr_col));
    }
    if (curr_row < grid.len() - 1) && grid[curr_row + 1][curr_col] != '#' {
        output.push((curr_row + 1, curr_col));
    }
    if (curr_col > 0) && grid[curr_row][curr_col - 1] != '#' {
        output.push((curr_row, curr_col - 1));
    }
    if (curr_col < grid[0].len() - 1) && grid[curr_row][curr_col + 1] != '#' {
        output.push((curr_row, curr_col + 1));
    }
    return output;
}

fn print_nodes(nodes: &HashMap<(usize, usize), Node>) {
    let mut node_list: Vec<((usize, usize), &Node)> = (*nodes).iter().map(|(k, v)| (*k, v)).collect();
    node_list.sort_by_key(|(k, v)| *k);
    println!("[{} nodes]", node_list.len());
    for (coords, node) in node_list {
        println!("{:?} - neighbors: {:?}", coords, node.neighbors);
    }
    //println!("Product of connections: {}", nodes.values().map(|n| if n.neighbors.len() > 1 { n.neighbors.len() - 1 } else { 1 }).product())
}

fn print_path(path: &HashMap<(usize, usize), ((usize, usize), usize)>, end_coords: (usize, usize)) {
    let mut edges = Vec::new();
    let mut cursor = end_coords;
    loop {
        let (prev_coords, dist) = path.get(&cursor).unwrap();
        edges.push((*prev_coords, cursor, *dist));
        if *prev_coords == cursor {
            break;
        }
        cursor = *prev_coords;
    }
    for (source_coords, dest_coords, dist) in edges.iter().rev() {
        println!("{:?} to {:?} - {}", source_coords, dest_coords, dist);
    }
}