use core::num;
use std::{cmp, collections::{BTreeSet, HashMap, HashSet}, io};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    /*
    let part1 = part1(&lines);
    println!("Part 1: {part1}");
    let part2 = part2(&lines);
    println!("Part 2: {part2}");
    */
    let (part1, part2) = parts_unified(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn parts_unified(lines: &Vec<String>) -> (u32, u32) {
    // Parse input
    let mut brick_map: HashMap<usize, ((usize, usize, usize), (usize, usize, usize))> = HashMap::new();
    for (i, ln) in lines.iter().enumerate() {
        let nums: Vec<usize> = ln.splitn(6, &[',', '~']).map(|n| n.parse().unwrap()).collect();
        let start_point = (cmp::min(nums[0], nums[3]), cmp::min(nums[1], nums[4]), cmp::min(nums[2], nums[5]));
        let end_point = (cmp::max(nums[0], nums[3]), cmp::max(nums[1], nums[4]), cmp::max(nums[2], nums[5]));
        brick_map.insert(i, (start_point, end_point));
    }
    //println!("{:?}", brick_map);
    let (brick_map, _) = simulate_drop_unified(&brick_map);
    //println!("{:?}", brick_map);

    let mut total_removable_bricks = 0;
    let mut total_dropped_bricks = 0;
    for removed_brick_id in brick_map.keys() {
        let mut brick_map_copy = brick_map.clone();
        brick_map_copy.remove(removed_brick_id);
        let (map_after_drop, num_dropped_bricks) = simulate_drop_unified(&brick_map_copy);
        if num_dropped_bricks > 0 {
            total_dropped_bricks += num_dropped_bricks;
        }
        else {
            total_removable_bricks += 1;
        }
    } 
    return (total_removable_bricks, total_dropped_bricks);
}

fn simulate_drop_unified(brick_map: &HashMap<usize, ((usize, usize, usize), (usize, usize, usize))>) -> (HashMap<usize, ((usize, usize, usize), (usize, usize, usize))>, u32) {
    let mut new_brick_map = brick_map.clone();

    // Set up occupied map
    let max_x = new_brick_map.values().map(|(_, p2)| p2.0).fold(0, |a, b| if b > a { b } else { a });
    let max_y = new_brick_map.values().map(|(_, p2)| p2.1).fold(0, |a, b| if b > a { b } else { a });
    let max_z = new_brick_map.values().map(|(_, p2)| p2.2).fold(0, |a, b| if b > a { b } else { a });
    let mut occupied_map = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];
    
    for (p1, p2) in new_brick_map.values() {
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x][y][z] = true;
        }
    }

    // Sort bricks by lowest point and move each one downwards
    let mut ordered_bricks: Vec<(usize, ((usize, usize, usize), (usize, usize, usize)))> = new_brick_map.iter().map(|(k, v)| (*k, *v)).collect();
    ordered_bricks.sort_by_key(|(k, (p1, p2))| p1.2);
    //println!("{:?}", ordered_bricks);

    let mut critical_z_vals = BTreeSet::from_iter(ordered_bricks.iter().map(|(_, (_, p2))| p2.2));
    critical_z_vals.insert(0);
    //println!("{:?}", critical_z_vals);
    // This can be slightly optimized by making critical_z_vals into a BTreeMap that counts how many bricks correspond to each critical value
    // Then critical values can be removed once there are no more bricks at that level

    let mut num_dropped_bricks = 0;
    for (brick_id, (p1, p2)) in ordered_bricks.iter() {
        // Remove current brick from map
        new_brick_map.remove(brick_id);
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x][y][z] = false;
        }

        // Repeatedly pick next lowest z-value to test
        let mut final_z_min = 1;
        for z in critical_z_vals.range(0..p1.2).rev() {
            let new_z_min = *z;
            let new_z_max = p2.2 - (p1.2 - z);
            if (*z == 0) || !between_points((p1.0, p1.1, new_z_min), (p2.0, p2.1, new_z_max)).into_iter().all(|(x, y, z)| !occupied_map[x][y][z]) {
                final_z_min = *z + 1;
                break;
            }
        }
        for (x, y, z) in between_points((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)).into_iter() {
            occupied_map[x][y][z] = true;
        }
        new_brick_map.insert(*brick_id, ((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)));
        if final_z_min != p1.2 {
            critical_z_vals.insert(final_z_min);
            num_dropped_bricks += 1;
        }
    }

    return (new_brick_map, num_dropped_bricks);
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let mut brick_map: HashMap<usize, ((usize, usize, usize), (usize, usize, usize))> = HashMap::new();
    for (i, ln) in lines.iter().enumerate() {
        let nums: Vec<usize> = ln.splitn(6, &[',', '~']).map(|n| n.parse().unwrap()).collect();
        let start_point = (cmp::min(nums[0], nums[3]), cmp::min(nums[1], nums[4]), cmp::min(nums[2], nums[5]));
        let end_point = (cmp::max(nums[0], nums[3]), cmp::max(nums[1], nums[4]), cmp::max(nums[2], nums[5]));
        brick_map.insert(i, (start_point, end_point));
    }
    //println!("{:?}", brick_map);
    let brick_map = simulate_drop(&brick_map);
    //println!("{:?}", brick_map);

    let mut subtotal = 0;
    for removed_brick_id in brick_map.keys() {
        let mut brick_map_copy = brick_map.clone();
        brick_map_copy.remove(removed_brick_id);
        let map_after_drop = simulate_drop(&brick_map_copy);
        if brick_map_copy == map_after_drop {
            subtotal += 1;
        }
    } 
    return subtotal;
}

