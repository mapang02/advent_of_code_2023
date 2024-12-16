#[macro_use]
extern crate nalgebra;
use std::io;
use nalgebra::{Matrix6, Vector6};
use fraction::Fraction;

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

fn cross_product(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    return (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0);
}

fn part2(lines: &Vec<String>) -> u32 {
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

    // Solve a system of linear equations to determine p0_x, p0_y, v0_x, v0_y
    // Perform Gaussian elimination using fraction objects to avoid precision errors
    let mut m_vec = vec![0.0; 36];
    let mut b_vec = vec![0.0; 6];
    for i in 0..6 {
        let (pos_i, vel_i) = &hailstones[i];
        /*
        m_vec[0] = vel_i.y;
        m_vec[6 + i] = -vel_i.x;
        m_vec[12 + i] = -pos_i.y;
        m_vec[18 + i] = pos_i.x;
        m_vec[24 + i] = -1.0;
        m_vec[30 + i] = 1.0;
        */
        m_vec[6 * i] = vel_i.y;
        m_vec[6 * i + 1] = -vel_i.x;
        m_vec[6 * i + 2] = -pos_i.y;
        m_vec[6 * i + 3] = pos_i.x;
        m_vec[6 * i + 4] = -1.0;
        m_vec[6 * i + 5] = 1.0;
        /*
        m_vec[0] = (6 * i) as f64;
        m_vec[6 + i] = (6 * i + 1) as f64;
        m_vec[12 + i] = (6 * i + 2) as f64;
        m_vec[18 + i] = (6 * i + 3) as f64;
        m_vec[24 + i] = (6 * i + 4) as f64;
        m_vec[30 + i] = (6 * i + 5) as f64;
        m_vec[6 * i] = (6 * i) as f64;
        m_vec[6 * i + 1] = (6 * i + 1) as f64;
        m_vec[6 * i + 2] = (6 * i + 2) as f64;
        m_vec[6 * i + 3] = (6 * i + 3) as f64;
        m_vec[6 * i + 4] = (6 * i + 4) as f64;
        m_vec[6 * i + 5] = (6 * i + 5) as f64;
        */

        b_vec[i] = pos_i.x * vel_i.y - pos_i.y * vel_i.x;
    }
    let m = Matrix6::from_vec(m_vec.iter().map(|n| Fraction::from(n as i64)).collect());
    let b = Vector6::from_vec(b_vec);
    let m_inv = m.try_inverse().unwrap();
    println!("{:?}", m);
    println!("{:?}", b);
    println!("{:?}", m_inv * b);

    return 0;   
}
