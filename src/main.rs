use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, io, iter};

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap_or_default()).collect();
    let part1 = part1(&lines);
    let part2 = part2_2(&lines);
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
    Conjunction(HashSet<String>, Vec<String>)
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
    for (module_str, output_list) in &parsed_lines {
        let output_list = output_list.iter().map(|s| String::from(*s)).collect();
        if module_str.starts_with('%') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::FlipFlop(output_list));
        }
        else if module_str.starts_with('&') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::Conjunction(HashSet::new(), output_list));
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
                inputs.insert(String::from(module_str.trim_start_matches(&['%', '&'])));
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
    let mut num_low_signals = 0;
    let mut num_high_signals = 0;
    let mut module_queue = VecDeque::new();

    for _ in (0..1000) {
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
                PulseModule::Conjunction(inputs, outputs) => {
                    output_module_names = outputs;
                },
            }
            
            // Determine state of source module
            let source_module_state = *state_map.get(source_module_name).unwrap();
    
            // Change state in output modules
            for dest_module_name in output_module_names {
                // Process incoming signal
                //println!("{:?}", state_map);
                //println!("{} --{:?}-> {}", source_module_name, source_module_state, dest_module_name);
                let mut state_changed_flag = false;
                if let Some(dest_module_connections) = connections_map.get(dest_module_name) {
                    match dest_module_connections {
                        PulseModule::Button(outputs) => {},
                        PulseModule::Broadcast(outputs) => {
                            state_map.insert(String::from(dest_module_name), source_module_state);
                            state_changed_flag = true;
                        },
                        PulseModule::FlipFlop(outputs) => {
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
                        PulseModule::Conjunction(inputs, outputs) => {
                            let dest_new_state;
                            if inputs.iter().all(|sgl| match state_map.get(sgl).unwrap() { Signal::High => true, Signal::Low => false }) {
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

    println!("num_low_signals: {}", num_low_signals);
    println!("num_high_signals: {}", num_high_signals);
    return num_low_signals * num_high_signals;
}

fn part2(lines: &Vec<String>) -> i32 {    // Parse input
    let mut parsed_lines = Vec::new();
    for ln in lines {
        let (module_str, output_list_str) = ln.split_once(" -> ").unwrap();
        let output_list: Vec<&str> = output_list_str.split(", ").collect();
        parsed_lines.push((module_str, output_list));
    }

    // Set up connections
    let mut connections_map: HashMap<String, PulseModule> = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        let output_list = output_list.iter().map(|s| String::from(*s)).collect();
        if module_str.starts_with('%') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::FlipFlop(output_list));
        }
        else if module_str.starts_with('&') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::Conjunction(HashSet::new(), output_list));
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
                inputs.insert(String::from(module_str.trim_start_matches(&['%', '&'])));
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
    //connections_map.insert(String::from("rx"), PulseModule::Broadcast(vec![]));
    //state_map.insert(String::from("rx"), Signal::High);

    let mut has_been_low = HashSet::new();
    let mut has_been_high = HashSet::new();
    for (module_name, state) in &state_map {
        match state {
            Signal::Low => {
                has_been_low.insert(String::from(module_name));
            },
            Signal::High => {
                has_been_high.insert(String::from(module_name));
            }
        }
    }

    // Process messages
    let mut num_low_signals = 0;
    let mut num_high_signals = 0;
    let mut module_queue = VecDeque::new();

    let mut button_press_counter = 0;
    println!("state_map: {:?}", state_map);
    'button_press_loop: for _ in 0..32000 {
        button_press_counter += 1;

        // Trigger button press
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
                PulseModule::Conjunction(inputs, outputs) => {
                    output_module_names = outputs;
                },
            }
            
            // Determine state of source module
            let source_module_state = *state_map.get(source_module_name).unwrap();
    
            // Change state in output modules
            for dest_module_name in output_module_names {
                // Check if 'low' signal was sent to module 'rx'
                if (source_module_state == Signal::Low) && (dest_module_name == "rx") {
                    println!("FINAL: {:?}", state_map);
                    break 'button_press_loop;
                }

                // Process incoming signal
                //println!("{:?}", state_map);
                //println!("{} --{:?}-> {}", source_module_name, source_module_state, dest_module_name);
                let mut state_changed_flag = false;
                if let Some(dest_module_connections) = connections_map.get(dest_module_name) {
                    match dest_module_connections {
                        PulseModule::Button(outputs) => {},
                        PulseModule::Broadcast(outputs) => {
                            state_map.insert(String::from(dest_module_name), source_module_state);
                            state_changed_flag = true;
                            match source_module_state {
                                Signal::Low => {
                                    if has_been_low.insert(String::from(dest_module_name)) {
                                        //println!("Cycle {}: '{}' was 'low'", button_press_counter, dest_module_name);
                                    }
                                },
                                Signal::High => {
                                    if has_been_high.insert(String::from(dest_module_name)) {
                                        //println!("Cycle {}: '{}' was 'high'", button_press_counter, dest_module_name);
                                    }
                                }
                            }
                        },
                        PulseModule::FlipFlop(outputs) => {
                            match source_module_state {
                                Signal::Low => {
                                    let dest_curr_state = state_map.get(dest_module_name).unwrap();
                                    let dest_new_state = match *dest_curr_state {
                                        Signal::Low => Signal::High,
                                        Signal::High => Signal::Low,
                                    };
                                    state_map.insert(String::from(dest_module_name), dest_new_state);
                                    state_changed_flag = true;
                                    match dest_new_state {
                                        Signal::Low => {
                                            if has_been_low.insert(String::from(dest_module_name)) {
                                                //println!("Cycle {}: '{}' was 'low'", button_press_counter, dest_module_name);
                                            }
                                        },
                                        Signal::High => {
                                            if has_been_high.insert(String::from(dest_module_name)) {
                                                //println!("Cycle {}: '{}' was 'high'", button_press_counter, dest_module_name);
                                            }
                                        }
                                    }
                                },
                                Signal::High => {}
                            }
                        },
                        PulseModule::Conjunction(inputs, outputs) => {
                            let dest_new_state;
                            if inputs.iter().all(|sgl| match state_map.get(sgl).unwrap() { Signal::High => true, Signal::Low => false }) {
                                dest_new_state = Signal::Low;
                            }
                            else {
                                dest_new_state = Signal::High;
                            }
                            state_map.insert(String::from(dest_module_name), dest_new_state);
                            state_changed_flag = true;
                            match dest_new_state {
                                Signal::Low => {
                                    if has_been_low.insert(String::from(dest_module_name)) {
                                        //println!("Cycle {}: '{}' was 'low'", button_press_counter, dest_module_name);
                                    }
                                },
                                Signal::High => {
                                    if has_been_high.insert(String::from(dest_module_name)) {
                                        //println!("Cycle {}: '{}' was 'high'", button_press_counter, dest_module_name);
                                    }
                                }
                            }
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
        /*
        if (*state_map.get("rx").unwrap() as u8 == Signal::Low as u8) {
            break 'button_press_loop;
        }
        */
        //println!("state_map: {:?}", state_map);
    }

    println!("num_low_signals: {}", num_low_signals);
    println!("num_high_signals: {}", num_high_signals);
    return button_press_counter;
}

fn part2_2(lines: &Vec<String>) -> i32 {
    let mut parsed_lines = Vec::new();
    for ln in lines {
        let (module_str, output_list_str) = ln.split_once(" -> ").unwrap();
        let output_list: Vec<&str> = output_list_str.split(", ").collect();
        parsed_lines.push((module_str, output_list));
    }

    // Set up connections
    let mut connections_map: HashMap<String, PulseModule> = HashMap::new();
    for (module_str, output_list) in &parsed_lines {
        let output_list = output_list.iter().map(|s| String::from(*s)).collect();
        if module_str.starts_with('%') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::FlipFlop(output_list));
        }
        else if module_str.starts_with('&') {
            connections_map.insert(String::from(&module_str[1..]), PulseModule::Conjunction(HashSet::new(), output_list));
        }
        else {
            connections_map.insert(String::from(*module_str), PulseModule::Broadcast(output_list));
        }
    }

    // Determine all inputs for conjunction modules
    for (module_str, output_list) in &parsed_lines {
        for dest_module in output_list {
            if let Some(PulseModule::Conjunction(inputs, outputs)) = connections_map.get_mut(*dest_module) {
                inputs.insert(String::from(module_str.trim_start_matches(&['%', '&'])));
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
    let subtree_1 = vec!["button", "broadcaster", "ml", "jc", "bs", "zv", "xm", "lx", "nv", "zm", "lq", "zg", "vn", "dg", "rd"];
    let subtree_2 = vec!["button", "broadcaster", "xs", "td", "fd", "jl", "vp", "hv", "xd", "lt", "mq", "zz", "pz", "qh", "bj"];
    let subtree_3 = vec!["button", "broadcaster", "kl", "tx", "xt", "hm", "zq", "tl", "vg", "mm", "xx", "km", "rg", "qf", "bf"];
    let subtree_4 = vec!["button", "broadcaster", "jn", "pl", "sb", "zc", "gj", "dl", "rj", "cz", "qp", "ng", "gt", "dk", "bc"];

    let subtrees = [subtree_0, subtree_1, subtree_2, subtree_3, subtree_4];

    for (i, s) in subtrees.iter().enumerate() {
        let subtree_nodes = HashSet::from_iter(s.iter().map(|m| String::from(*m)));
        let mut subtree_state = s.iter().map(|m| (String::from(*m), *state_map.get(*m).unwrap())).into_iter().collect();
        
        iterate_subgraph(&connections_map, &mut subtree_state, &subtree_nodes);
        iterate_subgraph(&connections_map, &mut subtree_state, &subtree_nodes);
        let init_state = subtree_state.clone();
        println!("init_state: {:?}", init_state);
        for button_press_counter in 1..100000 {
            iterate_subgraph(&connections_map, &mut subtree_state, &subtree_nodes);
            if subtree_state == init_state {
                println!("Subtree {} cycle length: {}", i, button_press_counter);
                break;
            }
        }
    }

    /*
    // Process messages
    let mut init_state_0 = HashMap::new();
    let mut init_state_1 = HashMap::new();
    let mut init_state_2 = HashMap::new();
    let mut init_state_3 = HashMap::new();
    let mut init_state_4 = HashMap::new();
    let mut cycle_0 = 0;
    let mut cycle_1 = 0;
    let mut cycle_2 = 0;
    let mut cycle_3 = 0;
    let mut cycle_4 = 0;

    for button_press_counter in (0..1) {
        iterate_subgraph(&connections_map, &mut state_map);

        // Check if all nodes in subtree have returned to default state
        if (button_press_counter == 0) {
            println!("state_map: {:?}", state_map);
            init_state_0 = subtree_0.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect();
            init_state_1 = subtree_1.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect();
            init_state_2 = subtree_2.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect();
            init_state_3 = subtree_3.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect();
            init_state_4 = subtree_4.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect();
            println!("init_state_0: {:?}", init_state_0);
            println!("init_state_1: {:?}", init_state_1);
            println!("init_state_2: {:?}", init_state_2);
            println!("init_state_3: {:?}", init_state_3);
            println!("init_state_4: {:?}", init_state_4);
        }
        else { 
            if cycle_0 == 0 && (init_state_0 == subtree_0.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect()) {
                cycle_0 = button_press_counter;
                println!("subtree_0 cycle length: {}", cycle_0);
            }
            if cycle_1 == 0 && (init_state_1 == subtree_1.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect()) {
                cycle_1 = button_press_counter;
                println!("subtree_1 cycle length: {}", cycle_1);
            }
            if cycle_2 == 0 && (init_state_2 == subtree_2.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect()) {
                cycle_2 = button_press_counter;
                println!("subtree_2 cycle length: {}", cycle_2);
            }
            if cycle_3 == 0 && (init_state_3 == subtree_3.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect()) {
                cycle_3 = button_press_counter;
                println!("subtree_3 cycle length: {}", cycle_3);
            }
            if cycle_4 == 0 && (init_state_4 == subtree_4.map(|m| (m, *state_map.get(m).unwrap())).into_iter().collect()) {
                cycle_4 = button_press_counter;
                println!("subtree_4 cycle length: {}", cycle_4);
            }
        }
    }
    */
    return 0;
}

fn iterate_subgraph(connections_map: &HashMap<String, PulseModule>, state_map: &mut HashMap<String, Signal>, subgraph: &HashSet<String>) {
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
            if subgraph.contains(dest_module_name.as_str()) {
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
                        let dest_new_state;
                        if inputs.iter().all(|sgl| match state_map.get(sgl).unwrap() { Signal::High => true, Signal::Low => false }) {
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
            if state_changed_flag {
                module_queue.push_back(dest_module_name);
            }
        }
            }
    }
}