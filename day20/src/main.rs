use std::{collections::{HashMap, HashSet, VecDeque}, io};

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Signal {
    Low,
    High
}

#[derive(Clone, Debug)]
enum PulseModule {
    Button(Vec<String>),
    Broadcast(Vec<String>),
    FlipFlop(Vec<String>),
    Conjunction(HashMap<String, Signal>, Vec<String>)
}

fn part1(lines: &Vec<String>) -> i32 {
    // Parse input
    let mut parsed_lines = Vec::new();
    for ln in lines {
        let (module_str, output_list_str) = ln.split_once(" -> ").unwrap();
        let output_list: Vec<&str> = output_list_str.split(", ").collect();
        parsed_lines.push((module_str, output_list));
    }

    // Set up connections
    let mut connections_map: HashMap<String, PulseModule> = HashMap::new();
    let mut conj_mem_map: HashMap<String, HashMap<String, Signal>> = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        let output_list = output_list.iter().map(|s| String::from(*s)).collect();
        if module_str.starts_with('%') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::FlipFlop(output_list));
        }
        else if module_str.starts_with('&') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::Conjunction(HashMap::new(), output_list));
            conj_mem_map.insert(String::from(&module_str[1..]), HashMap::new());
        }
        else {
            connections_map.insert(String::from(*module_str), PulseModule::Broadcast(output_list));
        }
    }
    // Set up dummy modules for names that appear in output but not input
    /*
    for (module_str, output_list) in &parsed_lines {
        for dest_module in output_list {
            if connections_map.get(*dest_module).is_none() {
                connections_map.insert(String::from(*dest_module),PulseModule::Button(Vec::new()));
            }
        }
    }
    */
    // Determine all inputs for conjunction modules
    for (module_str, output_list) in &parsed_lines {
        for dest_module in output_list {
            if let Some(PulseModule::Conjunction(inputs, outputs)) = connections_map.get_mut(*dest_module) {
                inputs.insert(String::from(module_str.trim_start_matches(&['%', '&'])), Signal::Low);
            }
            if let Some(conj_mem) = conj_mem_map.get_mut(*dest_module) {
                conj_mem.insert(String::from(module_str.trim_start_matches(&['%', '&'])), Signal::Low);
            }
        }
    }

    // Set up module states
    let mut state_map = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        if module_str.starts_with('%') {
            state_map.insert(String::from(&module_str[1..]), Signal::Low);
        }
        else if module_str.starts_with('&') {
            state_map.insert(String::from(&module_str[1..]), Signal::High);
        }
        else {
            state_map.insert(String::from(*module_str), Signal::Low);
        }
    }
    // Set up dummy modules for names that appear in output but not input
    /*
    for (module_str, output_list) in &parsed_lines {
        for dest_module in output_list {
            state_map.get(*dest_module).get_or_insert(&Signal::Low);
        }
    }
    */

    // Set up hard-coded button module
    connections_map.insert(String::from("button"), PulseModule::Button(vec![String::from("broadcaster")]));
    state_map.insert(String::from("button"), Signal::Low);

    // Process messages
    let mut total_low_signals = 0;
    let mut total_high_signals = 0;

    for _ in (0..1000) {
        let (num_low_signals, num_high_signals) = iterate_subgraph(&connections_map, &mut state_map, &mut conj_mem_map,&HashSet::new());
        total_low_signals += num_low_signals;
        total_high_signals += num_high_signals;
    }

    println!("total_low_signals: {}", total_low_signals);
    println!("total_high_signals: {}", total_high_signals);
    println!("Solution: {}", total_low_signals * total_high_signals);
    return total_low_signals * total_high_signals;
}

