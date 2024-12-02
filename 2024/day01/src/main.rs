use std::fs::read_to_string;
use std::time::Instant;

fn total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut sum = 0;
    left.sort();
    right.sort();
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    sum
}

fn main() {
    let mut now = Instant::now();
    // Part 1
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in read_to_string("./src/part1.txt").unwrap().lines() {
        let splits: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        left.push(splits[0].parse::<i32>().unwrap());
        right.push(splits[splits.len() - 1].parse::<i32>().unwrap());
    }
    println!("Part 1: {}", total_distance(left, right));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
}
