use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum ModuleTypes {
    FlipFlop,
    Broadcaster,
    Conjunction,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Module {
    module_type: ModuleTypes,
    outputs: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    input: String,
    pulse: Pulse,
    from: String,
}

/**
 * Rules:
 *  - Flip Flop ignores all high pulses (does nothing)
 *  - Flip Flop if receiving low pulse flips between on/off
 *  - Flip Flop starts off
 *  - If Flip Flop is off and receives low pulse - it turns on and sends high pulse
 *  - If Flip Flop is on and receives low pulse - it turns off and sends a low pulse
 *
 *  - Conjunction tracks the pulses from all its inputs
 *  - When it receives a pulse - if it has tracked all high pulses it sends a low pulse
 *  - If the con does not remember all high pulses it sends a high pulse
 */

fn send_pulses(
    map: &HashMap<String, Module>,
    flip_states: &mut HashMap<String, bool>,
    con_states: &mut HashMap<String, HashMap<String, Pulse>>,
) -> (u32, u32) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut pulses: VecDeque<State> = VecDeque::new();
    pulses.push_back(State {
        input: String::from("broadcaster"),
        pulse: Pulse::Low,
        from: String::from("button"),
    });
    while pulses.len() > 0 {
        let p = pulses.pop_front().unwrap();
        // println!("{:?} -{:?} -> {:?}", p.from, p.pulse, p.input);
        if p.pulse == Pulse::High {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }
        let mut new_pulse = p.pulse.clone();
        let module = map.get(&p.input).unwrap();
        if module.module_type == ModuleTypes::FlipFlop {
            if new_pulse == Pulse::High {
                // Do nothing if its a high pulse
                continue;
            } else {
                let is_on = flip_states.get(&p.input).unwrap();
                if *is_on {
                    new_pulse = Pulse::Low;
                } else {
                    new_pulse = Pulse::High;
                }
                flip_states.insert(p.input.clone(), !is_on);
            }
        } else if module.module_type == ModuleTypes::Conjunction {
            let inputs = con_states.get(&p.input).unwrap();
            let mut all_high = true;
            for (_i, pulse) in inputs {
                if *pulse == Pulse::Low {
                    all_high = false;
                    break;
                }
            }
            if all_high {
                new_pulse = Pulse::Low;
            } else {
                new_pulse = Pulse::High;
            }
        }
        let p_input = &p.input;
        for out in &module.outputs {
            if map.contains_key(out) {
                let output_module = map.get(out).unwrap();
                if output_module.module_type == ModuleTypes::Conjunction {
                    let mut inputs = con_states.get(out).unwrap().clone();
                    inputs.insert(p_input.clone(), new_pulse.clone());
                    con_states.insert(out.clone(), inputs);
                }
                pulses.push_back(State {
                    input: out.clone(),
                    pulse: new_pulse.clone(),
                    from: p.input.clone(),
                });
            } else {
                // Its an unknown so need to count the pulses now
                if new_pulse == Pulse::High {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
            }
        }
    }
    return (low_pulses, high_pulses);
}

fn iterate(map: &HashMap<String, Module>, count_iterations: usize) -> u32 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    // [name] -> on/off
    let mut flip_states: HashMap<String, bool> = HashMap::new();
    // [name] -> ( [input] -> high/low )
    let mut con_states: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
    // Prep states
    for (input, m) in map {
        if m.module_type == ModuleTypes::FlipFlop {
            flip_states.insert(input.to_string(), false);
        }
        for out in m.outputs.clone() {
            // Check if the output is an input to a conjunction
            if map.contains_key(&out) {
                let out_module = map.get(&out).unwrap();
                if out_module.module_type == ModuleTypes::Conjunction {
                    // Yes its a conjunction - map it
                    if con_states.contains_key(&out) {
                        let mut con = con_states.get(&out).unwrap().clone();
                        con.insert(input.to_string(), Pulse::Low);
                        *con_states.get_mut(&out).unwrap() = con.clone();
                    } else {
                        let mut new_con: HashMap<String, Pulse> = HashMap::new();
                        new_con.insert(input.to_string(), Pulse::Low);
                        con_states.insert(out, new_con);
                    }
                }
            }
        }
    }
    // Run iterations
    for _i in 0..count_iterations {
        let (low, high) = send_pulses(&map, &mut flip_states, &mut con_states);
        low_pulses += low;
        high_pulses += high;
    }
    return low_pulses * high_pulses;
}

fn parse_input(input_file: &str) -> HashMap<String, Module> {
    let mut map: HashMap<String, Module> = HashMap::new();
    for line in read_to_string(input_file).unwrap().lines() {
        let split: Vec<&str> = line.split(" -> ").collect();
        let input = split[0].to_string();
        let outputs: Vec<String> = split[1]
            .split(", ")
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        if input == "broadcaster" {
            map.insert(
                input,
                Module {
                    module_type: ModuleTypes::Broadcaster,
                    outputs: outputs,
                },
            );
        } else {
            let module_type = if &input[0..1] == "%" {
                ModuleTypes::FlipFlop
            } else {
                ModuleTypes::Conjunction
            };
            map.insert(
                input[1..].to_string(),
                Module {
                    module_type,
                    outputs,
                },
            );
        }
    }
    return map;
}

fn main() {
    let mut now = Instant::now();
    let map = parse_input("./input.txt");
    let part1 = iterate(&map, 1000);
    println!("Part 1: {}", part1);
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    // println!("Part 2: {:?}", get_combinations(&workflows));
    // println!("Done in: {:.2?}!", now.elapsed());
}
