use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

fn is_game_valid(sets: Vec<CubeSet>, max: CubeSet) -> bool {
    for set in sets {
        if set.red > max.red || set.green > max.green || set.blue > max.blue {
            return false;
        }
    }
    return true;
}

fn least_cubes(sets: Vec<CubeSet>) -> CubeSet {
    let mut c = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for set in sets {
        if set.red > c.red {
            c.red = set.red;
        }
        if set.blue > c.blue {
            c.blue = set.blue;
        }
        if set.green > c.green {
            c.green = set.green;
        }
    }
    return c;
}

fn parse_sets(line: &str) -> Vec<CubeSet> {
    let re: Regex = Regex::new(r"([0-9]+) (red|blue|green)").unwrap();
    let mut cubes: Vec<CubeSet> = Vec::new();
    for sets in line.split(';') {
        let mut cube = CubeSet {
            red: 0,
            blue: 0,
            green: 0,
        };
        for set in sets.split(',') {
            let cap = re.captures(set).unwrap();
            let count: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let color = cap.get(2).unwrap().as_str();
            if color == "red" {
                cube.red = count;
            }
            if color == "green" {
                cube.green = count;
            }
            if color == "blue" {
                cube.blue = count;
            }
        }
        cubes.push(cube);
    }
    return cubes;
}

fn main() {
    // Part 1
    let mut sum = 0;
    let id_re: Regex = Regex::new(r"^Game ([0-9]+): (.*)$").unwrap();
    for line in read_to_string("./part1.txt").unwrap().lines() {
        let cap = id_re.captures(line).unwrap();
        let id: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let sets = cap.get(2).unwrap().as_str();
        let cube_sets = parse_sets(&sets);
        let max = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let is_valid = is_game_valid(cube_sets, max);
        if is_valid {
            sum += id;
        }
    }
    println!("Part 1: {0}", sum);
    sum = 0;
    // Part 2
    for line in read_to_string("./part2.txt").unwrap().lines() {
        let cap = id_re.captures(line).unwrap();
        let id: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let sets = cap.get(2).unwrap().as_str();
        let cube_sets = parse_sets(&sets);
        let c = least_cubes(cube_sets);
        sum += c.red * c.green * c.blue;
    }
    println!("Part 2: {0}", sum);
}
