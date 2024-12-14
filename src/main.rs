use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

struct Point {
    x: f64,
    y: f64,
    z: f64
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let hailstones: Vec<((f64, f64, f64), (f64, f64, f64))> = lines.iter().map(|ln| {
        let (pos_str, vel_str) = ln.split_once("@").unwrap();
        //let pos: Vec<_> = pos_str.trim().splitn(3, ", ").map(|s| s.parse::<i64>().unwrap()).collect();
        //let vel: Vec<_> = vel_str.trim().splitn(3, ", ").map(|s| s.parse::<i64>().unwrap()).collect();
        let pos: Vec<_> = pos_str.splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        let vel: Vec<_> = vel_str.splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        return (
            (pos[0], pos[1], pos[2]),
            (vel[0], vel[1], vel[2])
        )
    }).collect();
    //println!("{:?}", hailstones);

    // Check collisions
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let (h1_pos, h1_vel) = hailstones[i];
            let (h2_pos, h2_vel) = hailstones[j];

            // Solve system of equations p_x1 + v_x1 * t_1 = p_x2 + v_x2 * t_2, p_y1 + v_y1 * t_1 = p_y2 + v_y2 * t_2
            let inv_const = (h1_vel.0 * -h2_vel.1) - -(h2_vel.0 * h1_vel.1);
            if inv_const == 0.0 {
                // If inv_const is 0, then the velocities are collinear
            }
            let rhs = (h2_pos.1 - h1_pos.0, h2_pos.1 - h1_pos.1);
            let t_1 = 
        }
    }

    return 0;
}

fn part2(lines: &Vec<String>) -> u32 {
    return 0;   
}
