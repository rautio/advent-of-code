use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn build_possibles(row: &str) -> Vec<String> {
    let mut possibles: Vec<String> = Vec::new();
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(row.to_string());
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        if !cur.chars().any(|c| c == '?') {
            // All clear, add!
            possibles.push(cur);
        } else {
            // There is a ?
            let i = cur.find('?').unwrap();
            // It's a #!
            let mut temp1: Vec<char> = cur.chars().collect();
            if let Some(ch) = temp1.get_mut(i) {
                *ch = '#';
            }
            q.push_back(temp1.into_iter().collect());
            // It's a .!
            let mut temp2: Vec<char> = cur.chars().collect();
            if let Some(ch) = temp2.get_mut(i) {
                *ch = '.';
            }
            q.push_back(temp2.into_iter().collect());
        }
    }
    return possibles;
}

fn is_valid(s: &str, groups: &Vec<u32>) -> bool {
    let working_groups: Vec<&str> = s.split(|c| c != '#').filter(|s| !s.is_empty()).collect();
    if working_groups.len() != groups.len() {
        return false;
    }
    for i in 0..working_groups.len() {
        if working_groups[i].len() as u32 != groups[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let now = Instant::now();
    let mut sum = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        let s = line.split(' ').collect::<Vec<&str>>();
        let row = s[0];
        let groups: Vec<u32> = s[1]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let possibles = build_possibles(row);
        for p in possibles.iter() {
            if is_valid(p, &groups) {
                sum += 1;
            }
        }
    }
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    // now = Instant::now();
    // println!("Part 2: {}", find_sim_steps(starts, &instr, &map));
    // println!("Done in: {:.2?}!", now.elapsed());
}
