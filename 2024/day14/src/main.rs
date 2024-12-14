use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    pub fn new(x: i32, y: i32) -> Self {
        Pt { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Robot {
    pos: Pt,
    vel: Pt,
}

fn mul(p: Pt, con: i32) -> Pt {
    Pt {
        x: p.x * con,
        y: p.y * con,
    }
}

fn add(p1: Pt, p2: Pt) -> Pt {
    Pt {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn move_robots(robots: Vec<Robot>, time: i32, board_max: Pt) -> Vec<Robot> {
    let mut moved: Vec<Robot> = Vec::new();
    for robot in robots {
        let mut new_pos = add(robot.pos, mul(robot.vel, time));
        new_pos.x = new_pos.x % board_max.x;
        new_pos.y = new_pos.y % board_max.y;
        if new_pos.x < 0 {
            new_pos.x = board_max.x + new_pos.x;
        }
        if new_pos.y < 0 {
            new_pos.y = board_max.y + new_pos.y;
        }
        moved.push(Robot {
            pos: new_pos,
            vel: robot.vel,
        });
    }

    moved
}

fn count_robots(robots: &Vec<Robot>, range: (Pt, Pt)) -> i32 {
    let mut count = 0;
    for robot in robots {
        let pos = robot.pos;
        if pos.x >= range.0.x && pos.y >= range.0.y && pos.x <= range.1.x && pos.y <= range.1.y {
            count += 1;
        }
    }
    count
}
fn safety_factor(robots: &Vec<Robot>, board_max: Pt) -> i32 {
    let mid_x = board_max.x / 2;
    let mid_y = board_max.y / 2;
    let q = vec![
        (Pt::new(0, 0), Pt::new(mid_x - 1, mid_y - 1)),
        (Pt::new(mid_x + 1, 0), Pt::new(board_max.x, mid_y - 1)),
        (Pt::new(0, mid_y + 1), Pt::new(mid_x - 1, board_max.y)),
        (
            Pt::new(mid_x + 1, mid_y + 1),
            Pt::new(board_max.x, board_max.y),
        ),
    ];

    count_robots(robots, q[0])
        * count_robots(robots, q[1])
        * count_robots(robots, q[2])
        * count_robots(robots, q[3])
}

fn print_robots(robots: &Vec<Robot>, board_max: Pt) {
    let mut map: HashMap<Pt, usize> = HashMap::new();
    for r in robots {
        *map.entry(r.pos).or_insert(0) += 1;
    }
    for y in 0..board_max.y {
        let mut line = String::new();
        for x in 0..board_max.x {
            let p: Pt = Pt { x, y };
            if map.contains_key(&p) {
                line.push_str(&map.get(&p).unwrap().to_string());
            } else {
                line.push_str(".");
            }
        }
        println!("{}", line);
    }
}

fn find_tree(robots: &Vec<Robot>, board_max: Pt, max_sec: i32) -> i32 {
    // lowest safety score is likely the tree.
    let mut safety = 0;
    let mut min_sec = 0;
    for s in 0..max_sec {
        let safe = safety_factor(&move_robots(robots.clone(), s, board_max), board_max);
        if safety == 0 || safe < safety {
            min_sec = s;
            safety = safe;
        }
    }
    min_sec
}

fn main() {
    lazy_static! {
        static ref re: Regex = Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();
    }
    let mut now = Instant::now();
    let mut robots: Vec<Robot> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let cap = re.captures(line).unwrap();
        robots.push(Robot {
            pos: Pt {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            },
            vel: Pt {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            },
        });
    }
    // Part 1
    // let board_max = Pt { x: 11, y: 7 };
    let board_max = Pt { x: 101, y: 103 };
    let new_robots = move_robots(robots.clone(), 100, board_max);
    // print_robots(&new_robots, board_max);
    println!("Part 1: {}", safety_factor(&new_robots, board_max));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", find_tree(&robots, board_max, 10000));
    println!("Done in: {:?}!", now.elapsed());
}
