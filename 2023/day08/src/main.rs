use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn find_steps(start: &String, instr: Vec<char>, map: HashMap<String, Vec<String>>) -> u32 {
    let mut steps = 0;
    let mut idx = 0;
    let mut cur = start;
    loop {
        let i = idx % instr.len();
        if cur == "ZZZ" {
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

fn main() {
    let mut now = Instant::now();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut instr: Vec<char> = Vec::new();
    let mut start = String::from("AAA");

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
            let l = line[7..10].to_string();
            let r = line[12..15].to_string();
            map.insert(s, vec![l, r]);
        }
    }
    println!("Part 1: {}", find_steps(&start, instr, map));
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