fn part2(lines: &Vec<String>) -> u32 {
    // Parse input
    let mut brick_map: HashMap<usize, ((usize, usize, usize), (usize, usize, usize))> = HashMap::new();
    for (i, ln) in lines.iter().enumerate() {
        let nums: Vec<usize> = ln.splitn(6, &[',', '~']).map(|n| n.parse().unwrap()).collect();
        let start_point = (cmp::min(nums[0], nums[3]), cmp::min(nums[1], nums[4]), cmp::min(nums[2], nums[5]));
        let end_point = (cmp::max(nums[0], nums[3]), cmp::max(nums[1], nums[4]), cmp::max(nums[2], nums[5]));
        brick_map.insert(i, (start_point, end_point));
    }
    //println!("{:?}", brick_map);
    let brick_map = simulate_drop(&brick_map);
    //println!("{:?}", brick_map);

    let mut subtotal = 0;
    for removed_brick_id in brick_map.keys() {
        let mut brick_map_copy = brick_map.clone();
        brick_map_copy.remove(removed_brick_id);
        let map_after_drop = simulate_drop(&brick_map_copy);

        for (brick_id, pos) in brick_map_copy {
            if pos != *map_after_drop.get(&brick_id).unwrap() {
                subtotal += 1;
            }
        }
    } 
    return subtotal;
}

fn simulate_drop(brick_map: &HashMap<usize, ((usize, usize, usize), (usize, usize, usize))>) -> HashMap<usize, ((usize, usize, usize), (usize, usize, usize))> {
    let mut new_brick_map = brick_map.clone();

    // Set up occupied map
    let max_x = new_brick_map.values().map(|(_, p2)| p2.0).fold(0, |a, b| if b > a { b } else { a });
    let max_y = new_brick_map.values().map(|(_, p2)| p2.1).fold(0, |a, b| if b > a { b } else { a });
    let max_z = new_brick_map.values().map(|(_, p2)| p2.2).fold(0, |a, b| if b > a { b } else { a });
    let mut occupied_map = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];
    
    for (p1, p2) in new_brick_map.values() {
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x][y][z] = true;
        }
    }

    // Sort bricks by lowest point and move each one downwards
    let mut ordered_bricks: Vec<(usize, ((usize, usize, usize), (usize, usize, usize)))> = new_brick_map.iter().map(|(k, v)| (*k, *v)).collect();
    ordered_bricks.sort_by_key(|(k, (p1, p2))| p1.2);
    //println!("{:?}", ordered_bricks);

    let mut critical_z_vals = BTreeSet::from_iter(ordered_bricks.iter().map(|(_, (p1, _))| p1.2));
    critical_z_vals.insert(0);
    //println!("{:?}", critical_z_vals);

    for (brick_id, (p1, p2)) in ordered_bricks.iter() {
        // Remove current brick from map
        new_brick_map.remove(brick_id);
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x][y][z] = false;
        }

        // Repeatedly pick next lowest z-value to test
        let mut final_z_min = 1;
        for z in critical_z_vals.range(0..p1.2).rev() {
            let new_z_min = *z;
            let new_z_max = p2.2 - (p1.2 - z);
            if (*z == 0) || !between_points((p1.0, p1.1, new_z_min), (p2.0, p2.1, new_z_max)).into_iter().all(|(x, y, z)| !occupied_map[x][y][z]) {
                final_z_min = *z + 1;
                break;
            }
        }
        for (x, y, z) in between_points((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)).into_iter() {
            occupied_map[x][y][z] = true;
        }
        new_brick_map.insert(*brick_id, ((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)));
        critical_z_vals.insert(final_z_min);
    }

    return new_brick_map;
}

fn between_points(p1: (usize, usize, usize), p2: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    if p1.0 != p2.0 {
        return (p1.0..(p2.0 + 1)).map(|x| (x, p1.1, p1.2)).collect();
    }
    else if p1.1 != p2.1 {
        return (p1.1..(p2.1 + 1)).map(|y| (p1.0, y, p1.2)).collect();
    }
    return (p1.2..(p2.2 + 1)).map(|z| (p1.0, p1.1, z)).collect();
}