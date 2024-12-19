use std::{io, collections::VecDeque};

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 1: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    // Parse instructions
    let mut instructions = Vec::new();
    for ln in lines {
        let parts: Vec<&str> = ln.splitn(3, " ").collect();
        let dir = parts[0];
        let step_count: usize = parts[1].parse().unwrap();
        let color = &parts[2][2..8]; // Cut out the 6 hexadecimal digits
        instructions.push((dir, step_count, color));
    }

    // Draw boundary in dynamic grid
    let mut num_rows = 4;
    let mut num_cols = 4;
    let mut grid = vec![vec!['.'; num_cols]; num_rows];
    let mut curr_row = 0;
    let mut curr_col = 0;
    for (dir, step_count, _) in instructions {
        if dir == "U" {
            // Expand upwards if necessary
            while curr_row < step_count {
                grid.splice(0..0, vec![vec!['.'; num_cols]; num_rows]);
                curr_row += num_rows;
                num_rows *= 2;
            }
            for _ in 0..step_count {
                grid[curr_row][curr_col] = '#';
                curr_row -= 1;
            }
        }
        else if dir == "D" {
            // Expand downwards if necessary
            while curr_row + step_count >= num_rows {
                grid.extend_from_slice(&vec![vec!['.'; num_cols]; num_rows]);
                num_rows *= 2;
            }
            for _ in 0..step_count {
                grid[curr_row][curr_col] = '#';
                curr_row += 1;
            }
        }
        else if dir == "L" {
            // Expand leftwards if necessary
            while curr_col < step_count {
                for row_num in 0..num_rows {
                    grid[row_num].splice(0..0, vec!['.'; num_cols]);
                }
                curr_col += num_cols;
                num_cols *= 2;
            }
            for _ in 0..step_count {
                grid[curr_row][curr_col] = '#';
                curr_col -= 1;
            }
        }
        else if dir == "R" {
            // Expand rightwards if necessary
            while curr_col + step_count >= num_cols {
                for row_num in 0..num_rows {
                    grid[row_num].extend_from_slice(&vec!['.'; num_cols]);
                }
                num_cols *= 2;
            }
            for _ in 0..step_count {
                grid[curr_row][curr_col] = '#';
                curr_col += 1;
            }
        }
    }

    // Perform BFS to fill interior
    let mut visited = vec![vec![false; num_cols]; num_rows];
    for row_num in 0..num_rows {
        for col_num in 0..num_cols {
            if grid[row_num][col_num] == '#' {
                visited[row_num][col_num] = true;
            }
        }
    }
    let start_row = grid.iter().position(|row| row.contains(&'#')).unwrap(); // Find row with top-left corner
    let start_col = grid[start_row].iter().position(|ch| *ch == '#').unwrap(); // Find column for top-left corner
    
    let mut bfs_queue = VecDeque::new();
    bfs_queue.push_back((start_row + 1, start_col + 1)); // Find first empty space within top-left corner
    while let Some((curr_row, curr_col)) = bfs_queue.pop_front() {
        grid[curr_row][curr_col] = '#';
        if curr_row > 0 && !visited[curr_row - 1][curr_col] {
            bfs_queue.push_back((curr_row - 1, curr_col));
            visited[curr_row - 1][curr_col] = true;
        }
        if curr_row + 1 < num_rows && !visited[curr_row + 1][curr_col] {
            bfs_queue.push_back((curr_row + 1, curr_col));
            visited[curr_row + 1][curr_col] = true;
        }
        if curr_col > 0 && !visited[curr_row][curr_col - 1] {
            bfs_queue.push_back((curr_row, curr_col - 1));
            visited[curr_row][curr_col - 1] = true;
        }
        if curr_col + 1 < num_cols && !visited[curr_row][curr_col + 1] {
            bfs_queue.push_back((curr_row, curr_col + 1));
            visited[curr_row][curr_col + 1] = true;
        }
    }

    let mut num_filled = 0;
    for row_num in 0..num_rows {
        for col_num in 0..num_cols {
            //print!("{}", grid[row_num][col_num]);
            if grid[row_num][col_num] == '#' {
                num_filled += 1;
            }
        }
        //println!("");
    }
    return num_filled;
}

fn part2(lines: &Vec<String>) -> i64 {
    // Parse instructions
    let mut instructions = Vec::new();
    for ln in lines {
        let (_, instr) = ln.rsplit_once(" ").unwrap();
        let step_count = i64::from_str_radix(&instr[2..7], 16).unwrap();
        let dir = match &instr[7..8] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => unreachable!()
        };
        instructions.push((dir, step_count));
    }

    // Calculate vertices of boundary
    // Adjust snapping based on direction of turn
    let mut vertices = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;
    for i in 0..instructions.len() {
        let (dir, step_count) = instructions[i];
        vertices.push((curr_row, curr_col));

        if dir == "U" {
            curr_row -= step_count;
        }
        else if dir == "D" {
            curr_row += step_count;
        }
        else if dir == "L" {
            curr_col -= step_count;
        }
        else if dir == "R" {
            curr_col += step_count;
        }
    }
    //println!("{:?}", vertices);

    // Calculate area
    let mut area = 0;
    for i in (0..vertices.len()) {
        area += 2 * ((vertices[i].0 + vertices[(i + 1) % vertices.len()].0) * (vertices[(i + 1) % vertices.len()].1 - vertices[i].1));

        // Add padding
        let line_len = (vertices[i].0 - vertices[(i + 1) % vertices.len()].0).abs() + (vertices[i].1 - vertices[(i + 1) % vertices.len()].1).abs();
        area -= 2 * (line_len - 1);
        
        // Check if corner is concave or convex, add padding appropriately
        let curr_dir = instructions[i].0;
        let next_dir = instructions[(i + 1) % instructions.len()].0;
        let is_convex = match curr_dir {
            "U" => (next_dir == "R"),
            "D" => (next_dir == "L"),
            "L" => (next_dir == "U"),
            "R" => (next_dir == "D"),
            _ => unreachable!()
        };
        if is_convex {
            area -= 3;
        }
        else {
            area -= 1;
        }
    }
    area /= -4;    
    return area;
}
