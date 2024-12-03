use std::fs::read_to_string;
use std::time::Instant;

fn is_safe(levels: Vec<i32>) -> bool {
    let direction = if levels[0] > levels[1] { -1 } else { 1 };
    for i in 1..levels.len() {
        let prev = levels[i - 1];
        let cur = levels[i];
        if prev == cur {
            return false;
        };
        let diff = cur - prev;
        if diff * direction > 3 || diff * direction < 1 {
            return false;
        };
    }
    true
}

fn count_safe(reports: Vec<Vec<i32>>, dampener_enabled: bool) -> i32 {
    let mut count = 0;
    for levels in reports {
        if is_safe(levels.clone()) {
            count += 1;
        } else if dampener_enabled {
            for i in 0..levels.len() - 1 {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe(new_levels) {
                    count += 1;
                    continue;
                }
            }
        }
    }
    count
}

fn main() {
    let mut now = Instant::now();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let levels: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    // Part 1
    println!("Part 1: {}", count_safe(reports.clone(), false));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", count_safe(reports, true));
    println!("Done in: {:?}!", now.elapsed());
}
