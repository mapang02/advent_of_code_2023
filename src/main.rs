use core::num;
use std::{array, collections::{HashMap, HashSet, VecDeque}, hash::Hash, io};

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock
}

fn print_map(map: &Vec<Vec<Tile>>, reachable: &Vec<Vec<bool>>) {
    let width = map[0].len();
    let height = map.len();
    for j in 0..height {
        for i in 0..width {
            if reachable[i][j] {
                print!("O");
            }
            else if map[i][j] == Tile::Garden {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!("");
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let width = lines[0].len();
    let height = lines.len();

    // Validate input, convert to map
    let tile_chars = lines.concat();
    if tile_chars.len() != width * height {
        println!("Invalid input, width = {}, height = {}, len = {}", width, height, width * height);
        return 0;
    }
    let start_index = tile_chars.find('S').unwrap_or(0);
    let start_x = start_index % height + 1; // Offset taking into account sentinel
    let start_y = start_index / height + 1; // Offset taking into account sentinel
    
    let mut map = vec![vec![Tile::Rock; height + 2]; width + 2]; // Extra rows and columns added for sentinel values
    for (i, ch) in tile_chars.char_indices() {
        let tile_x = i % height;
        let tile_y = i / height;
        map[tile_x + 1][tile_y + 1] = match ch {
            '#' => Tile::Rock,
            _ => Tile::Garden
        }
    }

    // Iterate from reachable positions
    let num_steps = 64;
    let mut reachable = vec![vec![false; height + 2]; width + 2];
    reachable[start_x][start_y] = true;
    for _ in 0..num_steps {
        let mut new_reachable = vec![vec![false; height + 2]; width + 2];
        for j in 1..(height + 1) {
            for i in 1..(width + 1) {
                if map[i][j] == Tile::Garden && (reachable[i - 1][j] || reachable[i + 1][j] || reachable[i][j - 1] || reachable[i][j + 1]) {
                    new_reachable[i][j] = true;
                }
            }
        }
        reachable = new_reachable;
    }

    // Count reachable positions
    //print_map(&map, &reachable);
    return reachable.into_iter().flatten().filter(|r| *r).count() as i32;
}

fn part2(lines: &Vec<String>) -> u64 {
    let width = lines[0].len();
    let height = lines.len();

    // Validate input, convert to map
    let tile_chars = lines.concat();
    if tile_chars.len() != width * height {
        println!("Invalid input, width = {}, height = {}, len = {}", width, height, width * height);
        return 0;
    }
    let mut tile_map = vec![vec![Tile::Rock; width]; height];
    for (i, ch) in tile_chars.char_indices() {
        let tile_x = i % height;
        let tile_y = i / height;
        tile_map[tile_y][tile_x] = match ch {
            '#' => Tile::Rock,
            _ => Tile::Garden
        }
    }

    // Simulate 5x5 to find reachable tile count on each tile variation
    let sim_map_width = 5 * width;
    let sim_map_height = 5 * height;
    let mut sim_tile_map = vec![vec![Tile::Rock; sim_map_width]; sim_map_height]; // Simulate a 5x5 in order to get all odd/even and corner variations
    for chunk_y in 0..5 {
        for chunk_x in 0..5 {
            for (i, ch) in tile_chars.char_indices() {
                let tile_x = i % height;
                let tile_y = i / height;
                sim_tile_map[tile_y + chunk_y * height][tile_x + chunk_x * width] = match ch {
                    '#' => Tile::Rock,
                    _ => Tile::Garden
                }
            }
        }
    }

    let mut sim_distances = vec![vec![std::usize::MAX; sim_map_width]; sim_map_height];
    let mut sim_bfs_queue = VecDeque::new();

    sim_distances[sim_map_height / 2][sim_map_width / 2] = 0;
    sim_bfs_queue.push_back((sim_map_height / 2, sim_map_width / 2));
    while !sim_bfs_queue.is_empty() {
        let (x, y) = sim_bfs_queue.pop_front().unwrap();
        let curr_tile_distance = sim_distances[y][x];
        if x + 1 < sim_map_width && sim_tile_map[y][x + 1] == Tile::Garden && sim_distances[y][x + 1] == std::usize::MAX {
            sim_distances[y][x + 1] = curr_tile_distance + 1;
            sim_bfs_queue.push_back((x + 1, y));
        }
        if x >= 1 && sim_tile_map[y][x - 1] == Tile::Garden && sim_distances[y][x - 1] == std::usize::MAX {
            sim_distances[y][x - 1] = curr_tile_distance + 1;
            sim_bfs_queue.push_back((x - 1, y));
        }
        if y + 1 < sim_map_height && sim_tile_map[y + 1][x] == Tile::Garden && sim_distances[y + 1][x] == std::usize::MAX {
            sim_distances[y + 1][x] = curr_tile_distance + 1;
            sim_bfs_queue.push_back((x, y + 1));
        }
        if y >= 1 && sim_tile_map[y - 1][x] == Tile::Garden && sim_distances[y - 1][x] == std::usize::MAX {
            sim_distances[y - 1][x] = curr_tile_distance + 1;
            sim_bfs_queue.push_back((x, y - 1));
        }
    }

    let num_sim_steps = 2 * width + width / 2;
    let reachable: Vec<Vec<bool>> = sim_distances.iter().map(|row| row.iter().map(|d| *d <= num_sim_steps && (d + num_sim_steps) % 2 == 0).collect()).collect();
    /*
    for j in 0..sim_map_height {
        for i in 0..sim_map_width {
            if reachable[j][i] {
                if sim_tile_map[j][i] == Tile::Garden {
                    print!("O");
                }
                else {
                    println!("Error: Tile ({i}, {j}) is reachable despite being a rock");
                }
            }
            else {
                if sim_tile_map[j][i] == Tile::Garden {
                    print!(".");
                }
                else {
                    print!("#");
                }
            }
        }
        println!("");
    }
    */

    let full_odd_reach = count_reachable(&reachable, 2 * width, 3 * width, 2 * height, 3 * height);
    let full_even_reach = count_reachable(&reachable, 3 * width, 4 * width, 2 * height, 3 * height);

    let odd_corner_top_reach = count_reachable(&reachable, 2 * width, 3 * width, 0, height);
    let odd_corner_bottom_reach = count_reachable(&reachable, 2 * width, 3 * width, 4 * height, 5 * height);
    let even_corner_tl_reach = count_reachable(&reachable, width, 2 * width, 0, height);
    let even_corner_tr_reach = count_reachable(&reachable, 3 * width, 4 * width, 0, height);
    let even_corner_bl_reach = count_reachable(&reachable, width, 2 * width, 4 * height, 5 * height);
    let even_corner_br_reach = count_reachable(&reachable, 3 * width, 4 * width, 4 * height, 5 * height);
    let even_corner_reach = even_corner_tl_reach + even_corner_tr_reach + even_corner_bl_reach + even_corner_br_reach;
    let odd_corner_reach = 2 * full_odd_reach - (odd_corner_top_reach + odd_corner_bottom_reach);
    
    println!("full_odd_reach: {full_odd_reach}, full_even_reach: {full_even_reach}");
    println!("odd_corner_top_reach: {odd_corner_top_reach}, odd_corner_bottom_reach: {odd_corner_bottom_reach}");
    println!("even_corner_tl_reach: {even_corner_tl_reach}, even_corner_tr_reach: {even_corner_tr_reach}, even_corner_bl_reach: {even_corner_bl_reach}, even_corner_br_reach: {even_corner_br_reach}");

    // Compute total reachable tiles
    let num_steps = 26501365;
    let chunk_radius: u64 = ((num_steps - height / 2) / height).try_into().unwrap();
    println!("chunk_radius: {chunk_radius}");
    println!("full_even_reach: {full_even_reach}, full_odd_reach: {full_odd_reach}, even_corner_reach: {even_corner_reach}, odd_corner_reach: {odd_corner_reach}");
    
    let num_odd_chunks = (chunk_radius + 1) * (chunk_radius + 1);
    let num_even_chunks = chunk_radius * chunk_radius;
    let num_odd_corners = (chunk_radius + 1); // Odd corners are removed
    let num_even_corners = chunk_radius;

    let total = num_even_chunks * full_even_reach + num_odd_chunks * full_odd_reach + num_even_corners * even_corner_reach - num_odd_corners * odd_corner_reach;
    println!("Total: {}", total);
    return total;
}

fn count_reachable(reachable: &Vec<Vec<bool>>, start_x: usize, end_x: usize, start_y: usize, end_y: usize) -> u64 {
    let mut num_reachable = 0;
    for j in start_y..end_y {
        for i in start_x..end_x {
            if reachable[j][i] {
                num_reachable += 1;
            }
        }
    }
    return num_reachable;
}