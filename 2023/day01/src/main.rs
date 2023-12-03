use std::fs::read_to_string;

fn find_digit(line: Vec<char>) -> char {
    for (_i, c) in line.into_iter().enumerate() {
        // do something with character `c` and index `i`
        if c.is_digit(10) {
            return c;
        }
    }
    return '0';
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("./src/part1.txt").unwrap().lines() {
        let mut str = String::new();
        str.push(find_digit(line.chars().collect()));
        str.push(find_digit(line.chars().rev().collect()));
        sum += str.parse::<i32>().unwrap();
    }
    println!("Part 1: {0}", sum);
}
