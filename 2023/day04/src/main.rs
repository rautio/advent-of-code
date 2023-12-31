use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn count_wins(winning: Vec<u32>, mine: Vec<u32>) -> u32 {
    let mut wins = 0;
    for w in winning {
        if mine.contains(&w) {
            wins += 1;
        }
    }
    let base: i32 = 2;
    if wins > 0 {
        return base.pow(wins - 1) as u32;
    }
    return 0;
}

fn solve_part1(input_file: &str) -> u32 {
    let mut sum = 0;
    let re: Regex = Regex::new(r"^Card[ \t]+([0-9]+): (.*)$").unwrap();
    for line in read_to_string(input_file).unwrap().lines() {
        let cap = re.captures(line).unwrap();
        let nums = cap.get(2).unwrap().as_str();
        let splits: Vec<&str> = nums.split("|").collect();
        let winning: Vec<u32> = splits[0]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .filter(|x| *x != "")
            .map(|x| x.parse().unwrap())
            .collect();
        let mine: Vec<u32> = splits[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .filter(|x| *x != "")
            .map(|x| x.parse().unwrap())
            .collect();
        sum += count_wins(winning, mine);
    }
    return sum;
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    // println!("Part 2: {0}", sum);
}