fn part2(lines: &Vec<String>) -> usize {
    let mut parsed_lines = Vec::new();
    for ln in lines {
        let (module_str, output_list_str) = ln.split_once(" -> ").unwrap();
        let output_list: Vec<&str> = output_list_str.split(", ").collect();
        parsed_lines.push((module_str, output_list));
    }

    // Set up connections
    let mut connections_map: HashMap<String, PulseModule> = HashMap::new();
    let mut conj_mem_map: HashMap<String, HashMap<String, Signal>> = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        let output_list = output_list.iter().map(|s| String::from(*s)).collect();
        if module_str.starts_with('%') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::FlipFlop(output_list));
        }
        else if module_str.starts_with('&') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::Conjunction(HashMap::new(), output_list));
            conj_mem_map.insert(String::from(&module_str[1..]), HashMap::new());
        }
        else {
            connections_map.insert(String::from(*module_str), PulseModule::Broadcast(output_list));
        }
    }

    // Determine all inputs for conjunction modules
    for (module_str, output_list) in &parsed_lines {
        for dest_module in output_list {
            if let Some(PulseModule::Conjunction(inputs, outputs)) = connections_map.get_mut(*dest_module) {
                inputs.insert(String::from(module_str.trim_start_matches(&['%', '&'])), Signal::Low);
            }
            if let Some(conj_mem) = conj_mem_map.get_mut(*dest_module) {
                conj_mem.insert(String::from(module_str.trim_start_matches(&['%', '&'])), Signal::Low);
            }
        }
    }

    // Set up module states
    let mut state_map = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        if module_str.starts_with('%') {
            state_map.insert(String::from(&module_str[1..]), Signal::Low);
        }
        else if module_str.starts_with('&') {
            state_map.insert(String::from(&module_str[1..]), Signal::High);
        }
        else {
            state_map.insert(String::from(*module_str), Signal::Low);
        }
    }

    // Set up hard-coded button module
    connections_map.insert(String::from("button"), PulseModule::Button(vec![String::from("broadcaster")]));
    state_map.insert(String::from("button"), Signal::Low);

    // Set up subtrees, count number of iterations needed to complete cycle
    let subtree_0 = vec!["button", "broadcaster"];
    let subtree_1 = vec!["button", "broadcaster", "ml", "jc", "bs", "zv", "xm", "lx", "nv", "zm", "lq", "zg", "vn", "dg", "rd", "ch"];
    let subtree_2 = vec!["button", "broadcaster", "xs", "td", "fd", "jl", "vp", "hv", "xd", "lt", "mq", "zz", "pz", "qh", "bj", "th"];
    let subtree_3 = vec!["button", "broadcaster", "kl", "tx", "xt", "hm", "zq", "tl", "vg", "mm", "xx", "km", "rg", "qf", "bf", "gh"];
    let subtree_4 = vec!["button", "broadcaster", "jn", "pl", "sb", "zc", "gj", "dl", "rj", "cz", "qp", "ng", "gt", "dk", "bc", "sv"];

    let subtrees = [subtree_0, subtree_1, subtree_2, subtree_3, subtree_4];
    let mut cycle_lengths = Vec::new();

    for (i, s) in subtrees.iter().enumerate() {
        let subtree_nodes = Vec::from_iter(s.iter().map(|m| String::from(*m)));
        let subtree_node_set = HashSet::from_iter(subtree_nodes.clone());
        let mut subtree_state: HashMap<String, Signal> = subtree_nodes.iter().map(|m| ((*m).clone(), *state_map.get(m).unwrap())).into_iter().collect();
        let mut subtree_conj_mem = conj_mem_map.clone();
        let mut swap_indices: Vec<usize> = Vec::new();

        iterate_subgraph(&connections_map, &mut subtree_state,&mut subtree_conj_mem, &subtree_node_set);
        let init_state = subtree_state.clone();
        let mut prev_output_state = (*subtree_state.get(*s.last().unwrap()).unwrap()).clone();
        //println!("init_state: {:?}", init_state);
        for button_press_counter in 1..10000 {
            iterate_subgraph(&connections_map, &mut subtree_state,&mut subtree_conj_mem, &subtree_node_set);
            let next_output_state = (*subtree_state.get(*s.last().unwrap()).unwrap()).clone();
            if next_output_state != prev_output_state {
                swap_indices.push(i);
                prev_output_state = next_output_state;
            }
            if subtree_state == init_state {
                println!("Subtree {} cycle length: {}", i, button_press_counter);
                cycle_lengths.push(button_press_counter);
                //println!("{:?}", swap_indices);
                //println!("subtree_state: {:?}", subtree_state);
                break;
            }
        }
    }
    return cycle_lengths.into_iter().product();
}

