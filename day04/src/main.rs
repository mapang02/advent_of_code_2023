use std::io;
use std::collections::HashSet;
use std::cmp;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut subtotal = 0;
    for line in lines {
        // Partition line into winning nums and card nums section
        let colon_index = line.find(":").unwrap();
        let sep_index = line.find("|").unwrap();

        // Get winning nums as HashSet, card nums as Vec
        let winning_nums_substr = &line[(colon_index + 1)..sep_index];
        let winning_nums: HashSet<u32> = winning_nums_substr.split_ascii_whitespace()
                                                            .map(|n| n.parse::<u32>().unwrap())
                                                            .collect();
        let card_nums_substr = &line[(sep_index + 1)..];
        let card_nums: Vec<u32> = card_nums_substr.split_ascii_whitespace()
                                                    .map(|n| n.parse::<u32>().unwrap())
                                                    .collect();
        
        // Calculate line score based on number of card nums which are winning nums
        let mut line_score = 0;
        for c in card_nums {
            if winning_nums.contains(&c) {
                if line_score == 0 {
                    line_score = 1;
                }
                else {
                    line_score *= 2;
                }
            }
        }
        subtotal += line_score;
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> i32 {
    // Calculate number of wins for each card
    let mut num_winning = Vec::new();
    for line in lines {
        // Get line and partition it into winning nums and card nums section
        let colon_index = line.find(":").unwrap();
        let sep_index = line.find("|").unwrap();

        // Get winning nums as HashSet, card nums as Vec
        let winning_nums_substr = &line[(colon_index + 1)..sep_index];
        let winning_nums: HashSet<u32> = winning_nums_substr.split_ascii_whitespace()
                                                            .map(|n| n.parse::<u32>().unwrap())
                                                            .collect();
        let card_nums_substr = &line[(sep_index + 1)..];
        let card_nums: Vec<u32> = card_nums_substr.split_ascii_whitespace()
                                                    .map(|n| n.parse::<u32>().unwrap())
                                                    .collect();
        
        // Calculate number of card numbers which are winning for the given line
        let mut line_score = 0;
        for c in card_nums {
            if winning_nums.contains(&c) {
                line_score += 1;
            }
        }
        num_winning.push(line_score);
    }
    //println!("{:?}", num_winning);

    // Calculate total number of copies
    let num_cards = num_winning.len();
    let mut num_copies = vec![1; num_cards];
    let mut subtotal = 0;
    for i in 0..num_cards {
        subtotal += num_copies[i];
        for j in (i + 1)..cmp::min(i + num_winning[i] + 1, num_cards) {
            num_copies[j] += num_copies[i];
        }
    }
    //println!("{:?}", num_copies);

    return subtotal
}