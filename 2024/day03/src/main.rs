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

fn str_sum_enables(s: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    }
    let mut sum = 0;
    let mut enabled = true;
    // iterate over all matches
    for captures in RE.captures_iter(s) {
        if &captures[0] == "do()" {
            enabled = true;
        } else if &captures[0] == "don't()" {
            enabled = false;
        } else if enabled {
            sum += &captures[2].parse::<i32>().unwrap() * &captures[3].parse::<i32>().unwrap();
        }
    }
    return sum;
}

fn main() {
    let mut now = Instant::now();
    let mut continous_input = String::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        continous_input += line;
    }
    // Part 1
    println!("Part 1: {}", str_sum(&continous_input));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", str_sum_enables(&continous_input));
    println!("Done in: {:?}!", now.elapsed());
}
