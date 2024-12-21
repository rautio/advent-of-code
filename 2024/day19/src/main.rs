use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn count_possible(
    towel: &str,
    patterns: &Vec<&str>,
    counter: i64,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if memo.contains_key(towel) {
        return *memo.get(towel).unwrap();
    }
    if towel.len() == 0 {
        return counter + 1;
    }
    let mut next: Vec<String> = Vec::new();
    for p in patterns {
        if towel.starts_with(p) {
            let mut new_towel = towel.to_string();
            new_towel.replace_range(0..p.len(), "");
            next.push(new_towel);
        }
    }
    if next.len() == 0 {
        memo.insert(towel.to_string(), 0);
        return 0;
    }
    let mut res = counter;
    for n in next {
        let c = count_possible(&n, patterns, counter, memo);
        res += c;
        memo.insert(n, c);
    }

    memo.insert(towel.to_string(), res);
    return res;
}

fn count_options(towels: &Vec<&str>, patterns: &Vec<&str>) -> i64 {
    let mut total = 0;
    let mut memo: HashMap<String, i64> = HashMap::new();

    for t in towels {
        let options = count_possible(t, patterns, 0, &mut memo);
        total += options;
    }
    total
}

fn is_possible(towel: &str, patterns: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
    if memo.contains_key(towel) {
        return *memo.get(towel).unwrap();
    }
    if towel.len() == 0 {
        return true;
    }
    let mut next: Vec<String> = Vec::new();
    for p in patterns {
        if towel.starts_with(p) {
            let mut new_towel = towel.to_string();
            new_towel.replace_range(0..p.len(), "");
            next.push(new_towel);
        }
    }
    if next.len() == 0 {
        memo.insert(towel.to_string(), false);
        return false;
    }
    for n in next {
        if is_possible(&n, patterns, memo) {
            memo.insert(n, true);
            return true;
        } else {
            memo.insert(n, false);
        }
    }

    memo.insert(towel.to_string(), false);
    return false;
}

fn filter_possible<'a>(towels: &'a Vec<&'a str>, patterns: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut possible: Vec<&str> = Vec::new();
    let mut memo: HashMap<String, bool> = HashMap::new();
    for t in towels {
        if is_possible(t, patterns, &mut memo) {
            possible.push(t);
        }
    }

    possible
}

fn main() {
    let mut now = Instant::now();
    let mut patterns: Vec<&str> = Vec::new();
    let mut towels: Vec<&str> = Vec::new();
    let binding = read_to_string("./src/input.txt").unwrap();
    for (i, line) in binding.lines().into_iter().enumerate() {
        if i == 0 {
            patterns = line.split(", ").collect();
        } else if line != "" {
            towels.push(line)
        }
    }
    println!("Part 1: {}", filter_possible(&towels, &patterns).len());
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {}", count_options(&towels, &patterns));
    println!("Done in: {:?}!", now.elapsed());
}
