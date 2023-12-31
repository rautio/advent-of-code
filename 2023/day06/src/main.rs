use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(Time|Distance): (.*)$").unwrap();
}
#[derive(Debug, Clone)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_input(input_file: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let lines: Vec<String> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let times: Vec<u32> = RE
        .captures(&lines[0])
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u32> = RE
        .captures(&lines[1])
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    for (i, t) in times.into_iter().enumerate() {
        races.push(Race {
            time: t,
            distance: distances[i],
        });
    }
    return races;
}

// Hold or Release

fn count_options(time_remaining: u32, distance_to_beat: u32, speed: u32) -> u32 {
    if time_remaining <= 0 {
        return 0;
    }
    let mut num_options = 0;
    // Can we let go?
    if speed * time_remaining > distance_to_beat {
        num_options += 1;
    }
    num_options += count_options(time_remaining - 1, distance_to_beat, speed + 1);
    return num_options;
}

fn solve_part1(races: &Vec<Race>) -> u32 {
    let mut res = 1;
    for r in races {
        res *= count_options(r.time, r.distance, 0);
    }
    return res;
}

fn main() {
    let mut now = Instant::now();
    let races: Vec<Race> = parse_input("./input.txt");
    println!("races: {:?}", races);
    println!("Part 1: {}", solve_part1(&races));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    // println!("Part 2: {0}", solve_part2("./input.txt"));
    // println!("Done in: {:?}!", now.elapsed());
}
