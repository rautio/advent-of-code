use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn sum_valid_updates(updates: &Vec<Vec<&str>>, rules: &HashMap<&str, bool>) -> (i32, i32) {
    let mut valid_sum: i32 = 0;
    let mut corrected_sum: i32 = 0;

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
            valid_sum += update[mid].parse::<i32>().unwrap();
        } else {
            // Invalid
            let mut corrected: Vec<&str> = vec![update[0]];
            for i in 1..update.len() {
                let new_page = update[i];
                for (j, page) in corrected.clone().iter().enumerate() {
                    let key = vec![new_page.to_string(), page.to_string()].join("|");
                    if rules.contains_key(key.as_str()) {
                        corrected.insert(j, new_page);
                        break;
                    } else if j == corrected.len() - 1 {
                        corrected.push(new_page);
                    }
                }
            }
            let mid = corrected.len() / 2;
            if corrected.len() % 2 == 0 {
                panic!("Even number of pages!")
            }
            corrected_sum += corrected[mid].parse::<i32>().unwrap();
        }
    }

    (valid_sum, corrected_sum)
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
    let (valid_sum, corrected_sum) = sum_valid_updates(&updates, &rules);
    // Part 1
    println!("Part 1: {}", valid_sum);
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", corrected_sum);
    println!("Done in: {:?}!", now.elapsed());
}
