use std::{io::{self, Read}, collections::HashMap};

fn main() {
    let mut input_str = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input_str) {
        println!("Error occurred when reading input");
        println!("{}", e.to_string());
    }
    let part1 = part1(&input_str);
    let part2 = part2(&input_str);
    println!("{}", part1);
    println!("{}", part2);
}

#[derive(Debug)]
struct RuleCondition {
    rating: char,
    cmp_type: char,
    cmp_value: i64
}

fn part1(input_str: &String) -> i64 {
    // Parse input
    let (rule_lines, part_lines) = input_str.split_once("\r\n\r\n").unwrap();

    let mut rules_map = HashMap::new();
    for rl in rule_lines.lines() {
        let (rule_name, rule_list_str) = rl.trim_end_matches('}').split_once('{').unwrap();
        let mut rule_list = Vec::new();
        for r_str in rule_list_str.split(",") {
            if let Some(colon_idx) = r_str.find(":") {
                let rule_condition_str = &r_str[0..colon_idx];
                let rating = rule_condition_str.chars().nth(0).unwrap();
                let cmp_type = rule_condition_str.chars().nth(1).unwrap();
                let cmp_value = i64::from_str_radix(&rule_condition_str[2..], 10).unwrap();

                let rule_condition = RuleCondition { rating: rating, cmp_type: cmp_type, cmp_value: cmp_value};
                let dest = &r_str[(colon_idx + 1)..];
                rule_list.push((Some(rule_condition), dest));
            }
            else {
                rule_list.push((None, r_str));
            }
        }
        rules_map.insert(rule_name, rule_list);
    }

    let mut parts = Vec::new();
    for pl in part_lines.lines() {
        let rating_str_list: Vec<&str> = pl.trim_matches(&['{', '}']).split(',').collect();
        let x_rating: i64 = rating_str_list[0][2..].parse().unwrap();
        let m_rating: i64 = rating_str_list[1][2..].parse().unwrap();
        let a_rating: i64 = rating_str_list[2][2..].parse().unwrap();
        let s_rating: i64 = rating_str_list[3][2..].parse().unwrap();
        parts.push((x_rating, m_rating, a_rating, s_rating));
    }

    let mut accepted_parts = Vec::new();
    for p in parts {
        let mut curr_rule = "in";
        while (curr_rule != "A") && (curr_rule != "R") {
            let rule_list = rules_map.get(curr_rule).unwrap();
            for (condition, dest) in rule_list {
                if let Some(RuleCondition { rating, cmp_type, cmp_value}) = condition {
                    let cmp_rating = match rating {
                        'x' => p.0,
                        'm' => p.1,
                        'a' => p.2,
                        's' => p.3,
                        _ => unreachable!()
                    };
                    let fulfills_condition = match cmp_type {
                        '>' => cmp_rating > *cmp_value,
                        '<' => cmp_rating < *cmp_value,
                        _ => unreachable!()
                    };
                    if fulfills_condition {
                        curr_rule = dest;
                        break;
                    }
                }
                else {
                    curr_rule = dest;
                    break;
                }
            }
        }
        if curr_rule == "A" {
            accepted_parts.push(p);
        }
    }
    
    let mut subtotal = 0;
    for p in accepted_parts {
        subtotal += p.0 + p.1 + p.2 + p.3;
    }
    return subtotal;
}

fn part2(input_str: &String) -> i64 {
    const X: usize = 0;
    const M: usize = 1;
    const A: usize = 2;
    const S: usize = 3;

    // Parse input
    let (rule_lines, part_lines) = input_str.split_once("\r\n\r\n").unwrap();

    let mut rules_map = HashMap::new();
    for rl in rule_lines.lines() {
        let (rule_name, rule_list_str) = rl.trim_end_matches('}').split_once('{').unwrap();
        let mut rule_list = Vec::new();
        for r_str in rule_list_str.split(",") {
            if let Some(colon_idx) = r_str.find(":") {
                let rule_condition_str = &r_str[0..colon_idx];
                let rating = rule_condition_str.chars().nth(0).unwrap();
                let cmp_type = rule_condition_str.chars().nth(1).unwrap();
                let cmp_value = i64::from_str_radix(&rule_condition_str[2..], 10).unwrap();

                let rule_condition = RuleCondition { rating: rating, cmp_type: cmp_type, cmp_value: cmp_value};
                let dest = &r_str[(colon_idx + 1)..];
                rule_list.push((Some(rule_condition), dest));
            }
            else {
                rule_list.push((None, r_str));
            }
        }
        rules_map.insert(rule_name, rule_list);
    }

    // Determine ranges of values that are accepted
    let mut accepted_ranges = Vec::new();
    let mut rejected_ranges = Vec::new();
    let mut range_stack: Vec<(Vec<(i64, i64)>, &str)> = vec![(vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)], "in"); 1];
    while let Some((mut p_range, curr_rule)) = range_stack.pop() {
        if curr_rule == "A" {
            accepted_ranges.push(p_range);
        }
        else if curr_rule == "R" {
            rejected_ranges.push(p_range);
        }
        else {
            let rule_list = rules_map.get(curr_rule).unwrap();
            for (condition, dest) in rule_list {
                let mut new_entry = p_range.clone();
                if let Some(RuleCondition { rating, cmp_type, cmp_value}) = condition {
                    let rating_idx = match rating {
                        'x' => X,
                        'm' => M,
                        'a' => A,
                        's' => S,
                        _ => unreachable!()
                    };
                    
                    let rating_range = p_range[rating_idx];
                    if *cmp_type == '>' {
                        if (rating_range.0 <= *cmp_value) && (rating_range.1 > *cmp_value) {
                            p_range[rating_idx].1 = *cmp_value;
                            new_entry[rating_idx].0 = cmp_value + 1;
                            range_stack.push((new_entry, dest));
                        }
                        else if *cmp_value < rating_range.0 {
                            range_stack.push((new_entry, dest));
                        }
                    }
                    else {
                        if (rating_range.0 < *cmp_value) && (rating_range.1 >= *cmp_value) {
                            p_range[rating_idx].0 = *cmp_value;
                            new_entry[rating_idx].1 = cmp_value - 1;
                            range_stack.push((new_entry, dest));
                        }
                        else if *cmp_value > rating_range.1 {
                            range_stack.push((new_entry, dest));
                        }
                    }
                }
                else {
                    range_stack.push((new_entry, dest));
                }
            }
        }
    }
    println!("{:?}", accepted_ranges);

    // Calculate number of valid combinations
    let mut subtotal = 0;
    for entry in accepted_ranges {
        subtotal += (entry[0].1 - entry[0].0 + 1) * (entry[1].1 - entry[1].0 + 1) * (entry[2].1 - entry[2].0 + 1) * (entry[3].1 - entry[3].0 + 1);
    }
    return subtotal;
}
