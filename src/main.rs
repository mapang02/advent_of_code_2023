use std::env;
use std::fs;

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
    let (times_line, distances_line) = file_str.split_once("\n").unwrap();
    let times: Vec<i64> = times_line.split_ascii_whitespace().map(|n| n.parse().unwrap_or_default()).collect();
    let distances: Vec<i64> = distances_line.split_ascii_whitespace().map(|n| n.parse().unwrap_or_default()).collect();
    dbg!(&times);
    dbg!(&distances);

    let mut subtotal = 1;
    for i in 1..times.len() {
        for j in 0..(times[i] / 2 + 1) {
            if j * (times[i] - j) > distances[i] {
                let num_ways = (times[i] + 1) - 2 * j;
                subtotal *= num_ways;
                println!("Time {}, Distance {} -> {}", times[i], distances[i], num_ways);
                break;
            }
        }
    }

    return subtotal as i64;
}

fn part2(file_str: &String) -> i64 {
    let (times_line, distances_line) = file_str.split_once("\n").unwrap();
    let time: i64 = times_line.split_once(":").unwrap().1.replace(" ", "").parse().unwrap_or_default();
    let distance: i64 = distances_line.split_once(":").unwrap().1.replace(" ", "").parse().unwrap_or_default();
    dbg!(&time);
    dbg!(&distance);

    for i in 1..(time / 2 + 1) {
        if i * (time - i) > distance {
            let num_ways = (time + 1) - 2 * i;
            println!("Time {}, Distance {} -> {}", time, distance, num_ways);
            return num_ways;
        }
    }
    return 0;
}