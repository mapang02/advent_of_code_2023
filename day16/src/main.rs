use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 1: {part2}");
}

#[derive(Copy, Clone, Debug)]
enum Dir { 
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3 
}

fn part1(lines: &Vec<String>) -> i32 {
    // Set up grid and tracking data
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut grid = vec![vec!['.'; num_cols]; num_rows];
    for (row, ln) in lines.iter().enumerate() {
        for (col, ch) in ln.chars().enumerate() {
            grid[row][col] = ch;
        }
    }
    let mut visited = vec![vec![vec![false; 4]; num_cols]; num_rows];

    // Trace all paths with DFS, stopping on tiles already visited
    let mut beam_stack = vec![(0, 0, Dir::RIGHT)];
    while beam_stack.len() > 0 {
        let (visit_row, visit_col, beam_dir) = beam_stack.pop().unwrap();
        // Check if current position is already visited in the current direction, mark as visited
        if visited[visit_row][visit_col][beam_dir as usize] {
            println!("Revisiting ({}, {}) in direction {:?}", visit_row, visit_col, beam_dir);
            continue;
        }
        visited[visit_row][visit_col][beam_dir as usize] = true;
        println!("Visiting ({}, {})", visit_row, visit_col);

        // Push new beam location(s) and direction(s) to stack
        let curr_tile = grid[visit_row][visit_col];
        if curr_tile == '.' { // Handle empty tile
            let next_row = match beam_dir {
                Dir::UP => visit_row.checked_sub(1),
                Dir::DOWN => Some(visit_row + 1),
                Dir::LEFT | Dir::RIGHT => Some(visit_row)
            };
            let next_col = match beam_dir {
                Dir::UP | Dir::DOWN => Some(visit_col),
                Dir::LEFT => visit_col.checked_sub(1),
                Dir::RIGHT => visit_col.checked_add(1)
            };
            if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                beam_stack.push((next_row.unwrap(), next_col.unwrap(), beam_dir));
            }
        }
        else if curr_tile == '\\' { // Handle '\' mirror
            let next_row = match beam_dir {
                Dir::UP | Dir::DOWN => Some(visit_row),
                Dir::LEFT => visit_row.checked_sub(1),
                Dir::RIGHT => visit_row.checked_add(1)
            };
            let next_col = match beam_dir {
                Dir::UP => visit_col.checked_sub(1),
                Dir::DOWN => visit_col.checked_add(1),
                Dir::LEFT | Dir::RIGHT => Some(visit_col)
            };
            let next_dir = match beam_dir {
                Dir::UP => Dir::LEFT,
                Dir::DOWN => Dir::RIGHT,
                Dir::LEFT => Dir::UP,
                Dir::RIGHT => Dir::DOWN
            };
            if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                beam_stack.push((next_row.unwrap(), next_col.unwrap(), next_dir));
            }
        }
        else if curr_tile == '/' { // Handle '/' mirror
            let next_row = match beam_dir {
                Dir::UP | Dir::DOWN => Some(visit_row),
                Dir::LEFT => visit_row.checked_add(1),
                Dir::RIGHT => visit_row.checked_sub(1)
            };
            let next_col = match beam_dir {
                Dir::UP => visit_col.checked_add(1),
                Dir::DOWN => visit_col.checked_sub(1),
                Dir::LEFT | Dir::RIGHT => Some(visit_col)
            };
            let next_dir = match beam_dir {
                Dir::UP => Dir::RIGHT,
                Dir::DOWN => Dir::LEFT,
                Dir::LEFT => Dir::DOWN,
                Dir::RIGHT => Dir::UP
            };
            if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                beam_stack.push((next_row.unwrap(), next_col.unwrap(), next_dir));
            }
        }
        else if curr_tile == '|' { // Handle vertical splitter
            let is_perpendicular = match beam_dir {
                Dir::UP | Dir::DOWN => false,
                Dir::LEFT | Dir::RIGHT => true
            };
            if is_perpendicular { // Handle splitting
                let up_beam_row = visit_row.checked_sub(1);
                let down_beam_row = visit_row + 1;
                if up_beam_row.is_some() {
                    beam_stack.push((up_beam_row.unwrap(), visit_col, Dir::UP));
                }
                if down_beam_row < num_rows {
                    beam_stack.push((down_beam_row, visit_col, Dir::DOWN));
                }
            }
            else { // Handle like empty tile
                // Only UP and DOWN are possible, so next_col is always visit_col
                let next_row = match beam_dir {
                    Dir::UP => visit_row.checked_sub(1),
                    Dir::DOWN => visit_row.checked_add(1),
                    _ => unreachable!()
                };
                if next_row.is_some() && next_row.unwrap() < num_rows {
                    beam_stack.push((next_row.unwrap(), visit_col, beam_dir));
                }
            }
        }
        else if curr_tile == '-' { // Handle horizontal splitter
            let is_perpendicular = match beam_dir {
                Dir::UP | Dir::DOWN => true,
                Dir::LEFT | Dir::RIGHT => false
            };
            if is_perpendicular { // Handle splitting
                let left_beam_col = visit_col.checked_sub(1);
                let right_beam_col = visit_col + 1;
                if left_beam_col.is_some() {
                    beam_stack.push((visit_row, left_beam_col.unwrap(), Dir::LEFT));
                }
                if right_beam_col < num_cols {
                    beam_stack.push((visit_row, right_beam_col, Dir::RIGHT));
                }
            }
            else { // Handle like empty tile
                // Only LEFT and RIGHT are possible, so next_row is always visit_row
                let next_col = match beam_dir {
                    Dir::LEFT => visit_col.checked_sub(1),
                    Dir::RIGHT => visit_col.checked_add(1),
                    _ => unreachable!()
                };
                if next_col.is_some() && next_col.unwrap() < num_cols {
                    beam_stack.push((visit_row, next_col.unwrap(), beam_dir));
                }
            }
        }
    }

    // Count visited tiles
    let mut visited_tiles = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if visited[i][j].iter().any(|v| *v) {
                visited_tiles += 1;
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    return visited_tiles;
}

