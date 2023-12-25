use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Val {
    Rock,
    Garden,
    Start,
}

// up, down, left, right
// (0,-1), (0,1), (-1,0), (1,0)
const UP: Pt = Pt { x: 0, y: -1 };
const DOWN: Pt = Pt { x: 0, y: 1 };
const LEFT: Pt = Pt { x: -1, y: 0 };
const RIGHT: Pt = Pt { x: 1, y: 0 };

fn parse_grid(input: &str) -> (HashMap<Pt, Val>, Pt) {
    let mut grid: HashMap<Pt, Val> = HashMap::new();
    let mut start = Pt { x: -1, y: -1 };

    for (_y, line) in read_to_string(input)
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
    {
        let y = _y as i32;
        for (_x, c) in line.chars().into_iter().enumerate() {
            let x = _x as i32;
            let mut v = Val::Rock;
            if c == '.' {
                v = Val::Garden;
            }
            if c == 'S' {
                v = Val::Start;
                start = Pt { x: x, y: y };
            }
            grid.insert(Pt { x: x, y: y }, v);
        }
    }
    return (grid, start);
}

fn step(grid: &HashMap<Pt, Val>, start: &Pt, steps: usize) -> Vec<Pt> {
    let mut positions: VecDeque<Pt> = VecDeque::new();
    let possible: Vec<Pt> = vec![UP, DOWN, LEFT, RIGHT];
    positions.push_back(*start);
    for _s in 0..steps {
        // Remove duplicate end positions
        let mut p_vec = Vec::from(positions);
        p_vec.sort_by(|a, b| a.x.cmp(&b.x));
        p_vec.sort_by(|a, b| a.y.cmp(&b.y));
        p_vec.dedup();
        positions = VecDeque::from(p_vec);
        let len = positions.len();
        for _i in 0..len {
            let p = positions.pop_front().unwrap();
            for dir in &possible {
                let new_pt = Pt {
                    x: p.x + dir.x,
                    y: p.y + dir.y,
                };
                if grid.contains_key(&new_pt) {
                    let val = *grid.get(&new_pt).unwrap();
                    if val == Val::Garden || val == Val::Start {
                        positions.push_back(new_pt);
                    }
                }
            }
        }
    }
    let mut res = Vec::from(positions);
    // Remove duplicate end positions
    res.sort_by(|a, b| a.x.cmp(&b.x));
    res.sort_by(|a, b| a.y.cmp(&b.y));
    res.dedup();
    return res;
}

fn main() {
    let mut now = Instant::now();
    let (grid, start) = parse_grid("./input.txt");
    println!("Part 1: {}", step(&grid, &start, 64).len());
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    now = Instant::now();
    // println!("Part 2: {}", weigh_load(&rotated, max));
    println!("Done in: {:.2?}!", now.elapsed());
}