fn iterate_subgraph(connections_map: &HashMap<String, PulseModule>, state_map: &mut HashMap<String, Signal>, conj_mem_map: &mut HashMap<String, HashMap<String, Signal>>, subgraph: &HashSet<String>) -> (i32, i32) {
    let mut num_low_signals = 0;
    let mut num_high_signals = 0;
    let mut module_queue = VecDeque::new();
    module_queue.push_back("button");
    while let Some(source_module_name) = module_queue.pop_front() {
        // Find modules to output signals to
        let output_module_names;
        match connections_map.get(source_module_name).unwrap() {
            PulseModule::Button(outputs) => {
                output_module_names = outputs;
            },
            PulseModule::Broadcast(outputs) => {
                output_module_names = outputs;
            },
            PulseModule::FlipFlop(outputs) => {
                output_module_names = outputs;
            },
            PulseModule::Conjunction(_, outputs) => {
                output_module_names = outputs;
            },
        }
        
        // Determine state of source module
        let source_module_state = *state_map.get(source_module_name).unwrap();

        // Change state in output modules
        for dest_module_name in output_module_names {
            if subgraph.is_empty() || subgraph.contains(dest_module_name.as_str()) {
                // Process incoming signal
                //println!("{:?}", state_map);
                //println!("{} --{:?}-> {}", source_module_name, source_module_state, dest_module_name);
                let mut state_changed_flag = false;
                if let Some(dest_module_connections) = connections_map.get(dest_module_name) {
                    match dest_module_connections {
                        PulseModule::Button(_) => {},
                        PulseModule::Broadcast(_) => {
                            state_map.insert(String::from(dest_module_name), source_module_state);
                            state_changed_flag = true;
                        },
                        PulseModule::FlipFlop(_) => {
                            match source_module_state {
                                Signal::Low => {
                                    let dest_curr_state = state_map.get(dest_module_name).unwrap();
                                    let dest_new_state = match *dest_curr_state {
                                        Signal::Low => Signal::High,
                                        Signal::High => Signal::Low,
                                    };
                                    state_map.insert(String::from(dest_module_name), dest_new_state);
                                    state_changed_flag = true;
                                },
                                Signal::High => {}
                            }
                        },
                        PulseModule::Conjunction(inputs, _) => {
                            // Update conjunction module memory
                            let module_mem = conj_mem_map.get_mut(dest_module_name).unwrap();
                            if *module_mem.get(source_module_name).unwrap() != source_module_state {
                                module_mem.insert(String::from(source_module_name), source_module_state);
                            }

                            // Update module state based on updated memory
                            let dest_new_state;
                            if module_mem.values().all(|sgl| *sgl == Signal::High) {
                                dest_new_state = Signal::Low;
                            }
                            else {
                                dest_new_state = Signal::High;
                            }
                            
                            state_map.insert(String::from(dest_module_name), dest_new_state);
                            state_changed_flag = true;
                        },
                    }
                }
                // Add destination to processing queue
                match source_module_state {
                    Signal::Low => { num_low_signals += 1 },
                    Signal::High => { num_high_signals += 1 }
                }
                if state_changed_flag {
                    module_queue.push_back(dest_module_name);
                }
            }
        }
    }
    return (num_low_signals, num_high_signals);
}