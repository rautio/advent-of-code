use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Op {
    inputs: (String, String),
    output: String,
    gate: Gate,
}

fn solve_ops(ops: Vec<Op>, values: &HashMap<String, bool>) -> HashMap<String, bool> {
    let mut res: HashMap<String, bool> = values.clone();
    let mut ops_to_check: VecDeque<Op> = VecDeque::from(ops);

    while ops_to_check.len() > 0 {
        let op = ops_to_check.pop_front().unwrap();
        if res.contains_key(&op.inputs.0) && res.contains_key(&op.inputs.1) {
            match op.gate {
                Gate::AND => {
                    res.insert(
                        op.output,
                        *res.get(&op.inputs.0).unwrap() && *res.get(&op.inputs.1).unwrap(),
                    );
                }
                Gate::OR => {
                    res.insert(
                        op.output,
                        *res.get(&op.inputs.0).unwrap() || *res.get(&op.inputs.1).unwrap(),
                    );
                }
                Gate::XOR => {
                    res.insert(
                        op.output,
                        *res.get(&op.inputs.0).unwrap() != *res.get(&op.inputs.1).unwrap(),
                    );
                }
            }
        } else {
            ops_to_check.push_back(op);
        }
    }

    res
}

fn solve_z(values: HashMap<String, bool>) -> i64 {
    let mut z: i64 = 0;
    let mut i = 0;
    let mut v: Vec<(String, bool)> = values.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    for (s, b) in v {
        if s.starts_with('z') {
            let base: i64 = 2;
            if b {
                z += base.pow(i);
            }
            i += 1;
        }
    }
    z
}

fn print_values(values: HashMap<String, bool>) {
    let mut v: Vec<(String, bool)> = values.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    for (s, b) in v {
        let mut val = 0;
        if b {
            val = 1;
        }
        println!("{}: {}", s, val);
    }
}

fn main() {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(.*) (AND|OR|XOR) (.*) -> (.*)").unwrap();
    }
    let mut now = Instant::now();
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut ops: Vec<Op> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let cap = re.captures(line);
        if cap.is_none() {
            let splits: Vec<&str> = line.split(": ").collect();
            if splits.len() > 1 {
                values.insert(splits[0].to_string(), splits[1] == "1");
            }
        } else {
            let caps = cap.unwrap();
            let inputs = (
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(3).unwrap().as_str().to_string(),
            );
            let output = caps.get(4).unwrap().as_str().to_string();
            let gate = match caps.get(2).unwrap().as_str() {
                "AND" => Gate::AND,
                "OR" => Gate::OR,
                "XOR" => Gate::XOR,
                _ => panic!("Unknown gate"),
            };
            ops.push(Op {
                inputs,
                output,
                gate,
            });
        }
    }
    // Part 1
    // print_values(values.clone());
    let solved_values = solve_ops(ops, &values);
    // print_values(solved_values.clone());
    println!("Part 1: {}", solve_z(solved_values.clone()));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
