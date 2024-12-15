use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

struct Vector {
    x: f64,
    y: f64,
    z: f64
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let hailstones: Vec<(Vector, Vector)> = lines.iter().map(|ln| {
        let (pos_str, vel_str) = ln.split_once("@").unwrap();
        //let pos: Vec<_> = pos_str.trim().splitn(3, ", ").map(|s| s.parse::<i64>().unwrap()).collect();
        //let vel: Vec<_> = vel_str.trim().splitn(3, ", ").map(|s| s.parse::<i64>().unwrap()).collect();
        let pos: Vec<_> = pos_str.splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        let vel: Vec<_> = vel_str.splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        return (
            Vector { x: pos[0], y: pos[1], z: pos[2] },
            Vector { x: vel[0], y: vel[1], z: vel[2] }
        )
    }).collect();
    //println!("{:?}", hailstones);

    // Check collisions
    let mut num_collisions = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let (h1_pos, h1_vel) = &hailstones[i];
            let (h2_pos, h2_vel) = &hailstones[j];

            // Solve system of equations p_x1 + v_x1 * t_1 = p_x2 + v_x2 * t_2, p_y1 + v_y1 * t_1 = p_y2 + v_y2 * t_2
            let inv_const = (h1_vel.x * -h2_vel.y) - -(h2_vel.x * h1_vel.y);
            if inv_const == 0.0 {
                // If inv_const is 0, then the velocities are collinear
                // Assume that input does not contain collinear paths that do collide, since the collision point would be indeterminate
                continue;
            }
            let rhs = (h2_pos.x - h1_pos.x, h2_pos.y - h1_pos.y);
            let t1 = (-h2_vel.y * rhs.0 + h2_vel.x * rhs.1) / inv_const;
            let t2 = (-h1_vel.y * rhs.0 + h1_vel.x * rhs.1) / inv_const;

            let min_pos = 200000000000000.0;
            let max_pos = 400000000000000.0;
            //let MIN_POS = 7.0;
            //let MAX_POS = 27.0;
            let (collision_x, collision_y) = (h1_pos.x + h1_vel.x * t1, h1_pos.y + h1_vel.y * t1);
            //println!("Hailstones {}, {} at ({}, {}) at t = ({}, {})", i, j, collision_x, collision_y, t1, t2);
            if (t1 >= 0.0 && t2 >= 0.0) && (collision_x >= min_pos && collision_x <= max_pos) && (collision_y >= min_pos && collision_y <= max_pos) {
                num_collisions += 1;
            }
        }
    }
    return num_collisions;
}

fn part2(lines: &Vec<String>) -> u32 {
    return 0;   
}
