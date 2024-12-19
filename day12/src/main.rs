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
    for ln in lines {
        // Parse input line
        let (condition_str, groups_str) = ln.split_once(" ").unwrap();
        let contiguous_groups: Vec<i32> = groups_str.split(",").map(|n| n.parse().unwrap()).collect();

        // Alternate between working/damaged for each unknown spring, check if it matches group sizes
        let mut valid_arrangements = 0;
        let num_unknown = condition_str.chars().filter(|ch| *ch == '?').count();
        for mut bitmap in 0..(1 << num_unknown) {
            // Count group sizes for given arrangement
            let mut arrangement_groups = Vec::new();
            let mut consec_damaged = 0;
            for ch in condition_str.chars() {
                let is_damaged = match ch {
                    '.' => false,
                    '#' => true,
                    '?' => {
                        let output = (bitmap % 2) == 0;
                        bitmap /= 2;
                        output
                    },
                    _ => unreachable!()
                };
                if is_damaged {
                    consec_damaged += 1;
                }
                else if consec_damaged > 0 {
                    arrangement_groups.push(consec_damaged);
                    consec_damaged = 0;
                }
            }
            if consec_damaged > 0 {
                arrangement_groups.push(consec_damaged);
            }
            //println!("{:?}", arrangement_groups);

            // Check if group sizes match
            if arrangement_groups == contiguous_groups {
                valid_arrangements += 1;
            }
        }
        //println!("{} valid arrangements", valid_arrangements);
        subtotal += valid_arrangements;
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut subtotal = 0;
    for ln in lines {
        // Parse input line
        let (input_condition_str, groups_str) = ln.split_once(" ").unwrap();

        let mut condition_data = Vec::new();
        for _ in 0..5 {
            for ch in input_condition_str.chars() {
                condition_data.push(ch);
            }
            condition_data.push('?');
        }
        condition_data.pop();
        
        let input_contiguous_groups: Vec<usize> = groups_str.split(",").map(|n| n.parse().unwrap()).collect();
        let mut condition_groups = Vec::new();
        for _ in 0..5 {
            for n in &input_contiguous_groups {
                condition_groups.push(*n);
            }
        }

        // Recursively count arrangements
        fn count_arrangements(results: &mut Vec<Vec<Option<i64>>>, 
            condition_data: &Vec<char>, 
            condition_groups: &Vec<usize>,
            start_index: usize, 
            num_groups_used: usize) -> i64 {

            // Check if memoized result exists
            if let Some(count) = results[start_index][num_groups_used] {
                //println!("Returned memoized value results[{}][{}] = {}", start_index, num_groups_used, count);
                return count;
            }

            // Check base cases
            if start_index == condition_data.len() {
                if num_groups_used == condition_groups.len() {
                    results[start_index][num_groups_used] = Some(1);
                    return 1;
                }
                else {
                    results[start_index][num_groups_used] = Some(0);
                    return 0;
                }
            }
            else if num_groups_used == condition_groups.len() {
                // Make sure all following values are not damaged if no groups remain
                if condition_data[start_index..].iter().all(|ch| (*ch == '.') | (*ch == '?')) {
                    results[start_index][num_groups_used] = Some(1);
                    return 1;
                }
                else {
                    results[start_index][num_groups_used] = Some(0);
                    return 0;
                }
            }

            // Check if next group can be placed starting at start_index (following k elements are '#' or '?')
            let mut arrangement_count = 0;
            let group_end_index = start_index + condition_groups[num_groups_used];
            if group_end_index > condition_data.len() {
                results[start_index][num_groups_used] = Some(0);
                return 0;
            }
            // Total arrangements if next group is chosen to start at this position
            if condition_data[start_index..group_end_index].iter().all(|ch| (*ch == '#') | (*ch == '?')) {
                // Check if there are k instances of '#' or '?' followed by '.' or '?' or end of list to terminate group
                if group_end_index == condition_data.len() {
                    arrangement_count += count_arrangements(results, condition_data, condition_groups, group_end_index, num_groups_used + 1);
                }
                else if condition_data[group_end_index] != '#' {
                    // Increase start index by 1 to cover terminating symbol
                    arrangement_count += count_arrangements(results, condition_data, condition_groups, group_end_index + 1, num_groups_used + 1);
                }
            }
            // Total arrangements if next group is chosen to not start at this position
            if condition_data[start_index] != '#' {
                arrangement_count += count_arrangements(results, condition_data, condition_groups, start_index + 1, num_groups_used);
            }

            // Memoize result and then return
            results[start_index][num_groups_used] = Some(arrangement_count);
            //print_results(&results);
            return arrangement_count;
        }
        let mut results: Vec<Vec<Option<i64>>> = vec![vec![None; condition_groups.len() + 1]; condition_data.len() + 1];
        let arrangement_count = count_arrangements(&mut results, &condition_data, &condition_groups, 0, 0);
        println!("{}", arrangement_count);
        subtotal += arrangement_count;
    }

    return subtotal;
}

fn print_results(results: &Vec<Vec<Option<i64>>>) {
    for i in 0..results.len() {
        for j in 0..results[0].len() {
            match results[i][j] {
                Some(n) => print!("{:5}", n),
                None => print!("    -")
            }
        }
        println!("");
    }
    println!("");
}