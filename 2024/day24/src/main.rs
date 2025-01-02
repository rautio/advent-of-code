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

fn solve_ops(ops: &Vec<Op>, values: &HashMap<String, bool>) -> (HashMap<String, bool>, bool) {
    let mut res: HashMap<String, bool> = values.clone();
    let mut ops_to_check: VecDeque<Op> = VecDeque::from(ops.clone());
    let mut checked: HashMap<Op, Op> = HashMap::new();
    let mut solvable = true;

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
            checked = HashMap::new(); // Reset
        } else {
            if !checked.contains_key(&op) {
                checked.insert(op.clone(), op.clone());
                ops_to_check.push_back(op);
            } else {
                // Ciruclar
                solvable = false;
            }
        }
    }

    (res, solvable)
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

fn is_xy(s: String) -> bool {
    s.starts_with('x') || s.starts_with('y')
}

fn find_invalid_gates(ops: &Vec<Op>) -> String {
    let mut incorrect_gates: Vec<Op> = Vec::new();
    // Not pretty but it works.
    // Following a set of rules based on the fact that each Z (except the first and last) should be a full adder.
    for op in ops {
        if op.output.starts_with('z') && op.output != "z45" && op.gate != Gate::XOR {
            if op.output == "hfm" {
                println!("yup hfm1");
            }
            incorrect_gates.push(op.clone());
        } else if op.gate == Gate::XOR
            && is_xy(op.inputs.0.clone())
            && is_xy(op.inputs.1.clone())
            && !op.output.starts_with('z')
            && op.output != "z00"
        {
            if op.output == "hfm" {
                println!("yup hfm2");
            }
            let mut a_gate_used = false;
            for o in ops {
                if (o.inputs.0 == op.output || o.inputs.1 == op.output) && o.gate == Gate::AND {
                    a_gate_used = true;
                }
            }
            if !a_gate_used {
                incorrect_gates.push(op.clone());
            }
        } else if op.gate == Gate::XOR && op.output != "z00" {
            let mut is_fine = false;
            // XOR can take x and y to produce intermediate
            if is_xy(op.inputs.0.clone())
                && is_xy(op.inputs.1.clone())
                && !op.output.starts_with('z')
            {
                is_fine = true;
            }
            // XOR can take intermediate bits to produce 'z'
            if !is_xy(op.inputs.0.clone())
                && !is_xy(op.inputs.1.clone())
                && op.output.starts_with('z')
            {
                is_fine = true;
            }
            if !is_fine {
                incorrect_gates.push(op.clone());
            }
        } else if op.gate == Gate::AND
            && is_xy(op.inputs.0.clone())
            && is_xy(op.inputs.1.clone())
            && op.inputs.0 != "x00"
            && op.inputs.0 != "y00"
        {
            let mut or_gate_used = false;
            for o in ops {
                if (o.inputs.0 == op.output || o.inputs.1 == op.output) && o.gate == Gate::OR {
                    or_gate_used = true;
                }
            }
            if !or_gate_used {
                incorrect_gates.push(op.clone());
            }
        }
    }
    let mut out = incorrect_gates
        .into_iter()
        .map(|op| op.output)
        .collect::<Vec<String>>();

    out.sort();
    out.join(",")
}

lazy_static! {
    static ref re: Regex = Regex::new(r"(.*) (AND|OR|XOR) (.*) -> (.*)").unwrap();
}

fn parse_op(line: &str) -> Op {
    let cap = re.captures(line);
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
    Op {
        inputs,
        output,
        gate,
    }
}

fn main() {
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
            ops.push(parse_op(line));
        }
    }
    // Part 1
    // print_values(values.clone());
    let solved_values = solve_ops(&ops, &values).0;
    // print_values(solved_values.clone());
    println!("Part 1: {}", solve_z(solved_values.clone()));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    // println!("Part 2: {}", swapped_gates(&ops, &values));
    println!("Part 2: {}", find_invalid_gates(&ops));
    println!("Done in: {:?}!", now.elapsed());
}
