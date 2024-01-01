use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(Time|Distance): (.*)$").unwrap();
}
#[derive(Debug, Clone)]
struct Race {
    time: u128,
    distance: u128,
}

fn can_win(hold_sec: u128, distance_to_beat: u128, time: u128) -> bool {
    let distance = (time - hold_sec) * hold_sec;
    return distance > distance_to_beat;
}

fn first_win<I>(range: I, distance_to_beat: u128, time: u128) -> u128
where
    I: IntoIterator<Item = u128>,
{
    for hold_sec in range {
        if can_win(hold_sec, distance_to_beat, time) {
            return hold_sec;
        }
    }
    panic!("Not a winning range.");
}

fn count_options(time: u128, distance_to_beat: u128) -> u128 {
    let lower_bound = first_win(0..time, distance_to_beat, time);
    let upper_bound = first_win((0..time).rev(), distance_to_beat, time);
    return upper_bound - lower_bound + 1;
}

fn solve_part1(input_file: &str) -> u128 {
    let mut races: Vec<Race> = Vec::new();
    let lines: Vec<String> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let times: Vec<u128> = RE
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
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    let distances: Vec<u128> = RE
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
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    for (i, t) in times.into_iter().enumerate() {
        races.push(Race {
            time: t,
            distance: distances[i],
        });
    }
    let mut res = 1;
    for r in races {
        res *= count_options(r.time, r.distance);
    }
    return res;
}

fn solve_part2(input_file: &str) -> u128 {
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
    let mut dist: String = RE
        .captures(&lines[1])
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .to_string();
    dist.retain(|c| !c.is_whitespace());
    race.distance = dist.parse::<u128>().unwrap();
    return count_options(race.time, race.distance);
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {0}", solve_part2("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
}
