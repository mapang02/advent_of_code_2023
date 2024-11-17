use core::num;
use std::{array, collections::{HashMap, HashSet}, hash::Hash, io};

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock
}

fn print_map(map: &Vec<Vec<Tile>>, reachable: &Vec<Vec<bool>>) {
    let width = map[0].len();
    let height = map.len();
    for j in 0..height {
        for i in 0..width {
            if reachable[i][j] {
                print!("O");
            }
            else if map[i][j] == Tile::Garden {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!("");
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let width = lines[0].len();
    let height = lines.len();

    // Validate input, convert to map
    let tile_chars = lines.concat();
    if tile_chars.len() != width * height {
        println!("Invalid input, width = {}, height = {}, len = {}", width, height, width * height);
        return 0;
    }
    let start_index = tile_chars.find('S').unwrap_or(0);
    let start_x = start_index % height + 1; // Offset taking into account sentinel
    let start_y = start_index / height + 1; // Offset taking into account sentinel
    
    let mut map = vec![vec![Tile::Rock; height + 2]; width + 2]; // Extra rows and columns added for sentinel values
    for (i, ch) in tile_chars.char_indices() {
        let tile_x = i % height;
        let tile_y = i / height;
        map[tile_x + 1][tile_y + 1] = match ch {
            '#' => Tile::Rock,
            _ => Tile::Garden
        }
    }

    // Iterate from reachable positions
    let num_steps = 64;
    let mut reachable = vec![vec![false; height + 2]; width + 2];
    reachable[start_x][start_y] = true;
    for _ in 0..num_steps {
        let mut new_reachable = vec![vec![false; height + 2]; width + 2];
        for j in 1..(height + 1) {
            for i in 1..(width + 1) {
                if map[i][j] == Tile::Garden && (reachable[i - 1][j] || reachable[i + 1][j] || reachable[i][j - 1] || reachable[i][j + 1]) {
                    new_reachable[i][j] = true;
                }
            }
        }
        reachable = new_reachable;
    }

    // Count reachable positions
    //print_map(&map, &reachable);
    return reachable.into_iter().flatten().filter(|r| *r).count() as i32;
}

#[derive(PartialEq, Eq, Hash)]
struct AdjChunkStates {
    curr: u32,
    north: u32,
    south: u32,
    east: u32,
    west: u32
}

struct ReachMapLookup {
    width: usize,
    height: usize,
    reach_map_ids: HashMap<Vec<bool>, u32>,
    reach_map_lookup: HashMap<u32, Vec<bool>>,
    num_reach_maps: u32
}
impl ReachMapLookup {
    fn new(width: usize, height: usize) -> ReachMapLookup {
        return ReachMapLookup { width: width, height: height, reach_map_ids: HashMap::new(), reach_map_lookup: HashMap::new(), num_reach_maps: 0 }
    }
    fn get_id(&self, reach_map: &Vec<bool>) -> Option<u32> {
        return self.reach_map_ids.get(reach_map).copied();
    }
    fn get_reach_map(&self, id: u32) -> Option<&Vec<bool>> {
        return self.reach_map_lookup.get(&id);
    }
    fn insert(&mut self, reach_map: &Vec<bool>) -> Option<u32> {
        if !self.reach_map_ids.contains_key(reach_map) {
            self.reach_map_ids.insert(reach_map.clone(), self.num_reach_maps);
            self.reach_map_lookup.insert(self.num_reach_maps, reach_map.clone());
            self.num_reach_maps += 1;
            return Some(self.num_reach_maps - 1);
        }
        return None;
    }
}

fn part2(lines: &Vec<String>) -> u64 {
    let width = lines[0].len();
    let height = lines.len();

    // Validate input, convert to map
    let tile_chars = lines.concat();
    if tile_chars.len() != width * height {
        println!("Invalid input, width = {}, height = {}, len = {}", width, height, width * height);
        return 0;
    }
    let start_index = tile_chars.find('S').unwrap_or(0);
    
    let mut tile_map = vec![vec![Tile::Rock; height]; width];
    for (i, ch) in tile_chars.char_indices() {
        let tile_x = i % height;
        let tile_y = i / height;
        tile_map[tile_x][tile_y] = match ch {
            '#' => Tile::Rock,
            _ => Tile::Garden
        }
    }

    // Each reach map is represented by a (width * height)-length vector of booleans
    // Each reach map that is created during the process is given a unique ID, and a hash table can be used to look it up
    // The next state for each step on a given reach map is computed once, and can subsequently be looked up
    // When entering a newly generated "chunk", it can be generated based on its neighbors
    // Chunks don't need to be tracked after all alternating tiles are filled
    // A chunk is "generated" when it is first added to the chunks map, and is "live" after being generated until reaching a filled state
    let mut reach_map_lookup = ReachMapLookup::new(width, height);
    let mut reach_map_transitions: HashMap<AdjChunkStates, u32> = HashMap::new();
    let mut chunks: HashMap<(i32, i32), u32> = HashMap::new();
    let mut live_chunks: HashSet<(i32, i32)> = HashSet::new();
    let mut num_filled_chunks = 0;

    // Pre-generate maximally filled chunks
    let empty_chunk = vec![false; width * height];
    let max_fill_even = (0..(width * height)).map(|i| i % 2 == 0 && tile_map[i % height][i / height] == Tile::Garden).collect();
    let max_fill_odd = (0..(width * height)).map(|i| i % 2 == 1 && tile_map[i % height][i / height] == Tile::Garden).collect();

    let empty_chunk_id = reach_map_lookup.insert(&empty_chunk).unwrap();
    let max_fill_even_id = reach_map_lookup.insert(&max_fill_even).unwrap();
    let max_fill_odd_id = reach_map_lookup.insert(&max_fill_odd).unwrap();
    
    // Generate starting chunk
    let mut start_reach = vec![false; width * height];
    start_reach[start_index] = true;
    let start_reach_id = reach_map_lookup.insert(&start_reach).unwrap();
    chunks.insert((0, 0), start_reach_id);
    live_chunks.insert((0, 0));
    for adj_chunk_coords in [(-1, 0), (1, 0), (0, -1), (0, 1)] { // Make chunks adjacent to start chunk live
        chunks.insert(adj_chunk_coords, 0);
        live_chunks.insert(adj_chunk_coords);
    }

    // Iterate through steps
    let total_step_count = 5;
    for step_num in 0..total_step_count {
        //println!("{:?}", chunks);
        let filled_tile_state = ((step_num + start_index) % 2 + 1).try_into().unwrap();

        // Determine which chunks to change
        let mut changed_chunks: HashMap<(i32, i32), u32> = HashMap::new();
        let mut new_live_chunks: HashSet<(i32, i32)> = HashSet::new();
        let mut new_filled_chunks: HashSet<(i32, i32)> = HashSet::new();
        for (chunk_x, chunk_y) in live_chunks.iter().copied() {
            // Attempt to look up state change based on neighboring chunks, compute it otherwise
            // TODO: Make sure to check state for filled chunks correctly
            let curr_chunk_state = match chunks.get(&(chunk_x, chunk_y)).copied() {
                Some(id) if (id == max_fill_even_id || id == max_fill_odd_id) => filled_tile_state,
                Some(id) => id,
                None => unreachable!()
            };
            let north_chunk_state = match chunks.get(&(chunk_x, chunk_y - 1)).copied() {
                Some(id) if (id == max_fill_even_id || id == max_fill_odd_id) => filled_tile_state,
                Some(id) => id,
                None => empty_chunk_id
            };
            let south_chunk_state = match chunks.get(&(chunk_x, chunk_y + 1)).copied() {
                Some(id) if (id == max_fill_even_id || id == max_fill_odd_id) => filled_tile_state,
                Some(id) => id,
                None => empty_chunk_id
            };
            let east_chunk_state = match chunks.get(&(chunk_x + 1, chunk_y)).copied() {
                Some(id) if (id == max_fill_even_id || id == max_fill_odd_id) => filled_tile_state,
                Some(id) => id,
                None => empty_chunk_id
            };
            let west_chunk_state = match chunks.get(&(chunk_x - 1, chunk_y)).copied() {
                Some(id) if (id == max_fill_even_id || id == max_fill_odd_id) => filled_tile_state,
                Some(id) => id,
                None => empty_chunk_id
            };

            let adj_chunk_states = AdjChunkStates { curr: curr_chunk_state, north: north_chunk_state, south: south_chunk_state, east: east_chunk_state, west: west_chunk_state };
            let new_chunk_state = reach_map_transitions.get(&adj_chunk_states).copied().unwrap_or_else(|| compute_chunk_state(&tile_map, &mut reach_map_lookup, adj_chunk_states));
            if new_chunk_state != curr_chunk_state {
                // When chunk becomes non-empty, make adjacent non-generated chunks live
                if curr_chunk_state == 0 {
                    let adj_chunks = [(chunk_x - 1, chunk_y), (chunk_x + 1, chunk_y), (chunk_x, chunk_y - 1), (chunk_x, chunk_y + 1)];
                    for (adj_chunk_x, adj_chunk_y) in adj_chunks {
                        if !chunks.contains_key(&(adj_chunk_x, adj_chunk_y)) {
                            chunks.insert((adj_chunk_x, adj_chunk_y), 0);
                            new_live_chunks.insert((adj_chunk_x, adj_chunk_y));
                        }
                    }
                }

                if (new_chunk_state == max_fill_even_id) || (new_chunk_state == max_fill_odd_id) {
                    // When chunk enters a filled state, it is removed from live chunks
                    // Additionally, IDs are handled in a special manner, they are based on which state it is in on even-numbered steps
                    new_filled_chunks.insert((chunk_x, chunk_y));
                    let filled_chunk_id = match (new_chunk_state == max_fill_even_id) ^ (step_num % 2 == 0) {
                        true => max_fill_odd_id,
                        false => max_fill_even_id
                    };
                    changed_chunks.insert((chunk_x, chunk_y), filled_chunk_id);
                }
                else {
                    changed_chunks.insert((chunk_x, chunk_y), new_chunk_state);
                }
            }
        }

        // Update set of live chunks and carry out state changes
        for coords in new_live_chunks {
            chunks.insert(coords, 0);
            live_chunks.insert(coords);
        }
        for coords in new_filled_chunks {
            live_chunks.remove(&coords);
            num_filled_chunks += 1;
        }
        for (changed_chunk_coords, new_reach_id) in changed_chunks.iter() {
            chunks.insert(*changed_chunk_coords, *new_reach_id);
        }

        // Print test
        println!("Step {}", step_num + 1);
        print_chunks(&tile_map, &reach_map_lookup, &chunks, 3 - filled_tile_state);
    }

    // Count total reachable tiles
    // Count number of each chunk ID, calculate reachable tiles in each different map
    println!("{} live chunks", live_chunks.len());
    println!("{} filled chunks", num_filled_chunks);
    let mut end_chunk_id_count: HashMap<u32, u32> = HashMap::new();
    for coords in live_chunks {
        let chunk_id = chunks.get(&coords).copied().unwrap();
        end_chunk_id_count.insert(chunk_id, end_chunk_id_count.get(&chunk_id).copied().unwrap_or(0) + 1);
    }
    println!("{} empty chunks", end_chunk_id_count.get(&0).unwrap());
    
    let mut subtotal = 0;
    for (chunk_id, count) in end_chunk_id_count.iter() {
        let num_reachable_tiles: u64 = reach_map_lookup.get_reach_map(*chunk_id).unwrap().iter().filter(|r| **r).count().try_into().unwrap();
        subtotal += num_reachable_tiles * (*count as u64);
    }
    let end_filled_tile_state = ((total_step_count + start_index) % 2 + 1).try_into().unwrap();
    let filled_chunk_reachable_count: u32 = reach_map_lookup.get_reach_map(end_filled_tile_state).unwrap().iter().filter(|r| **r).count().try_into().unwrap();
    subtotal += (filled_chunk_reachable_count * num_filled_chunks) as u64;
    return subtotal;
}

fn compute_chunk_state(tile_map: &Vec<Vec<Tile>>, reach_map_lookup: &mut ReachMapLookup, AdjChunkStates { curr, north, south, east, west }: AdjChunkStates) -> u32 {
    //println!("Computing state for {curr} with neighbors {north}, {south}, {east}, {west}");
    let height = reach_map_lookup.height;
    let width = reach_map_lookup.width;
    let num_chunk_tiles = width * height;

    // Build simulation
    let mut sim_tile_map = vec![vec![Tile::Rock; 3 * width + 2]; 3 * height + 2];
    for (x_offset, y_offset) in [(1, 1), (0, 1), (2, 1), (1, 0), (1, 2)] {
        for i in 0..height {
            for j in 0..width {
                sim_tile_map[height * x_offset + i + 1][width * y_offset + j + 1] = tile_map[i][j];
            }
        }
    }

    let curr_chunk_map = reach_map_lookup.get_reach_map(curr).unwrap();
    let north_chunk_map = reach_map_lookup.get_reach_map(north).unwrap();
    let south_chunk_map = reach_map_lookup.get_reach_map(south).unwrap();
    let east_chunk_map = reach_map_lookup.get_reach_map(east).unwrap();
    let west_chunk_map = reach_map_lookup.get_reach_map(west).unwrap();

    let mut sim_reach_map = vec![vec![false; 3 * width + 2]; 3 * height + 2];
    for (x_offset, y_offset, reach_map) in [(1, 1, curr_chunk_map), (1, 0, north_chunk_map), (1, 2, south_chunk_map), (0, 1, west_chunk_map), (2, 1, east_chunk_map)] {
        for (i, tl) in reach_map.iter().enumerate() {
            let tile_x = x_offset * width + 1 + (i % height);
            let tile_y = y_offset * height + 1 + (i / height);
            sim_reach_map[tile_x][tile_y] = *tl;
        }
    }

    // Compute next step
    let mut sim_reach_map_next = vec![vec![false; 3 * width + 2]; 3 * height + 2];
    for j in 1..(3 * height + 1) {
        for i in 1..(3 * width + 1) {
            if sim_tile_map[i][j] == Tile::Garden && (sim_reach_map[i - 1][j] || sim_reach_map[i + 1][j] || sim_reach_map[i][j - 1] || sim_reach_map[i][j + 1]) {
                sim_reach_map_next[i][j] = true;
            }
        }
    }

    // Check ID of new chunk state, create new ID if state has not been recorded yet
    let new_reach_map = (0..num_chunk_tiles).into_iter().map(|i| sim_reach_map_next[width + 1 + (i % height)][height + 1 + (i / height)]).collect();
    let new_chunk_id;
    match reach_map_lookup.get_id(&new_reach_map) {
        Some(id) => { new_chunk_id = id },
        None => { new_chunk_id = reach_map_lookup.insert(&new_reach_map).unwrap() }
    }
    return new_chunk_id;
}

fn print_chunks(tile_map: &Vec<Vec<Tile>>, reach_map_lookup: &ReachMapLookup, chunks: &HashMap<(i32, i32), u32>, filled_tile_state: u32) {
    let width = reach_map_lookup.width;
    let height = reach_map_lookup.height;
    let min_chunk_x = chunks.keys().map(|(x, y)| *x).min().unwrap();
    let max_chunk_x = chunks.keys().map(|(x, y)| *x).max().unwrap();
    let min_chunk_y = chunks.keys().map(|(x, y)| *y).min().unwrap();
    let max_chunk_y = chunks.keys().map(|(x, y)| *y).max().unwrap();

    let horiz_chunks: usize = (max_chunk_x - min_chunk_x + 1).try_into().unwrap();
    let vert_chunks: usize = (max_chunk_y - min_chunk_y + 1).try_into().unwrap();
    let mut map_render = vec![vec![' '; width * horiz_chunks]; height * vert_chunks];
    for ((chunk_x, chunk_y), chunk_id) in chunks {
        let offset_chunk_x: usize = (*chunk_x - min_chunk_x).try_into().unwrap();
        let offset_chunk_y: usize = (*chunk_y - min_chunk_y).try_into().unwrap();

        let reach_map = match *chunk_id {
            1 | 2 => reach_map_lookup.get_reach_map(filled_tile_state).unwrap(),
            id => reach_map_lookup.get_reach_map(id).unwrap()
        };
        for (i, reachable) in reach_map.iter().enumerate() {
            let tile_x = i % height;
            let tile_y = i / height;
            if *reachable {
                map_render[offset_chunk_x * width + tile_x][offset_chunk_y * height + tile_y] = 'O';
            }
            else if *chunk_id == 1 || *chunk_id == 2 {
                map_render[offset_chunk_x * width + tile_x][offset_chunk_y * height + tile_y] = '/';
            }
            else if tile_map[tile_x][tile_y] == Tile::Garden {
                map_render[offset_chunk_x * width + tile_x][offset_chunk_y * height + tile_y] = '.';
            }
            else {
                map_render[offset_chunk_x * width + tile_x][offset_chunk_y * height + tile_y] = '#';
            }
        }
    }
    for ln in map_render {
        println!("{}", ln.iter().collect::<String>());
    }
}