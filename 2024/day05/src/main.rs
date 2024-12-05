use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn sum_valid_updates(updates: &Vec<Vec<&str>>, rules: &HashMap<&str, bool>) -> i32 {
    let mut sum: i32 = 0;

    for update in updates {
        let mut is_valid = true;
        for (i, first) in update.iter().enumerate() {
            for j in i + 1..update.len() {
                let second = &update[j];
                let key = vec![second.to_string(), first.to_string()].join("|");
                if rules.contains_key(key.as_str()) {
                    is_valid = false;
                }
            }
        }
        if is_valid {
            let mid = update.len() / 2;
            if update.len() % 2 == 0 {
                panic!("Even number of pages!")
            }
            sum += update[mid].parse::<i32>().unwrap();
        }
    }

    sum
}

fn main() {
    let mut now = Instant::now();
    let mut rules: HashMap<&str, bool> = HashMap::new();
    let mut updates: Vec<Vec<&str>> = Vec::new();
    let binding = read_to_string("./src/input.txt").unwrap();
    for line in binding.lines() {
        if line.contains('|') {
            rules.insert(line, true);
        }
        if line.contains(',') {
            updates.push(line.split(',').collect());
        }
    }
    // Part 1
    println!("Part 1: {}", sum_valid_updates(&updates, &rules));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
