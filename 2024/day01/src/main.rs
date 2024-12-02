use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    sum
}

fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut score = 0;
    let mut right_occurences = HashMap::new();
    for i in 0..right.len() {
        *right_occurences.entry(right[i]).or_insert(0) += 1;
    }
    for i in 0..left.len() {
        if right_occurences.contains_key(&left[i]) {
            score += left[i] * right_occurences.get(&left[i]).unwrap();
        }
    }
    score
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
    left.sort();
    right.sort();
    println!("Part 1: {}", total_distance(left.clone(), right.clone()));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {}", similarity_score(left, right));
    println!("Done in: {:?}!", now.elapsed());
}
