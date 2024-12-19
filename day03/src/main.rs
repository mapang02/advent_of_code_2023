use std::{collections::{HashMap, HashSet}, io};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> u32 {
    // Load input into padded character grid
    let num_rows = lines.len();
    let num_cols = lines[lines.len() - 1].len();
    let mut grid = vec![vec!['.'; num_cols + 2]; num_rows + 2];
    for i in 0..num_rows {
        for (j, ch) in lines[i].chars().enumerate() {
                grid[i + 1][j + 1] = ch;
        }
    }

    // Add up part numbers
    let mut subtotal = 0;
    for i in 1..(num_rows + 1) {
        let mut number_value = 0;
        let mut is_touching_symbol = false;
        for j in 1..(num_cols + 2) { // Includes right padding column so numbers finish building
            if grid[i][j].is_ascii_digit() { // Building number
                number_value *= 10;
                number_value += grid[i][j].to_digit(10).unwrap();

                let offset_coords: [(usize, usize); 8] = [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1), (i, j - 1), (i, j + 1), (i + 1, j - 1), (i + 1, j), (i + 1, j + 1)];
                for (off_x, off_y) in offset_coords {
                    if !grid[off_x][off_y].is_ascii_digit() && grid[off_x][off_y] != '.' {
                        is_touching_symbol = true;
                    }
                }
            }
            else { // Finish building number (if any)
                if number_value > 0 {
                    // println!("{number_value}, {is_touching_symbol}");
                }
                if is_touching_symbol {
                    subtotal += number_value;
                }
                number_value = 0;
                is_touching_symbol = false;
            }
        }
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> u32 {
    // Load input into padded character grid
    let num_rows = lines.len();
    let num_cols = lines[lines.len() - 1].len();
    let mut grid = vec![vec!['.'; num_cols + 2]; num_rows + 2];
    for i in 0..num_rows {
        for (j, ch) in lines[i].chars().enumerate() {
                grid[i + 1][j + 1] = ch;
        }
    }

    // Find all numbers and the gears they are adjacent to
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for i in 1..(num_rows + 1) {
        let mut number_value = 0;
        let mut number_start = (0, 0);
        let mut adj_gears: HashSet<(usize, usize)> = HashSet::new();
        for j in 1..(num_cols + 2) { // Includes right padding column so numbers finish building
            if grid[i][j].is_ascii_digit() { // Building number
                if number_start == (0, 0) {
                    number_start = (i, j);
                }

                number_value *= 10;
                number_value += grid[i][j].to_digit(10).unwrap();

                let offset_coords: [(usize, usize); 8] = [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1), (i, j - 1), (i, j + 1), (i + 1, j - 1), (i + 1, j), (i + 1, j + 1)];
                for (off_x, off_y) in offset_coords {
                    if grid[off_x][off_y] == '*' {
                        adj_gears.insert((off_x, off_y));
                    }
                }
            }
            else { // Finish building number (if any)
                if number_start != (0, 0) {
                    //println!("{}, {:?}, {:?}", number_value, number_start, adj_gears);
                    for g in adj_gears {
                        let gear_num_list = gears.entry(g).or_insert(Vec::new());
                        gear_num_list.push(number_value);
                    }
                }
                number_value = 0;
                number_start = (0, 0);
                adj_gears = HashSet::new();
            }
        }
    }
    //println!("{:?}", gears);

    // Find gear number for each gear adjacent to exactly two numbers
    let mut subtotal = 0;
    for adj_nums in gears.values() {
        if adj_nums.len() == 2 {
            subtotal += adj_nums[0] * adj_nums[1];
        }
    }
    return subtotal;
}