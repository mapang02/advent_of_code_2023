use std::io;
use regex::Regex;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut subtotal = 0;
    for line in lines {
        let mut first_digit = 0;
        let mut last_digit = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last_digit = c.to_digit(10).unwrap();
                break;
            }
        }
        
        subtotal += 10 * first_digit + last_digit;
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut subtotal = 0;
    for line in lines {
        if line.len() == 0 {
            break;
        }
        let reversed_line = line.chars().rev().collect::<String>();

        let first_re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|ten|0|1|2|3|4|5|6|7|8|9").unwrap();
        let first_number = first_re.find(&line).unwrap().as_str();
        let last_re = Regex::new(r"orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|net|0|1|2|3|4|5|6|7|8|9").unwrap();
        let last_number = last_re.find(&reversed_line).unwrap().as_str();
        
        let first_digit = match first_number {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => 0
        };
        let last_digit = match last_number {
            "0" | "orez" => 0,
            "1" | "eno" => 1,
            "2" | "owt" => 2,
            "3" | "eerht" => 3,
            "4" | "ruof" => 4,
            "5" | "evif" => 5,
            "6" | "xis" => 6,
            "7" | "neves" => 7,
            "8" | "thgie" => 8,
            "9" | "enin" => 9,
            _ => 0
        };
        //println!("{first_digit}{last_digit}");
        subtotal += 10 * first_digit + last_digit;
    }
    return subtotal;
}