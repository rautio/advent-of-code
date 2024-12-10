use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

const N: Pt = Pt { x: 0, y: -1 };
const E: Pt = Pt { x: 1, y: 0 };
const S: Pt = Pt { x: 0, y: 1 };
const W: Pt = Pt { x: -1, y: 0 };

fn add(p1: &Pt, p2: &Pt) -> Pt {
    Pt {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn score_trailhead(start: Pt, grid: &HashMap<Pt, i32>) -> (i32, i32) {
    let mut trail: VecDeque<Pt> = VecDeque::new();
    let mut rating = 0;
    trail.push_back(start);
    let mut ends: HashMap<Pt, bool> = HashMap::new();
    while trail.len() > 0 {
        let cur = trail.pop_front().unwrap();
        if *grid.get(&cur).unwrap() == 9 {
            ends.insert(cur, true);
            rating += 1;
        } else {
            let next: Vec<Pt> = vec![N, W, S, E]
                .into_iter()
                .map(|p| add(&p, &cur))
                .filter(|p| grid.contains_key(&p))
                .filter(|p| *grid.get(&p).unwrap() == *grid.get(&cur).unwrap() + 1)
                .collect();
            for n in next {
                trail.push_back(n);
            }
        }
    }
    (ends.keys().len() as i32, rating)
}

fn sum_trailheads(grid: &HashMap<Pt, i32>, trailheads: &Vec<Pt>) -> (i32, i32) {
    let mut sum_score = 0;
    let mut sum_rating = 0;

    for trailhead in trailheads {
        let (score, rating) = score_trailhead(*trailhead, grid);
        sum_score += score;
        sum_rating += rating;
    }

    (sum_score, sum_rating)
}
fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, i32> = HashMap::new();
    let mut trailheads: Vec<Pt> = Vec::new();
    let mut y = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for ch in l {
            let c = ch.to_string().parse::<i32>().unwrap();
            if c == 0 {
                trailheads.push(Pt { x, y });
            }
            grid.insert(Pt { x, y }, c);
            x += 1;
        }
        y += 1;
    }
    let (score, rating) = sum_trailheads(&grid, &trailheads);
    println!("Part 1: {}", score);
    println!("Part 2: {}", rating);
    println!("Done in: {:?}!", now.elapsed());
}
