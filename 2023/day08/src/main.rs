use num::integer::lcm;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn find_steps(
    start: &String,
    instr: &Vec<char>,
    map: &HashMap<String, Vec<String>>,
    use_full: bool,
) -> u32 {
    let mut steps = 0;
    let mut idx = 0;
    let mut cur = start;
    loop {
        let i = idx % instr.len();
        if (!use_full && cur.ends_with("Z")) || (use_full && cur.ends_with("ZZZ")) {
            return steps;
        }
        if instr[i] == 'L' {
            cur = &map[cur][0];
        }
        if instr[i] == 'R' {
            cur = &map[cur][1];
        }
        idx += 1;
        steps += 1;
    }
}

fn find_sim_steps(
    starts: Vec<String>,
    instr: &Vec<char>,
    map: &HashMap<String, Vec<String>>,
) -> u64 {
    let mut res: u64 = 1;
    for s in starts {
        let steps = find_steps(&s, instr, map, false);
        res = lcm(res, steps as u64);
    }
    return res;
}

fn main() {
    let mut now = Instant::now();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut instr: Vec<char> = Vec::new();
    let start = String::from("AAA");
    let mut starts: Vec<String> = Vec::new();

    // Parse all the numbers and parts
    for (i, line) in read_to_string("./input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
    {
        if i == 0 {
            instr = line.chars().collect();
        }
        if i > 1 {
            let s = line[..3].to_string();
            if s.ends_with("A") {
                starts.push(s.clone());
            }
            let l = line[7..10].to_string();
            let r = line[12..15].to_string();
            map.insert(s, vec![l, r]);
        }
    }
    println!("Part 1: {}", find_steps(&start, &instr, &map, true));
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    now = Instant::now();
    println!("Part 2: {}", find_sim_steps(starts, &instr, &map));
    println!("Done in: {:.2?}!", now.elapsed());
}
