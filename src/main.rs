use std::{cmp, collections::{BTreeSet, HashMap, HashSet}, io};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    let part1 = part1(&lines);
    println!("Part 1: {part1}");
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let mut brick_map: HashMap<usize, ((i32, i32, i32), (i32, i32, i32))> = HashMap::new();
    for (i, ln) in lines.iter().enumerate() {
        let nums: Vec<i32> = ln.splitn(6, &[',', '~']).map(|n| n.parse().unwrap()).collect();
        let start_point = (cmp::min(nums[0], nums[3]), cmp::min(nums[1], nums[4]), cmp::min(nums[2], nums[5]));
        let end_point = (cmp::max(nums[0], nums[3]), cmp::max(nums[1], nums[4]), cmp::max(nums[2], nums[5]));
        brick_map.insert(i, (start_point, end_point));
    }
    //println!("{:?}", brick_map);
    let brick_map = simulate_drop(&brick_map);
    //println!("{:?}", brick_map);

    let mut subtotal = 0;
    for i in brick_map.keys() {
        let mut brick_map_copy = brick_map.clone();
        brick_map_copy.remove(i);
        let dropped_map = simulate_drop(&brick_map_copy);
        if brick_map_copy == dropped_map {
            subtotal += 1;
        }
    } 
    return subtotal;
}

fn simulate_drop(brick_map: &HashMap<usize, ((i32, i32, i32), (i32, i32, i32))>) -> HashMap<usize, ((i32, i32, i32), (i32, i32, i32))> {
    let mut new_brick_map = brick_map.clone();

    // Set up occupied map
    let max_x = new_brick_map.values().map(|(_, p2)| p2.0).fold(0, |a, b| if b > a { b } else { a }) as usize;
    let max_y = new_brick_map.values().map(|(_, p2)| p2.1).fold(0, |a, b| if b > a { b } else { a }) as usize;
    let max_z = new_brick_map.values().map(|(_, p2)| p2.2).fold(0, |a, b| if b > a { b } else { a }) as usize;
    let mut occupied_map = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];
    
    for (p1, p2) in new_brick_map.values() {
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x as usize][y as usize][z as usize] = true;
        }
    }

    // Sort bricks by lowest point and move each one downwards
    let mut ordered_bricks: Vec<(usize, ((i32, i32, i32), (i32, i32, i32)))> = new_brick_map.iter().map(|(k, v)| (*k, *v)).collect();
    ordered_bricks.sort_by_key(|(k, (p1, p2))| p1.2);
    //println!("{:?}", ordered_bricks);

    let mut critical_z_vals = BTreeSet::from_iter(ordered_bricks.iter().map(|(_, (p1, _))| p1.2));
    critical_z_vals.insert(0);
    //println!("{:?}", critical_z_vals);

    for (brick_id, (p1, p2)) in ordered_bricks.iter() {
        // Remove current brick from map
        new_brick_map.remove(brick_id);
        for (x, y, z) in between_points(*p1, *p2) {
            occupied_map[x as usize][y as usize][z as usize] = false;
        }

        // Repeatedly pick next lowest z-value to test
        let mut final_z_min = 1;
        for z in critical_z_vals.range(0..p1.2).rev() {
            let new_z_min = *z;
            let new_z_max = p2.2 - (p1.2 - z);
            if (*z == 0) || !between_points((p1.0, p1.1, new_z_min), (p2.0, p2.1, new_z_max)).into_iter().all(|(x, y, z)| !occupied_map[x as usize][y as usize][z as usize]) {
                final_z_min = *z + 1;
                break;
            }
        }
        for (x, y, z) in between_points((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)).into_iter() {
            occupied_map[x as usize][y as usize][z as usize] = true;
        }
        new_brick_map.insert(*brick_id, ((p1.0, p1.1, final_z_min), (p2.0, p2.1, p2.2 + final_z_min - p1.2)));
        critical_z_vals.insert(final_z_min);
    }

    return new_brick_map;
}

fn between_points(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    if p1.0 != p2.0 {
        return (p1.0..(p2.0 + 1)).map(|x| (x, p1.1, p1.2)).collect();
    }
    else if p1.1 != p2.1 {
        return (p1.1..(p2.1 + 1)).map(|y| (p1.0, y, p1.2)).collect();
    }
    return (p1.2..(p2.2 + 1)).map(|z| (p1.0, p1.1, z)).collect();
}

fn intersects(p1: (i32, i32, i32), p2: (i32, i32, i32), p3: (i32, i32, i32), p4: (i32, i32, i32)) -> bool {
    // From https://paulbourke.net/geometry/pointlineplane/
    fn collinear(a: (i32, i32, i32), b: (i32, i32, i32), c: (i32, i32, i32)) -> bool {
        // Check if cross product of vectors is 0, triangle of zero area indicates collinearity
        let ab_vec = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
        let ac_vec = (c.0 - a.0, c.1 - a.1, c.2 - a.2);
        return (ab_vec.1 * ac_vec.2 - ab_vec.2 * ac_vec.2 == 0) && (ab_vec.2 * ac_vec.0 - ab_vec.0 * ac_vec.2 == 0) && (ab_vec.0 * ac_vec.1 - ab_vec.1 * ac_vec.0 == 0);
    }
    fn d(m: (i32, i32, i32), n: (i32, i32, i32), o: (i32, i32, i32), p: (i32, i32, i32)) -> i32 {
        return (m.0 - n.0) * (o.0 - p.0) + (m.1 - n.1) * (o.1 - p.1) + (m.2 - n.2) * (o.2 - p.2);
    }
    if p1 == p2 {
        return collinear(p1, p3, p4);
    }
    else if p3 == p4 {
        return collinear(p3, p1, p2);
    }

    let mu_a_denom = d(p2, p1, p2, p1) * d(p4, p3, p4, p3) - d(p4, p3, p2, p1) * d(p4, p3, p2, p1);
    let mu_b_denom = d(p4, p3, p4, p3);
    if (mu_a_denom == 0) || (mu_b_denom == 0) {
        return false;
    }
    let mu_a_num = d(p1, p3, p4, p3) * d(p4, p3, p2, p1) - mu_b_denom * mu_b_denom;
    let mu_a = (mu_a_num as f32) / (mu_a_denom as f32);
    let mu_b = ((d(p1, p3, p4, p3) as f32) + mu_a * (d(p4, p3, p2, p1) as f32)) / (mu_b_denom as f32);

    return (mu_a >= 0.0) && (mu_a <= 1.0) && (mu_b >= 0.0) && (mu_b <= 1.0);
}