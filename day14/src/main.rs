use std::io;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    // Split lines into grid
    // Add padding of '#' at top to simplify algorithm
    let mut grid = vec![vec!['#'; lines[0].len()]; lines.len() + 1];
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    for i in 0..lines.len() {
        for (j, ch) in lines[i].chars().enumerate() {
            grid[i + 1][j] = ch;
        }
    }

    // Move each round boulder as far north as possible
    for i in 1..grid_height {
        for j in 0..grid_width {
            if grid[i][j] == 'O' {
                grid[i][j] = '.';
                let mut new_row = i;
                while new_row > 0 && grid[new_row - 1][j] == '.' {
                    new_row -= 1;
                }
                grid[new_row][j] = 'O';
            }
        }
    }
    // Debug print
    /*
    for i in 1..grid_height {
        for j in 0..grid_width {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
    */

    // Calculate total score
    let mut subtotal = 0;
    for i in 1..grid_height {
        for j in 0..grid_width {
            if grid[i][j] == 'O' {
                subtotal += grid_height - i;
            }
        }
    }
    return subtotal as i32;
}

fn part2(lines: &Vec<String>) -> i32 {
    // Split lines into grid\
    let mut grid = vec![vec!['#'; lines[0].len()]; lines.len()];
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    for i in 0..lines.len() {
        for (j, ch) in lines[i].chars().enumerate() {
            grid[i][j] = ch;
        }
    }

    // Run movement cycles, find when positions recur
    let mut pos_map = HashMap::new();
    let total_cycles = 1000000000;
    let mut recurring_cycle_index = 0;
    for cycle_count in 1..(total_cycles + 1) {
        // Move each round boulder as far north as possible
        for i in 0..grid_height {
            for j in 0..grid_width {
                if grid[i][j] == 'O' {
                    grid[i][j] = '.';
                    let mut new_row = i;
                    while new_row > 0 && grid[new_row - 1][j] == '.' {
                        new_row -= 1;
                    }
                    grid[new_row][j] = 'O';
                }
            }
        }
        // Move each round boulder as far west as possible
        for i in 0..grid_height {
            for j in 0..grid_width {
                if grid[i][j] == 'O' {
                    grid[i][j] = '.';
                    let mut new_col = j;
                    while new_col > 0 && grid[i][new_col - 1] == '.' {
                        new_col -= 1;
                    }
                    grid[i][new_col] = 'O';
                }
            }
        }
        // Move each round boulder as far south as possible
        for i in (0..grid_height).rev() {
            for j in 0..grid_width {
                if grid[i][j] == 'O' {
                    grid[i][j] = '.';
                    let mut new_row = i;
                    while new_row < grid_height - 1 && grid[new_row + 1][j] == '.' {
                        new_row += 1;
                    }
                    grid[new_row][j] = 'O';
                }
            }
        }
        // Move each round boulder as far east as possible
        for i in 0..grid_height {
            for j in (0..grid_width).rev() {
                if grid[i][j] == 'O' {
                    grid[i][j] = '.';
                    let mut new_col = j;
                    while new_col < grid_width - 1 && grid[i][new_col + 1] == '.' {
                        new_col += 1;
                    }
                    grid[i][new_col] = 'O';
                }
            }
        }
        
        // Check if current position has already been generated, store position otherwise
        if pos_map.contains_key(&grid) {
            // Calculate which cycle equals the final value
            println!("Recurs over {} cycles", pos_map.len());
            println!("Cycle {} equals cycle {}", cycle_count, pos_map.get(&grid).unwrap());
            let recurrence_start_index = pos_map.get(&grid).unwrap();
            let recurrence_length = cycle_count - recurrence_start_index;
            recurring_cycle_index = (total_cycles - recurrence_start_index) % recurrence_length + recurrence_start_index;
            break;
        }
        else {
            pos_map.insert(grid.clone(), cycle_count);
        }

    }
    // Debug print
    let final_grid = pos_map.iter().find_map(|(k, v)| 
                                                if *v == recurring_cycle_index { Some(k) } else { None }
                                                ).unwrap();
    for i in 0..grid_height {
        for j in 0..grid_width {
            print!("{}", final_grid[i][j]);
        }
        println!("");
    }
    /*
    for (k, v) in &pos_map {
        let mut score = 0;
        for i in 0..grid_height {
            for j in 0..grid_width {
                print!("{}", k[i][j]);
                if k[i][j] == 'O' {
                    score += grid_height - i;
                }
            }
            println!("");
        }
        println!("Cycle {}, score = {}\n", v, score);
    }
     */

    // Calculate total score
    let mut subtotal = 0;
    for i in 0..grid_height {
        for j in 0..grid_width {
            if final_grid[i][j] == 'O' {
                subtotal += grid_height - i;
            }
        }
    }
    return subtotal as i32;
}