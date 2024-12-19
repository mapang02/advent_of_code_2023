use std::io;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    // Hand is stored as a 6-digit number in base-13
    fn hand_order(hand: &str) -> i32 {
        const CARD_ORDER: &str = "23456789TJQKA";

        // Get rank of hand type
        let mut card_count = HashMap::new();
        for c in hand.chars() {
            *card_count.entry(c).or_insert(0) += 1;
        }
        let mut card_dist: Vec<&i32> = card_count.values().collect();
        card_dist.sort_unstable();
        let hand_rank = match card_dist[..] {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            _ => 0
        };

        // Get rank of each card
        let mut subtotal = hand_rank;
        for c in hand.chars() {
            let card_rank = CARD_ORDER.find(c).unwrap_or_default();
            subtotal *= CARD_ORDER.len();
            subtotal += card_rank;
        }

        return subtotal.try_into().unwrap();
    }

    // Parse lines
    let mut hand_bid_pairs = Vec::new();
    for l in lines {
        let hand = &l[..l.find(" ").unwrap()];
        let bid: i32 = l[(l.find(" ").unwrap() + 1)..].parse().unwrap();
        hand_bid_pairs.push((hand, bid));
    }

    // Sort hands by rank and compute output
    hand_bid_pairs.sort_by_key(|tup| hand_order(tup.0));
    let mut subtotal = 0;
    for (i, (_, bid)) in hand_bid_pairs.iter().enumerate() {
        subtotal += ((i as i32) + 1) * bid;
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> i32 {
    // Hand is stored as a 6-digit number in base-13
    fn hand_order(hand: &str) -> i32 {
        const CARD_ORDER: &str = "J23456789TQKA";

        // Get rank of hand type (substituting each possible value of J)
        // Using the same value for each J is always optimal
        let mut max_hand_rank = 0;
        for joker_sub in CARD_ORDER[1..].chars() {
            let mut card_count = HashMap::new();
            for mut c in hand.chars() {
                if c == 'J' {
                    c = joker_sub;
                }
                *card_count.entry(c).or_insert(0) += 1;
            }
            let mut card_dist: Vec<&i32> = card_count.values().collect();
            card_dist.sort_unstable();
            let hand_rank = match card_dist[..] {
                [5] => 6,
                [1, 4] => 5,
                [2, 3] => 4,
                [1, 1, 3] => 3,
                [1, 2, 2] => 2,
                [1, 1, 1, 2] => 1,
                _ => 0
            };

            if hand_rank > max_hand_rank {
                max_hand_rank = hand_rank;
            }
        }

        // Get rank of each card
        let mut subtotal = max_hand_rank;
        for c in hand.chars() {
            let card_rank = CARD_ORDER.find(c).unwrap_or_default();
            subtotal *= CARD_ORDER.len();
            subtotal += card_rank;
        }

        return subtotal.try_into().unwrap();
    }

    // Parse lines
    let mut hand_bid_pairs = Vec::new();
    for l in lines {
        let hand = &l[..l.find(" ").unwrap()];
        let bid: i32 = l[(l.find(" ").unwrap() + 1)..].parse().unwrap();
        hand_bid_pairs.push((hand, bid));
    }

    // Sort hands by rank and compute output
    hand_bid_pairs.sort_by_key(|tup| hand_order(tup.0));
    let mut subtotal = 0;
    for (i, (_, bid)) in hand_bid_pairs.iter().enumerate() {
        subtotal += ((i as i32) + 1) * bid;
    }
    return subtotal;
}