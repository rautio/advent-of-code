use std::fs::read_to_string;

const DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit(line: Vec<char>) -> char {
    for (_i, c) in line.into_iter().enumerate() {
        if c.is_digit(10) {
            return c;
        }
    }
    return '0';
}

fn find_forward_digit(line: &Vec<char>) -> char {
    for (i, c) in line.into_iter().enumerate() {
        if c.is_digit(10) {
            return *c;
        } else {
            let mut num = 1;
            for str in DIGITS {
                if i + str.len() < line.len() {
                    let line_str = &line[i..i + str.len()];
                    let mut i: usize = 0;
                    for c in line_str {
                        if *c != str.as_bytes()[i] as char {
                            break;
                        }
                        i += 1;
                    }
                    if i == line_str.len() {
                        return char::from_digit(num, 10).unwrap();
                    }
                }
                num += 1;
            }
        }
    }
    return '0';
}

fn find_backward_digit(reversed_line: &Vec<char>) -> char {
    for (i, c) in reversed_line.into_iter().enumerate() {
        if c.is_digit(10) {
            return *c;
        } else {
            let mut num = 1;
            for str in DIGITS {
                if i + str.len() < reversed_line.len() {
                    let line_str = &reversed_line[i..i + str.len()];
                    let mut i: usize = 0;
                    for c in line_str.into_iter().rev() {
                        if *c != str.as_bytes()[i] as char {
                            break;
                        }
                        i += 1;
                    }
                    if i == line_str.len() {
                        return char::from_digit(num, 10).unwrap();
                    }
                }
                num += 1;
            }
        }
    }
    return '0';
}

fn main() {
    // Part 1
    let mut sum = 0;
    for line in read_to_string("./src/part1.txt").unwrap().lines() {
        let mut str = String::new();
        str.push(find_digit(line.chars().collect()));
        str.push(find_digit(line.chars().rev().collect()));
        sum += str.parse::<i32>().unwrap();
    }
    println!("Part 1: {0}", sum);
    // Part 2
    sum = 0;
    for line in read_to_string("./src/part2.txt").unwrap().lines() {
        let mut str = String::new();
        str.push(find_forward_digit(&line.chars().collect()));
        str.push(find_backward_digit(&line.chars().rev().collect()));
        sum += str.parse::<i32>().unwrap();
    }
    println!("Part 2: {0}", sum);
}
