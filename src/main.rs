use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_str = fs::read_to_string(file_path).expect("Could not read file").replace("\r\n", "\n");
    let part1 = part1(&file_str);
    let part2 = part2(&file_str);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(file_str: &String) -> i64 {
    let (seed_line, file_str) = file_str.split_once("\n").unwrap();
    
    // Create map ranges
    let map_descriptors = file_str.trim().split("\n\n");
    let mut maps = Vec::new();
    for map_str in map_descriptors {
        let mut map_ranges = Vec::new();
        for line in map_str.split("\n").skip(1) {
            let line_nums: Vec<i64> = line.split(" ").map(|n| n.parse().unwrap()).collect();
            // Output in src_start, range_size, offset format
            map_ranges.push((line_nums[1], line_nums[2], line_nums[0] - line_nums[1]));
        }
        maps.push(map_ranges);
    }
    
    // Get seed numbers and determine end locations
    let seed_nums: Vec<i64> = seed_line[(seed_line.find(":").unwrap() + 1)..]
                                .trim()
                                .split(" ")
                                .map(|n| n.parse().unwrap())
                                .collect();
    let mut min_location = std::i64::MAX;
    for mut n in seed_nums {
        for map_ranges in &maps {
            for (src_start, range_size, offset) in map_ranges {
                if n >= *src_start && n < src_start + range_size {
                    n = n + offset;
                    break;
                }
            }
        }
        if n < min_location {
            min_location = n;
        }
        //println!("{n}");
    }

    return min_location;
}

fn part2(file_str: &String) -> i64 {
    let (seed_line, file_str) = file_str.split_once("\n").unwrap();
    
    // Create map ranges
    let map_descriptors = file_str.trim().split("\n\n");
    let mut maps = Vec::new();
    for map_str in map_descriptors {
        let mut map_ranges = Vec::new();
        for line in map_str.split("\n").skip(1) {
            let line_nums: Vec<i64> = line.split(" ").map(|n| n.parse().unwrap()).collect();
            // Output in src_start, range_size, offset format
            map_ranges.push((line_nums[1], line_nums[2], line_nums[0] - line_nums[1]));
        }
        maps.push(map_ranges);
    }

    // Calculate seed numbers corresponding to range endpoints (candidates for minimum location)
    let mut range_endpoints = HashSet::new();
    for i in 0..maps.len() {
        for (range_start, range_size, _) in &maps[i] {
            // Search backwards to determine seed number corresponding to current map start
            let mut range_start_seed_num = *range_start;
            let mut start_backtrack_debug = String::from(format!("{range_start_seed_num}"));
            for j in (0..i).rev() {
                for (prev_range_start, prev_range_size, prev_range_offset) in &maps[j] {
                    if (range_start_seed_num >= prev_range_start + prev_range_offset) && 
                        (range_start_seed_num < prev_range_start + prev_range_size + prev_range_offset) {
                            range_start_seed_num -= prev_range_offset;
                    }
                }
                start_backtrack_debug.push_str(&format!(" -> {range_start_seed_num}"));
            }
            //println!("{}", &start_backtrack_debug);
            range_endpoints.insert(range_start_seed_num);
            
            // Search backwards to determine seed number corresponding to current map end
            let range_end = range_start + range_size - 1;
            let mut range_end_seed_num = range_end;
            let mut end_backtrack_debug = String::from(format!("{range_end_seed_num}"));
            for j in (0..i).rev() {
                for (prev_range_start, prev_range_size, prev_range_offset) in &maps[j] {
                    if (range_end_seed_num >= prev_range_start + prev_range_offset) && 
                        (range_end_seed_num < prev_range_start + prev_range_size + prev_range_offset) {
                            range_end_seed_num -= prev_range_offset;
                    }
                }
                end_backtrack_debug.push_str(&format!(" -> {range_end_seed_num}"));
            }
            //println!("{}", &end_backtrack_debug);
            range_endpoints.insert(range_end_seed_num);
        }
    }

    
    // Get seed ranges and determine end locations for range endpoints
    let seed_nums: Vec<i64> = seed_line[(seed_line.find(":").unwrap() + 1)..]
                                .trim()
                                .split(" ")
                                .map(|n| n.parse().unwrap())
                                .collect();
    let seed_ranges: Vec<(i64, i64)> = (0..(seed_nums.len() / 2)).map(|i| (seed_nums[2 * i], seed_nums[2 * i + 1])).collect();
    for (range_start, range_size) in &seed_ranges { // Insert seed number range endpoints into set
        range_endpoints.insert(*range_start);
        range_endpoints.insert(range_start + range_size);
    }
    
    let mut min_location = std::i64::MAX;
    'endpoint_check: for n in range_endpoints {
        // Check if n is in seed range
        let mut is_valid_seed = false;
        for (range_start, range_size) in &seed_ranges {
            if n >= *range_start && n < range_start + range_size {
                is_valid_seed = true;
                break;
            }
        }
        if !is_valid_seed {
            continue 'endpoint_check;
        }

        // Evaluate end location
        let mut end_loc = n;
        for map_ranges in &maps {
            for (src_start, range_size, offset) in map_ranges {
                if end_loc >= *src_start && end_loc < src_start + range_size {
                    end_loc = end_loc + offset;
                    break;
                }
            }
        }
        if end_loc < min_location {
            min_location = end_loc;
        }
        println!("Seed {n} -> Location {end_loc}");
    }

    return min_location;
}