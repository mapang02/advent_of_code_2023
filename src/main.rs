use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    // Find empty rows and columns
    let mut empty_rows = Vec::new();
    for i in 0..lines.len() {
        if lines[i].chars().all(|c| c == '.') {
            empty_rows.push(i);
        }
    }
    let mut empty_cols = Vec::new();
    for i in 0..lines[0].len() {
        if (0..lines.len()).map(|row_num| &lines[row_num][i..i+1]).all(|c| c == ".") {
            empty_cols.push(i);
        }
    }
    empty_rows.push(std::usize::MAX); // Add padding value to simplify coord calculation
    empty_cols.push(std::usize::MAX);
    //println!("{:?}", empty_rows);
    //println!("{:?}", empty_cols);

    
    // Calculate coordinates of each galaxy
    let mut galaxy_coords = Vec::new();
    let mut empty_row_offset = 0;
    for i in 0..lines.len() {
        if empty_rows[empty_row_offset] == i {
            empty_row_offset += 1;
            continue;
        }
        let mut empty_col_offset = 0;
        for j in 0..lines[0].len() {
            if empty_cols[empty_col_offset] == j {
                empty_col_offset += 1;
            }
            if &lines[i][j..j+1] == "#" {
                galaxy_coords.push((i + empty_row_offset, j + empty_col_offset))
            }
        }
    }
    //println!("{:?}", galaxy_coords);

    // Calculate all pair distances
    let mut subtotal = 0;
    for i in 0..galaxy_coords.len() {
        for j in (i + 1)..galaxy_coords.len() {
            let dist_row = (galaxy_coords[i].0).abs_diff(galaxy_coords[j].0);
            let dist_col = (galaxy_coords[i].1).abs_diff(galaxy_coords[j].1);
            subtotal += dist_row + dist_col;
            //println!("Dist from {} to {}: {}", i, j, dist_row + dist_col);
        }
    }
    return subtotal as i32;
}

fn part2(lines: &Vec<String>) -> i64 {
    // Find empty rows and columns
    let mut empty_rows = Vec::new();
    for i in 0..lines.len() {
        if lines[i].chars().all(|c| c == '.') {
            empty_rows.push(i);
        }
    }
    let mut empty_cols = Vec::new();
    for i in 0..lines[0].len() {
        if (0..lines.len()).map(|row_num| &lines[row_num][i..i+1]).all(|c| c == ".") {
            empty_cols.push(i);
        }
    }
    empty_rows.push(std::usize::MAX); // Add padding value to simplify coord calculation
    empty_cols.push(std::usize::MAX);
    //println!("{:?}", empty_rows);
    //println!("{:?}", empty_cols);

    
    // Calculate coordinates of each galaxy
    let mut galaxy_coords = Vec::new();
    let mut empty_row_offset = 0;
    for i in 0..lines.len() {
        if empty_rows[empty_row_offset] == i {
            empty_row_offset += 1;
            continue;
        }
        let mut empty_col_offset = 0;
        for j in 0..lines[0].len() {
            if empty_cols[empty_col_offset] == j {
                empty_col_offset += 1;
            }
            if &lines[i][j..j+1] == "#" {
                galaxy_coords.push((i + empty_row_offset * 999999, j + empty_col_offset * 999999))
            }
        }
    }
    //println!("{:?}", galaxy_coords);

    // Calculate all pair distances
    let mut subtotal = 0;
    for i in 0..galaxy_coords.len() {
        for j in (i + 1)..galaxy_coords.len() {
            let dist_row = (galaxy_coords[i].0).abs_diff(galaxy_coords[j].0);
            let dist_col = (galaxy_coords[i].1).abs_diff(galaxy_coords[j].1);
            subtotal += dist_row + dist_col;
            //println!("Dist from {} to {}: {}", i, j, dist_row + dist_col);
        }
    }
    return subtotal as i64;
}