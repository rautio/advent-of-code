use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn str_sum(s: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }
    let mut sum = 0;
    // iterate over all matches
    for captures in RE.captures_iter(s) {
        sum += &captures[1].parse::<i32>().unwrap() * &captures[2].parse::<i32>().unwrap();
    }
    return sum;
}

fn main() {
    let mut now = Instant::now();
    let mut part1_sum = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        part1_sum += str_sum(line);
    }
    // Part 1
    println!("Part 1: {}", part1_sum);
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
