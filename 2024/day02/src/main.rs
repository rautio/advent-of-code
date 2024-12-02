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

fn count_safe(reports: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        if is_safe(report) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut now = Instant::now();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string("./src/part1.txt").unwrap().lines() {
        let levels: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        reports.push(levels);
    }
    // Part 1
    println!("Part 1: {}", count_safe(reports));
    println!("Done in: {:?}!", now.elapsed());
    // Part 1
    now = Instant::now();
}
