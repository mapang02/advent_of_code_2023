use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

struct Pipe {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

#[derive(Copy, Clone, Debug)]
enum TileType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S
}

fn get_tile_connections(tile_type: TileType) -> Pipe {
    match tile_type {
        TileType::NS => Pipe { north: true, south: true, east: false, west: false },
        TileType::EW => Pipe { north: false, south: false, east: true, west: true },
        TileType::NE => Pipe { north: true, south: false, east: true, west: false },
        TileType::NW => Pipe { north: true, south: false, east: false, west: true },
        TileType::SW => Pipe { north: false, south: true, east: false, west: true },
        TileType::SE => Pipe { north: false, south: true, east: true, west: false },
        TileType::G => Pipe { north: false, south: false, east: false, west: false },
        TileType::S => Pipe { north: true, south: true, east: true, west: true }
    }
}

fn get_tile_type(c: char) -> TileType {
    match c {
        '|' => TileType::NS,
        '-' => TileType::EW,
        'L' => TileType::NE,
        'J' => TileType::NW,
        '7' => TileType::SW,
        'F' => TileType::SE,
        'S' => TileType::S,
        _ => TileType::G
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    // Load tiles, find start point
    let mut start_pos = (0, 0);
    let mut grid = vec![vec!['.'; lines[0].len() + 2]; lines.len() + 2];
    for i in 0..lines.len() {
        for (j, ch) in lines[i].chars().enumerate() {
            grid[i + 1][j + 1] = ch;
            if ch == 'S' {
                start_pos = (i + 1, j + 1);
            }
        }
    }

    // Navigate path
    let mut visited = vec![vec![false; lines[0].len() + 2]; lines.len() + 2];
    let mut num_visited = 1;
    let (mut curr_row, mut curr_col) = start_pos;
    let mut curr_tile = TileType::S;

    visited[start_pos.0][start_pos.1] = true;
    loop {
        let north_tile_type = get_tile_type(grid[curr_row - 1][curr_col]);
        let south_tile_type = get_tile_type(grid[curr_row + 1][curr_col]);
        let east_tile_type = get_tile_type(grid[curr_row][curr_col + 1]);
        let west_tile_type = get_tile_type(grid[curr_row][curr_col - 1]);
        // Check if north path is available
        if get_tile_connections(curr_tile).north && 
            get_tile_connections(north_tile_type).south &&
            !visited[curr_row - 1][curr_col] {
            curr_row = curr_row - 1;
            curr_tile = north_tile_type;
            
            num_visited += 1;
            visited[curr_row][curr_col] = true;
            continue;
        }
        // Check if south path is available
        else if get_tile_connections(curr_tile).south && 
            get_tile_connections(south_tile_type).north &&
            !visited[curr_row + 1][curr_col] {
            curr_row = curr_row + 1;
            curr_tile = south_tile_type;

            num_visited += 1;
            visited[curr_row][curr_col] = true;
            continue;
        }
        // Check if east path is available
        else if get_tile_connections(curr_tile).east && 
            get_tile_connections(east_tile_type).west &&
            !visited[curr_row][curr_col + 1] {
            curr_col = curr_col + 1;
            curr_tile = east_tile_type;

            num_visited += 1;
            visited[curr_row][curr_col] = true;
            continue;
        }
        // Check if east path is available
        else if get_tile_connections(curr_tile).west && 
            get_tile_connections(west_tile_type).east &&
            !visited[curr_row][curr_col - 1] {
            curr_col = curr_col - 1;
            curr_tile = west_tile_type;

            num_visited += 1;
            visited[curr_row][curr_col] = true;
            continue;
        }

        // If no unvisited connections can be found, the end has been reached
        break;
    }

    return (num_visited / 2) as i32;
}

fn part2(lines: &Vec<String>) -> i32 {
    // Load tiles, find start point
    let mut start_pos = (0, 0);
    let mut grid = vec![vec!['.'; lines[0].len() + 2]; lines.len() + 2];
    for i in 0..lines.len() {
        for (j, ch) in lines[i].chars().enumerate() {
            grid[i + 1][j + 1] = ch;
            if ch == 'S' {
                start_pos = (i + 1, j + 1);
            }
        }
    }

    // Navigate boundary
    let mut is_boundary = vec![vec![false; lines[0].len() + 2]; lines.len() + 2];
    let mut num_boundary_tiles = 1;
    let (mut curr_row, mut curr_col) = start_pos;
    let mut curr_tile = TileType::S;

    is_boundary[start_pos.0][start_pos.1] = true;
    loop {
        let north_tile_type = get_tile_type(grid[curr_row - 1][curr_col]);
        let south_tile_type = get_tile_type(grid[curr_row + 1][curr_col]);
        let east_tile_type = get_tile_type(grid[curr_row][curr_col + 1]);
        let west_tile_type = get_tile_type(grid[curr_row][curr_col - 1]);
        // Check if north path is available
        if get_tile_connections(curr_tile).north && 
            get_tile_connections(north_tile_type).south &&
            !is_boundary[curr_row - 1][curr_col] {
            curr_row = curr_row - 1;
            curr_tile = north_tile_type;
            
            num_boundary_tiles += 1;
            is_boundary[curr_row][curr_col] = true;
            continue;
        }
        // Check if south path is available
        else if get_tile_connections(curr_tile).south && 
            get_tile_connections(south_tile_type).north &&
            !is_boundary[curr_row + 1][curr_col] {
            curr_row = curr_row + 1;
            curr_tile = south_tile_type;

            num_boundary_tiles += 1;
            is_boundary[curr_row][curr_col] = true;
            continue;
        }
        // Check if east path is available
        else if get_tile_connections(curr_tile).east && 
            get_tile_connections(east_tile_type).west &&
            !is_boundary[curr_row][curr_col + 1] {
            curr_col = curr_col + 1;
            curr_tile = east_tile_type;

            num_boundary_tiles += 1;
            is_boundary[curr_row][curr_col] = true;
            continue;
        }
        // Check if east path is available
        else if get_tile_connections(curr_tile).west && 
            get_tile_connections(west_tile_type).east &&
            !is_boundary[curr_row][curr_col - 1] {
            curr_col = curr_col - 1;
            curr_tile = west_tile_type;

            num_boundary_tiles += 1;
            is_boundary[curr_row][curr_col] = true;
            continue;
        }

        // If no unvisited connections can be found, the end has been reached
        break;
    }

    // Replace 'S' with matching boundary shape
    if is_boundary[start_pos.0 - 1][start_pos.1] {
        if is_boundary[start_pos.0][start_pos.1 - 1] {
            grid[start_pos.0][start_pos.1] = 'J';
        }
        else if is_boundary[start_pos.0][start_pos.1 + 1]{
            grid[start_pos.0][start_pos.1] = 'L';
        }
        else {
            grid[start_pos.0][start_pos.1] = '|';
        }
    }
    else {
        if !is_boundary[start_pos.0 + 1][start_pos.1] {
            grid[start_pos.0][start_pos.1] = '-';
        }
        else if is_boundary[start_pos.0][start_pos.1 - 1] {
            grid[start_pos.0][start_pos.1] = '7';
        }
        else {
            grid[start_pos.0][start_pos.1] = 'F';
        }
    }
    for row_num in 0..grid.len() {
        for col_num in 0..grid[0].len() {
            if is_boundary[row_num][col_num] {
                print!("{}", grid[row_num][col_num]);
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }

    // Check if each point is interior by counting boundary crossings
    let mut num_interior_tiles = 0;
    for row_num in 0..grid.len() {
        let mut boundary_crossings = 0;
        let mut crossing_start_tile = TileType::G;
        for col_num in 0..grid[0].len() {
            if is_boundary[row_num][col_num] {
                let prev_boundary_crossings = boundary_crossings;
                let curr_tile = get_tile_type(grid[row_num][col_num]);
                match curr_tile {
                    TileType::NS => boundary_crossings += 1,
                    TileType::NE | TileType::SE => crossing_start_tile = curr_tile,
                    TileType::NW => {
                        if let TileType::SE = crossing_start_tile {
                            boundary_crossings += 1;
                        }
                        crossing_start_tile = TileType::G;
                    },
                    TileType::SW => {
                        if let TileType::NE = crossing_start_tile {
                            boundary_crossings += 1;
                        }
                        crossing_start_tile = TileType::G;
                    },
                    TileType::EW => {},
                    _ => unreachable!()
                }
            }
            else if boundary_crossings % 2 == 1 {
                num_interior_tiles += 1;
            }
        }
    }
    
    return num_interior_tiles;
}