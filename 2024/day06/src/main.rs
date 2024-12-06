use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Dir {
    pt: Pt,
    dir: Pt,
}

const N: Pt = Pt { x: 0, y: -1 };
const E: Pt = Pt { x: 1, y: 0 };
const S: Pt = Pt { x: 0, y: 1 };
const W: Pt = Pt { x: -1, y: 0 };

fn is_loop(grid: &HashMap<Pt, char>, start: &Pt) -> bool {
    let dirs = vec![N, E, S, W];
    let mut dir_idx = 0; // Always start moving up
    let mut cur = start.clone();
    let mut visited: HashMap<Dir, bool> = HashMap::new();

    while grid.contains_key(&cur) {
        let dir = dirs[dir_idx % dirs.len()];
        visited.insert(Dir { pt: cur, dir }, true);
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
        if visited.contains_key(&Dir {
            pt: cur,
            dir: dirs[dir_idx % dirs.len()],
        }) {
            return true;
        }
    }

    false
}

fn count_possible_loops(grid: &HashMap<Pt, char>, visited: &HashMap<Pt, bool>, start: &Pt) -> i32 {
    let mut count = 0;

    for pt in visited.keys() {
        if pt != start {
            let mut grid_copy = grid.clone();
            grid_copy.insert(*pt, '#');
            if is_loop(&grid_copy, start) {
                count += 1;
            }
        }
    }

    count
}

fn find_path(grid: &HashMap<Pt, char>, start: &Pt) -> HashMap<Pt, bool> {
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

    visited
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
    let path = find_path(&grid, &start);
    println!("Part 1: {}", path.keys().len());
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", count_possible_loops(&grid, &path, &start));
    println!("Done in: {:?}!", now.elapsed());
}