fn part2(lines: &Vec<String>) -> i32 {    // Set up grid and tracking data
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut grid = vec![vec!['.'; num_cols]; num_rows];
    for (row, ln) in lines.iter().enumerate() {
        for (col, ch) in ln.chars().enumerate() {
            grid[row][col] = ch;
        }
    }
    
    // Helper function for counting visited tiles with given start configuration
    fn count_visited(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, start_dir: Dir) -> i32 {
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        let mut visited = vec![vec![vec![false; 4]; num_cols]; num_rows];

        // Trace all paths with DFS, stopping on tiles already visited
        let mut beam_stack = vec![(start_row, start_col, start_dir)];
        while beam_stack.len() > 0 {
            let (visit_row, visit_col, beam_dir) = beam_stack.pop().unwrap();
            // Check if current position is already visited in the current direction, mark as visited
            if visited[visit_row][visit_col][beam_dir as usize] {
                continue;
            }
            visited[visit_row][visit_col][beam_dir as usize] = true;

            // Push new beam location(s) and direction(s) to stack
            let curr_tile = grid[visit_row][visit_col];
            if curr_tile == '.' { // Handle empty tile
                let next_row = match beam_dir {
                    Dir::UP => visit_row.checked_sub(1),
                    Dir::DOWN => Some(visit_row + 1),
                    Dir::LEFT | Dir::RIGHT => Some(visit_row)
                };
                let next_col = match beam_dir {
                    Dir::UP | Dir::DOWN => Some(visit_col),
                    Dir::LEFT => visit_col.checked_sub(1),
                    Dir::RIGHT => visit_col.checked_add(1)
                };
                if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                    beam_stack.push((next_row.unwrap(), next_col.unwrap(), beam_dir));
                }
            }
            else if curr_tile == '\\' { // Handle '\' mirror
                let next_row = match beam_dir {
                    Dir::UP | Dir::DOWN => Some(visit_row),
                    Dir::LEFT => visit_row.checked_sub(1),
                    Dir::RIGHT => visit_row.checked_add(1)
                };
                let next_col = match beam_dir {
                    Dir::UP => visit_col.checked_sub(1),
                    Dir::DOWN => visit_col.checked_add(1),
                    Dir::LEFT | Dir::RIGHT => Some(visit_col)
                };
                let next_dir = match beam_dir {
                    Dir::UP => Dir::LEFT,
                    Dir::DOWN => Dir::RIGHT,
                    Dir::LEFT => Dir::UP,
                    Dir::RIGHT => Dir::DOWN
                };
                if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                    beam_stack.push((next_row.unwrap(), next_col.unwrap(), next_dir));
                }
            }
            else if curr_tile == '/' { // Handle '/' mirror
                let next_row = match beam_dir {
                    Dir::UP | Dir::DOWN => Some(visit_row),
                    Dir::LEFT => visit_row.checked_add(1),
                    Dir::RIGHT => visit_row.checked_sub(1)
                };
                let next_col = match beam_dir {
                    Dir::UP => visit_col.checked_add(1),
                    Dir::DOWN => visit_col.checked_sub(1),
                    Dir::LEFT | Dir::RIGHT => Some(visit_col)
                };
                let next_dir = match beam_dir {
                    Dir::UP => Dir::RIGHT,
                    Dir::DOWN => Dir::LEFT,
                    Dir::LEFT => Dir::DOWN,
                    Dir::RIGHT => Dir::UP
                };
                if next_row.is_some() && next_row.unwrap() < num_rows && next_col.is_some() && next_col.unwrap() < num_cols {
                    beam_stack.push((next_row.unwrap(), next_col.unwrap(), next_dir));
                }
            }
            else if curr_tile == '|' { // Handle vertical splitter
                let is_perpendicular = match beam_dir {
                    Dir::UP | Dir::DOWN => false,
                    Dir::LEFT | Dir::RIGHT => true
                };
                if is_perpendicular { // Handle splitting
                    let up_beam_row = visit_row.checked_sub(1);
                    let down_beam_row = visit_row + 1;
                    if up_beam_row.is_some() {
                        beam_stack.push((up_beam_row.unwrap(), visit_col, Dir::UP));
                    }
                    if down_beam_row < num_rows {
                        beam_stack.push((down_beam_row, visit_col, Dir::DOWN));
                    }
                }
                else { // Handle like empty tile
                    // Only UP and DOWN are possible, so next_col is always visit_col
                    let next_row = match beam_dir {
                        Dir::UP => visit_row.checked_sub(1),
                        Dir::DOWN => visit_row.checked_add(1),
                        _ => unreachable!()
                    };
                    if next_row.is_some() && next_row.unwrap() < num_rows {
                        beam_stack.push((next_row.unwrap(), visit_col, beam_dir));
                    }
                }
            }
            else if curr_tile == '-' { // Handle horizontal splitter
                let is_perpendicular = match beam_dir {
                    Dir::UP | Dir::DOWN => true,
                    Dir::LEFT | Dir::RIGHT => false
                };
                if is_perpendicular { // Handle splitting
                    let left_beam_col = visit_col.checked_sub(1);
                    let right_beam_col = visit_col + 1;
                    if left_beam_col.is_some() {
                        beam_stack.push((visit_row, left_beam_col.unwrap(), Dir::LEFT));
                    }
                    if right_beam_col < num_cols {
                        beam_stack.push((visit_row, right_beam_col, Dir::RIGHT));
                    }
                }
                else { // Handle like empty tile
                    // Only LEFT and RIGHT are possible, so next_row is always visit_row
                    let next_col = match beam_dir {
                        Dir::LEFT => visit_col.checked_sub(1),
                        Dir::RIGHT => visit_col.checked_add(1),
                        _ => unreachable!()
                    };
                    if next_col.is_some() && next_col.unwrap() < num_cols {
                        beam_stack.push((visit_row, next_col.unwrap(), beam_dir));
                    }
                }
            }
        }

        // Count visited tiles
        let mut visited_tiles = 0;
        for i in 0..num_rows {
            for j in 0..num_cols {
                if visited[i][j].iter().any(|v| *v) {
                    visited_tiles += 1;
                }
            }
        }
        return visited_tiles;
    }

    // Find start position with greatest number of visited tiles
    let mut max_visited_tiles = 0;
    for i in 0..num_rows { // Start from left
        let visited_tiles = count_visited(&grid, i, 0, Dir::RIGHT);
        if max_visited_tiles < visited_tiles {
            max_visited_tiles = visited_tiles;
        }
    }
    for i in 0..num_rows { // Start from right
        let visited_tiles = count_visited(&grid, i, num_cols - 1, Dir::LEFT);
        if max_visited_tiles < visited_tiles {
            max_visited_tiles = visited_tiles;
        }
    }
    for i in 0..num_cols { // Start from top
        let visited_tiles = count_visited(&grid, 0, i, Dir::DOWN);
        if max_visited_tiles < visited_tiles {
            max_visited_tiles = visited_tiles;
        }
    }
    for i in 0..num_cols { // Start from bottom
        let visited_tiles = count_visited(&grid, num_rows - 1, i, Dir::UP);
        if max_visited_tiles < visited_tiles {
            max_visited_tiles = visited_tiles;
        }
    }
    return max_visited_tiles;
}
