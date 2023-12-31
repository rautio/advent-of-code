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
    time: u64,
    distance: u64,
}

// Hold or Release

fn count_options(time_remaining: u64, distance_to_beat: u64, speed: u64) -> u64 {
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

fn solve_part1(input_file: &str) -> u64 {
    let mut races: Vec<Race> = Vec::new();
    let lines: Vec<String> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let times: Vec<u64> = RE
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
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = RE
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
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    for (i, t) in times.into_iter().enumerate() {
        races.push(Race {
            time: t,
            distance: distances[i],
        });
    }
    let mut res = 1;
    for r in races {
        res *= count_options(r.time, r.distance, 0);
    }
    return res;
}

fn solve_part2(input_file: &str) -> u64 {
    let mut race = Race {
        time: 0,
        distance: 0,
    };
    let lines: Vec<String> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut time: String = RE
        .captures(&lines[0])
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .to_string();
    time.retain(|c| !c.is_whitespace());
    race.time = time.parse().unwrap();
    // .parse()
    // .unwrap();
    println!("time: {:?}", time);
    let mut dist: String = RE
        .captures(&lines[1])
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .to_string();
    dist.retain(|c| !c.is_whitespace());
    println!("dist: {:?}", dist);
    race.distance = dist.parse::<u64>().unwrap();
    let mut res = 1;
    println!("race: {:?}", race);
    // for r in races {
    //     res *= count_options(r.time, r.distance, 0);
    // }
    return count_options(race.time, race.distance, 0);
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {0}", solve_part2("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
}
