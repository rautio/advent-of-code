use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: usize,
    y: usize,
}
// . # . # #
//  ^ ^ ^
//  1 2 3
fn is_mirror(line: &String, idx: usize) -> bool {
    let (mut s, mut e) = line.split_at(idx);
    if s.len() > e.len() {
        s = &s[s.len() - e.len()..];
    }
    if e.len() > s.len() {
        e = &e[..s.len()];
    }
    for (i, c) in s.chars().rev().into_iter().enumerate() {
        if c != e.chars().nth(i).unwrap() {
            return false;
        }
    }
    return true;
}

// -1 means no mirror
fn find_mirror(lines: &Vec<String>) -> i32 {
    let mut possible: VecDeque<usize> = VecDeque::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            for j in 0..line.len() {
                // Check every item (except the last or first)
                if j != line.len() - 1 && is_mirror(line, j + 1) {
                    possible.push_back(j);
                }
            }
        } else {
            let mut new_p: VecDeque<usize> = VecDeque::new();
            while possible.len() > 0 {
                let idx = possible.pop_front().unwrap();
                if is_mirror(line, idx + 1) {
                    new_p.push_back(idx);
                }
            }
            possible = new_p;
        }
    }
    if possible.len() == 1 {
        return possible[0] as i32 + 1;
    }
    if possible.len() > 1 {
        panic!("Found more than one vertical mirror")
    }
    return -1;
}

fn main() {
    let now = Instant::now();
    let mut sum = 0;
    let mut grids: Vec<Vec<String>> = Vec::new();
    let mut grids_h: Vec<Vec<String>> = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut cur_h: Vec<String> = Vec::new();
    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            grids.push(cur.clone());
            grids_h.push(cur_h.clone());
            cur = Vec::new();
            cur_h = Vec::new();
        } else {
            let l = String::from(line);
            cur.push(l.clone());
            for (i, c) in l.chars().into_iter().enumerate() {
                if cur_h.len() < i + 1 {
                    cur_h.push(String::from(c));
                } else {
                    let h = &mut cur_h[i];
                    h.push(c);
                    cur_h[i] = h.clone();
                }
            }
        }
    }
    // Dont forget the last one
    grids.push(cur.clone());
    grids_h.push(cur_h.clone());
    // Check for mirrors
    for grid in &grids {
        let idx = find_mirror(grid);
        if idx > -1 {
            sum += idx;
        }
    }
    for grid in &grids_h {
        let idx = find_mirror(grid);
        if idx > -1 {
            sum += idx * 100;
        }
    }
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    // now = Instant::now();
    // println!("Part 2: {}", find_sim_steps(starts, &instr, &map));
    // println!("Done in: {:.2?}!", now.elapsed());
}
