use std::collections::HashMap;
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

fn count_path(grid: &HashMap<Pt, char>, start: &Pt) -> usize {
    let dirs = vec![N, E, S, W];
    let mut visited: HashMap<Pt, bool> = HashMap::new();
    let mut dir_idx = 0; // Always start moving up
    let mut cur = start.clone();

    while grid.contains_key(&cur) {
        visited.insert(cur, true);
        let dir = dirs[dir_idx % dirs.len()];
        let next = Pt {
            x: cur.x + dir.x,
            y: cur.y + dir.y,
        };
        if grid.contains_key(&next) && *grid.get(&next).unwrap() == '#' {
            // Change direction
            dir_idx += 1;
        } else {
            cur = next;
        }
    }

    visited.keys().len()
}

fn main() {
    let mut now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut start = Pt { x: 0, y: 0 };
    let mut y = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for c in l {
            if c == '^' {
                start = Pt { x, y };
                grid.insert(Pt { x, y }, '.');
            } else {
                grid.insert(Pt { x, y }, c);
            }
            x += 1;
        }
        y += 1;
    }
    // Part 1
    println!("Part 1: {}", count_path(&grid, &start));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
