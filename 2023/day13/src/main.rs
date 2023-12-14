use std::collections::HashMap;
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
        panic!("Found more than one mirror")
    }
    return -1;
}

fn count_smudges(line: &String, idx: usize) -> u32 {
    let mut count = 0;
    let (mut s, mut e) = line.split_at(idx);
    if s.len() > e.len() {
        s = &s[s.len() - e.len()..];
    }
    if e.len() > s.len() {
        e = &e[..s.len()];
    }
    for (i, c) in s.chars().rev().into_iter().enumerate() {
        if c != e.chars().nth(i).unwrap() {
            count += 1;
        }
    }
    return count;
}

// -1 means no mirror
fn find_mirror_smudges(lines: &Vec<String>) -> i32 {
    let mut possible: VecDeque<usize> = VecDeque::new();
    // Tracks how many smudges need to be fixed for the column to be a mirror
    let mut smudges: HashMap<usize, u32> = HashMap::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            for j in 0..line.len() {
                // Check every item (except the last or first)
                let c = count_smudges(line, j + 1);
                if j != line.len() - 1 && c <= 1 {
                    smudges.insert(j, c);
                    possible.push_back(j);
                }
            }
        } else {
            let mut new_p: VecDeque<usize> = VecDeque::new();
            while possible.len() > 0 {
                let idx = possible.pop_front().unwrap();
                let c = count_smudges(line, idx + 1);
                if c == 0 {
                    new_p.push_back(idx);
                }
                if c == 1 && *smudges.get(&idx).unwrap() == 0 {
                    new_p.push_back(idx);
                    smudges.insert(idx, c);
                }
            }
            possible = new_p;
        }
    }
    for p in &possible {
        if smudges.contains_key(&p) && *smudges.get(&p).unwrap() == 1 {
            return *p as i32 + 1;
        }
    }
    // No new reflection found (possible for Part 2)
    return -1;
}

fn main() {
    let mut now = Instant::now();
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
    for i in 0..grids.len() {
        let v = find_mirror(&grids[i]);
        if v > -1 {
            sum += v;
        }
        let h = find_mirror(&grids_h[i]);
        if h > -1 {
            sum += h * 100;
        }
    }
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    // Part 2
    now = Instant::now();
    sum = 0;
    for i in 0..grids.len() {
        let v = find_mirror_smudges(&grids[i]);
        if v > -1 {
            sum += v;
        }
        let h = find_mirror_smudges(&grids_h[i]);
        if h > -1 {
            sum += h * 100;
        }
    }
    println!("Part 2: {}", sum);
    println!("Done in: {:.2?}!", now.elapsed());
}
