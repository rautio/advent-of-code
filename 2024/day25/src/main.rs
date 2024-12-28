use std::fs::read_to_string;
use std::time::Instant;

fn convert(cur: &Vec<Vec<char>>) -> (Vec<i32>, bool) {
    let mut res: Vec<i32> = vec![0, 0, 0, 0, 0];
    let mut is_lock = true;
    for c in cur[0].clone() {
        if c == '.' {
            is_lock = false;
        }
    }
    let mut cur_i = cur.clone();
    if !is_lock {
        cur_i = cur_i.clone().into_iter().rev().collect();
    }
    cur_i.remove(0);
    for row in cur_i {
        for c in 0..row.len() {
            let col = row[c];
            if col == '#' {
                res[c] += 1;
            }
        }
    }
    (res, is_lock)
}

fn solve_keys(locks: &Vec<Vec<i32>>, keys: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let max = 5;

    for l in locks {
        for k in keys {
            let mut m = true;
            for i in 0..5 {
                if l[i] + k[i] > max {
                    m = false;
                }
            }
            if m {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut now = Instant::now();
    let mut locks: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut cur: Vec<Vec<char>> = Vec::new();
    let mut objects: Vec<Vec<Vec<char>>> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 0 {
            objects.push(cur.clone());
            // Reset!
            cur = Vec::new();
        } else {
            cur.push(chars);
        }
    }
    objects.push(cur.clone());
    for o in &objects {
        let (cur, is_lock) = convert(o);
        if is_lock {
            locks.push(cur);
        } else {
            keys.push(cur);
        }
    }
    // Part 1
    println!("Part 1: {}", solve_keys(&locks, &keys));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {}
}
