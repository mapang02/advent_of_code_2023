#[macro_use]
extern crate nalgebra;
extern crate fraction;
use std::{io, ops::{Add, Div, Mul}, str::FromStr};
use fraction::{Fraction, GenericFraction, ToPrimitive};
type F128 = GenericFraction<i128>;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

struct Vector<T: Div<Output = T>> {
    x: T,
    y: T,
    z: T
}

fn part1(lines: &Vec<String>) -> u32 {
    // Parse input
    let hailstones: Vec<(Vector<f64>, Vector<f64>)> = lines.iter().map(|ln| {
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

fn part2(lines: &Vec<String>) -> u64 {
    // Parse input
    let hailstones: Vec<(Vector<_>, Vector<_>)> = lines.iter().map(|ln| {
        let (pos_str, vel_str) = ln.split_once("@").unwrap();
        //let pos: Vec<_> = pos_str.trim().splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        //let vel: Vec<_> = vel_str.trim().splitn(3, ", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        let pos: Vec<_> = pos_str.splitn(3, ", ").map(|s| F128::from_str(s.trim()).unwrap()).collect();
        let vel: Vec<_> = vel_str.splitn(3, ", ").map(|s| F128::from_str(s.trim()).unwrap()).collect();
        return (
            Vector { x: pos[0], y: pos[1], z: pos[2] },
            Vector { x: vel[0], y: vel[1], z: vel[2] }
        )
    }).collect();

    // Solve a system of linear equations to determine rock_p_x, rock_p_y, rock_v_x, rock_v_y
    // Perform Gaussian elimination using fraction objects to avoid precision errors
    let f = Fraction::from(0).into_fraction::<i128>();
    let mut m = [[F128::from(0); 4]; 4];
    let mut b = [F128::from(0); 4];
    //let mut m = [[0.0; 4]; 4];
    //let mut b = [0.0; 4];
    let (hail_p0, hail_v0) = &hailstones[0];
    for i in 0..4 {
        let (hail_p, hail_v) = &hailstones[i + 1];
        m[i][0] = F128::from(hail_v0.y - hail_v.y);
        m[i][1] = F128::from(hail_v.x - hail_v0.x);
        m[i][2] = F128::from(hail_p.y - hail_p0.y);
        m[i][3] = F128::from(hail_p0.x - hail_p.x);
        b[i] = F128::from((hail_p0.x * hail_v0.y - hail_p0.y * hail_v0.x) - (hail_p.x * hail_v.y - hail_p.y * hail_v.x));
        /*
        m[i][0] = hail_v0.y - hail_v.y;
        m[i][1] = hail_v.x - hail_v0.x;
        m[i][2] = hail_p.y - hail_p0.y;
        m[i][3] = hail_p0.x - hail_p.x;
        b[i] = (hail_p0.x * hail_v0.y - hail_p0.y * hail_v0.x) - (hail_p.x * hail_v.y - hail_p.y * hail_v.x);
        */
    }
    println!("{:?}", m);
    println!("{:?}", b);
    for i in 0..4 { // Forward elimination
        // Multiply row by ratio which makes first coefficient equal to 1
        let ratio = m[i][i].recip();
        //let ratio = 1.0 / m[i][i];
        for j in i..4 {
            m[i][j] *= ratio;
        }
        b[i] *= ratio;

        // Set values below diagonal to 0
        for i_next in (i + 1)..4 {
            let mult_ratio = m[i_next][i];
            for j in i..4 {
                m[i_next][j] -= mult_ratio * m[i][j];
            }
            b[i_next] -= mult_ratio * b[i];
        }
    }
    println!("{:?}", m);
    println!("{:?}", b);
    for i in (1..4).rev() { // Back substitution
        for i_next in (0..i).rev() {
            b[i_next] -= m[i_next][i] * b[i];
            m[i_next][i] -= m[i_next][i] * m[i][i];
        }
    }

    // Compute rock_p_z and rock_v_z from other values
    let rock_p_x = b[0];
    let rock_p_y = b[1];
    let rock_v_x = b[2];
    let rock_v_y = b[3];
    
    let (p1, v1) = &hailstones[1];
    let (p2, v2) = &hailstones[2];
    let t1 = (rock_p_x - p1.x) / (v1.x - rock_v_x);
    let t2 = (rock_p_x - p2.x) / (v2.x - rock_v_x);
    let rock_v_z = (p1.z + v1.z * t1 - p2.z - v2.z * t2) / (t1 - t2);
    let rock_p_z = (p1.z + v1.z * t1) - rock_v_z * t1;

    println!("Pos: ({}, {}, {})", rock_p_x.to_i128().unwrap(), rock_p_y.to_i128().unwrap(), rock_p_z.to_i128().unwrap());
    println!("Vel: ({}, {}, {})", rock_v_x.to_i128().unwrap(), rock_v_y.to_i128().unwrap(), rock_v_z.to_i128().unwrap());
    //return (rock_p_x + rock_p_y + rock_p_z) as u64;
    return (rock_p_x + rock_p_y + rock_p_z).to_u64().unwrap();   
}
