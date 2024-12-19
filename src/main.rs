use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input_str = String::new();
    let _ = io::stdin().read_to_string(&mut input_str);
    let part1 = part1(&input_str);
    let part2 = part2(&input_str);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(input_str: &String) -> i32 {
    let hash_inputs: Vec<&str> = input_str.split(",").collect();
    let mut subtotal = 0;
    for input in hash_inputs {
        let mut hash_value = 0;
        for ch in input.as_bytes() {
            hash_value += *ch as i32;
            hash_value *= 17;
            hash_value %= 256;
        }
        subtotal += hash_value;
    }
    return subtotal;
}

fn part2(input_str: &String) -> i32 {
    //let boxes = HashMap::new();
    let mut boxes: [Vec<(&str, i32)>; 256] = std::array::from_fn(|_| Vec::new());
    let cmd_inputs: Vec<&str> = input_str.split(",").collect();
    for cmd in cmd_inputs {
        // Process lens removal command
        if cmd.ends_with('-') {
            let lens_label = &cmd[0..(cmd.len() - 1)];
            let box_num = hash(lens_label);
            let lens_list = &mut boxes[box_num];
            let lens_index = (0..lens_list.len()).find(|idx| lens_list[*idx].0 == lens_label);
            if let Some(idx) = lens_index {
                lens_list.remove(idx);
            }
        }
        // Process lens insertion command
        else {
            let lens_label = &cmd[0..cmd.find('=').unwrap()];
            let focal_length: i32 = cmd[(cmd.find('=').unwrap() + 1)..].parse().unwrap();
            let box_num = hash(lens_label);
            let lens_list = &mut boxes[box_num];
            match (0..lens_list.len()).find(|idx| lens_list[*idx].0 == lens_label) {
                Some(idx) => lens_list[idx] = (lens_label, focal_length),
                None => lens_list.push((lens_label, focal_length))
            }
        }
    }
    // Debug print
    for box_num in 0..boxes.len() {
        if boxes[box_num].len() > 0 {
            print!("Box {}: ", box_num);
            for (lbl, focal_length) in &boxes[box_num] {
                print!("[{} {}]", lbl, focal_length);
            }
            println!("");
        }

    }
    // Calculate final score
    let mut score = 0;
    for box_num in 0..boxes.len() {
        for lens_idx in 0..boxes[box_num].len() {
            score += (box_num + 1) * (lens_idx + 1) * (boxes[box_num][lens_idx].1 as usize);
        }
    }
    return score as i32;
}

fn hash(input_str: &str) -> usize {
    let mut hash_value = 0;
    for ch in input_str.as_bytes() {
        hash_value += *ch as usize;
        hash_value *= 17;
        hash_value %= 256;
    }
    return hash_value;
}
