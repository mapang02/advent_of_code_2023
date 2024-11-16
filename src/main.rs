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

struct ReachMapIdLookup {
    reach_map_ids: HashMap<Vec<bool>, u32>,
    reach_map_lookup: HashMap<u32, Vec<bool>>,
    num_reach_maps: u32
}
impl ReachMapIdLookup {
    fn new() -> ReachMapIdLookup {
        return ReachMapIdLookup { reach_map_ids: HashMap::new(), reach_map_lookup: HashMap::new(), num_reach_maps: 0 }
    }
    fn get_id(&mut self, reach_map: &Vec<bool>) -> Option<u32> {
        return self.reach_map_ids.get(reach_map).copied();
    }
    fn get_reach_map(&mut self, id: u32) -> Option<&Vec<bool>> {
        return self.reach_map_lookup.get(&id);
    }
    fn insert(&mut self, reach_map: &Vec<bool>) -> Option<u32> {
        if !self.reach_map_ids.contains_key(reach_map) {
            self.reach_map_ids.insert(reach_map.clone(), self.num_reach_maps);
            self.reach_map_lookup.insert(self.num_reach_maps, reach_map.clone());
            self.num_reach_maps += 1;
            return Some(self.num_reach_maps);
        }
        return None;
    }
}

fn part2(lines: &Vec<String>) -> i32 {
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
    let mut reach_map_ids = ReachMapIdLookup::new();
    let mut reach_map_transitions: HashMap<AdjChunkStates, u32> = HashMap::new();
    let mut chunks: HashMap<(i32, i32), u32> = HashMap::new();
    let mut live_chunks: HashSet<(i32, i32)> = HashSet::new();
    let mut num_filled_chunks = 0;

    // Pre-generate maximally filled chunks
    let empty_chunk = vec![false; width * height];
    let max_fill_even = (0..(width * height)).map(|i| i % 2 == 0 && tile_map[i % height][i / height] == Tile::Garden).collect();
    let max_fill_odd = (0..(width * height)).map(|i| i % 2 == 1 && tile_map[i % height][i / height] == Tile::Garden).collect();

    reach_map_ids.insert(&empty_chunk);
    reach_map_ids.insert(&max_fill_even);
    reach_map_ids.insert(&max_fill_odd);
    
    // Generate starting chunk
    let mut start_reach = vec![false; width * height];
    start_reach[start_index] = true;
    let start_reach_id = reach_map_ids.insert(&start_reach).unwrap();
    chunks.insert((0, 0), start_reach_id);
    live_chunks.insert((0, 0));
    for adj_chunk_coords in [(-1, 0), (1, 0), (0, -1), (0, 1)] { // Make chunks adjacent to start chunk live
        chunks.insert(adj_chunk_coords, 0);
        live_chunks.insert(adj_chunk_coords);
    }

    // Iterate through steps
    let total_step_count = 6;
    for step_num in 0..total_step_count {
        // Determine which chunks to change
        let mut changed_chunks: HashMap<(i32, i32), u32> = HashMap::new();
        let mut new_live_chunks: HashSet<(i32, i32)> = HashSet::new();
        for (chunk_x, chunk_y) in live_chunks.iter().copied() {
            // Attempt to look up state change based on neighboring chunks, compute it otherwise
            let curr_chunk_state = chunks.get(&(chunk_x, chunk_y)).copied().unwrap();
            let north_chunk_state = chunks.get(&(chunk_x, chunk_y + 1)).copied().unwrap_or(0);
            let south_chunk_state = chunks.get(&(chunk_x, chunk_y - 1)).copied().unwrap_or(0);
            let east_chunk_state = chunks.get(&(chunk_x + 1, chunk_y)).copied().unwrap_or(0);
            let west_chunk_state = chunks.get(&(chunk_x - 1, chunk_y)).copied().unwrap_or(0);

            let adj_chunk_states = AdjChunkStates { curr: curr_chunk_state, north: north_chunk_state, south: south_chunk_state, east: east_chunk_state, west: west_chunk_state };
            let new_chunk_state = reach_map_transitions.get(&adj_chunk_states).copied().unwrap_or_else(|| compute_chunk_state(&reach_map_ids, adj_chunk_states));
            if new_chunk_state != curr_chunk_state {
                changed_chunks.insert((chunk_x, chunk_y), new_chunk_state);

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
            }
        }

        // Add newly live chunks and carry out state changes
        for coords in new_live_chunks {
            chunks.insert(coords, 0);
            live_chunks.insert(coords);
        }
        for (changed_chunk_coords, new_reach_id) in changed_chunks.iter() {
            chunks.insert(*changed_chunk_coords, *new_reach_id);
        }
    }

    return 0;
}

fn compute_chunk_state(reach_map_ids: &ReachMapIdLookup, AdjChunkStates { curr, north, south, east, west }: AdjChunkStates) -> u32 {
    let curr_chunk_map = reach_map_ids;
}