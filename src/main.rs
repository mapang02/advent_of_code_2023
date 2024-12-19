use std::io;
use std::cmp;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn part1(lines: &Vec<String>) -> i32 {
    // Split lines into grids
    let mut grids = Vec::new();
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for ln in lines {
        if ln.len() == 0 {
            grids.push(new_grid);
            new_grid = Vec::new();
        }
        else {
            let char_line: Vec<char> = ln.chars().collect();
            new_grid.push(char_line);
        }
    }
    if new_grid.len() > 0 {
        grids.push(new_grid);
    }

    // Find vertical and horizontal mirroring lines for each grid
    let mut subtotal = 0;
    for (n, grd) in grids.iter().enumerate() {
        let num_rows = grd.len();
        let num_cols = grd[0].len();

        // Find any lines of vertical symmetry
        for vert_mp_candidate in 1..num_cols {
            let num_mirrored = cmp::min(vert_mp_candidate, num_cols - vert_mp_candidate); // Prevent out of bounds
            let mut is_valid = true;
            // Check if each row is mirrored correctly
            'mirror_check: for row_index in 0..num_rows {
                for mirror_offset in 0..num_mirrored {
                    if grd[row_index][vert_mp_candidate - 1 - mirror_offset] != grd[row_index][vert_mp_candidate + mirror_offset] {
                        is_valid = false;
                        break 'mirror_check;
                    }
                }
            }
            if is_valid {
                //println!("Grid {}, Vertical midpoint: {}", n, vert_mp_candidate);
                subtotal += vert_mp_candidate;
            }
        }

        // Find line of horizontal symmetry
        for horiz_mp_candidate in 1..num_rows {
            let num_mirrored = cmp::min(horiz_mp_candidate, num_rows - horiz_mp_candidate); // Prevent out of bounds
            let mut is_valid = true;
            // Check if each column is mirrored correctly
            'mirror_check: for col_index in 0..num_cols {
                for mirror_offset in 0..num_mirrored {
                    if grd[horiz_mp_candidate - 1 - mirror_offset][col_index] != grd[horiz_mp_candidate + mirror_offset][col_index] {
                        is_valid = false;
                        break 'mirror_check;
                    }
                }
            }
            if is_valid {
                //println!("Grid {}, Horizontal midpoint: {}", n, horiz_mp_candidate);
                subtotal += 100 * horiz_mp_candidate;
            }
        }
    }

    return subtotal as i32;
}

fn part2(lines: &Vec<String>) -> i32 {
    // Split lines into grids
    let mut grids = Vec::new();
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for ln in lines {
        if ln.len() == 0 {
            grids.push(new_grid);
            new_grid = Vec::new();
        }
        else {
            let char_line: Vec<char> = ln.chars().collect();
            new_grid.push(char_line);
        }
    }
    if new_grid.len() > 0 {
        grids.push(new_grid);
    }

    // Check all smudge positions for each grid
    let mut subtotal = 0;
    for (n, grd) in grids.iter_mut().enumerate() {
        let num_rows = grd.len();
        let num_cols = grd[0].len();

        'smudge_pos_loop: for smudge_row in 0..num_rows {
            for smudge_col in 0..num_cols {
                // Flip value at smudge position
                grd[smudge_row][smudge_col] = match grd[smudge_row][smudge_col] {
                    '.' => '#',
                    '#' => '.',
                    _ => unreachable!()
                };

                // Find line of vertical symmetry (only checking lines where smudge position would be mirrored)
                for vert_mp_candidate in 1..num_cols {
                    let num_mirrored = cmp::min(vert_mp_candidate, num_cols - vert_mp_candidate); // Prevent out of bounds
                    // Check if smudge position would be mirrored
                    if !((smudge_col >= vert_mp_candidate - num_mirrored) && (smudge_col <= vert_mp_candidate + num_mirrored - 1)) {
                        continue;
                    }

                    // Check if each row is mirrored correctly
                    let mut is_valid = true;
                    'mirror_check: for row_index in 0..num_rows {
                        for mirror_offset in 0..num_mirrored {
                            if grd[row_index][vert_mp_candidate - 1 - mirror_offset] != grd[row_index][vert_mp_candidate + mirror_offset] {
                                is_valid = false;
                                break 'mirror_check;
                            }
                        }
                    }
                    if is_valid {
                        println!("Grid {}, Smudge: ({}, {}), Vertical midpoint: {}", n, smudge_row, smudge_col, vert_mp_candidate);
                        subtotal += vert_mp_candidate;
                        break 'smudge_pos_loop;
                    }
                }
        
                // Find line of horizontal symmetry (only checking lines where smudge position would be mirrored)
                for horiz_mp_candidate in 1..num_rows {
                    let num_mirrored = cmp::min(horiz_mp_candidate, num_rows - horiz_mp_candidate); // Prevent out of bounds
                    // Check if smudge position would be mirrored
                    if !((smudge_row >= horiz_mp_candidate - num_mirrored) && (smudge_row <= horiz_mp_candidate + num_mirrored - 1)) {
                        continue;
                    }

                    // Check if each column is mirrored correctly
                    let mut is_valid = true;
                    'mirror_check: for col_index in 0..num_cols {
                        for mirror_offset in 0..num_mirrored {
                            if grd[horiz_mp_candidate - 1 - mirror_offset][col_index] != grd[horiz_mp_candidate + mirror_offset][col_index] {
                                is_valid = false;
                                break 'mirror_check;
                            }
                        }
                    }
                    if is_valid {
                        println!("Grid {}, Smudge: ({}, {}), Horizontal midpoint: {}", n, smudge_row, smudge_col, horiz_mp_candidate);
                        subtotal += 100 * horiz_mp_candidate;
                        break 'smudge_pos_loop;
                    }
                }
                
                // Revert value at smudge position
                grd[smudge_row][smudge_col] = match grd[smudge_row][smudge_col] {
                    '.' => '#',
                    '#' => '.',
                    _ => unreachable!()
                };
            }
        }
    }

    return subtotal as i32;
}