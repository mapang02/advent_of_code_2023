use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap_or_default().chars().collect()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> u32 {
    const NUM_RED: u32 = 12;
    const NUM_GREEN: u32 = 13;
    const NUM_BLUE: u32 = 14;
    
    let mut subtotal = 0;
    'outer_loop: for line in lines {
        if line.len() == 0 {
            break;
        }
        
        let colon_index = line.find(":").unwrap();
        let game_num = line[5..colon_index].parse::<u32>().unwrap();
        let pulls = line[colon_index+2..].split("; ");
        for p in pulls {
            let ball_count_entries = p.split(", ");
            for entry in ball_count_entries {
                let space_index = entry.find(" ").unwrap();
                let ball_num = entry[..space_index].parse::<u32>().unwrap();
                let ball_color = &entry[space_index+1..];
                if (ball_color == "red") && (ball_num > NUM_RED) {
                    continue 'outer_loop;
                }
                else if (ball_color == "green") && (ball_num > NUM_GREEN) {
                    continue 'outer_loop;
                }
                else if (ball_color == "blue") && (ball_num > NUM_BLUE) {
                    continue 'outer_loop;
                }
            }
        }
        subtotal += game_num;
    }
    return subtotal;
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut subtotal = 0;
    for line in lines {
        if line.len() == 0 {
            break;
        }
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;    
        
        let colon_index = line.find(":").unwrap();
        let pulls = line[colon_index+2..].split("; ");
        for p in pulls {
            let ball_count_entries = p.split(", ");
            for entry in ball_count_entries {
                let space_index = entry.find(" ").unwrap();
                let ball_num = entry[..space_index].parse::<u32>().unwrap();
                let ball_color = &entry[space_index+1..];
                if (ball_color == "red") && (ball_num > max_red) {
                    max_red = ball_num;
                }
                else if (ball_color == "green") && (ball_num > max_green) {
                    max_green = ball_num;
                }
                else if (ball_color == "blue") && (ball_num > max_blue) {
                    max_blue = ball_num;
                }
            }
        }

        subtotal += max_red * max_green * max_blue;
    }
    return subtotal;
}