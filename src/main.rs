use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut subtotal = 0;
    for l in lines {
        // Calculate nth order differences
        let nums: Vec<i32> = l.split(" ").map(|n| n.parse().unwrap()).collect();
        let mut nth_order_diffs = Vec::new();
        nth_order_diffs.push(nums);
        while !nth_order_diffs.last().unwrap().iter().all(|n| *n == 0) {
            let prev_diff_vec = nth_order_diffs.last().unwrap();
            let mut diffs = Vec::new();
            for i in 1..prev_diff_vec.len() {
                diffs.push(prev_diff_vec[i] - prev_diff_vec[i - 1]);
            }
            nth_order_diffs.push(diffs);
        }

        // Calculate next value in sequence
        nth_order_diffs.last_mut().unwrap().push(0);
        for i in (1..nth_order_diffs.len()).rev() {
            let next_diff_val = nth_order_diffs[i - 1].last().unwrap() + nth_order_diffs[i].last().unwrap();
            nth_order_diffs[i - 1].push(next_diff_val);
        }
        subtotal += nth_order_diffs[0].last().unwrap();
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut subtotal = 0;
    for l in lines {
        // Calculate nth order differences
        let nums: Vec<i32> = l.split(" ").map(|n| n.parse().unwrap()).collect();
        let mut nth_order_diffs = Vec::new();
        nth_order_diffs.push(nums);
        while !nth_order_diffs.last().unwrap().iter().all(|n| *n == 0) {
            let prev_diff_vec = nth_order_diffs.last().unwrap();
            let mut diffs = Vec::new();
            for i in 1..prev_diff_vec.len() {
                diffs.push(prev_diff_vec[i] - prev_diff_vec[i - 1]);
            }
            nth_order_diffs.push(diffs);
        }

        // Calculate next value in sequence
        nth_order_diffs.last_mut().unwrap().insert(0, 0);
        for i in (1..nth_order_diffs.len()).rev() {
            let prev_diff_val = nth_order_diffs[i - 1][0] - nth_order_diffs[i][0];
            nth_order_diffs[i - 1].insert(0, prev_diff_val);
        }
        subtotal += nth_order_diffs[0][0];
        //dbg!(&nth_order_diffs);
    }
    return subtotal;
}