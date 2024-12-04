use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

const N: Pt = Pt { x: 0, y: -1 };
const NE: Pt = Pt { x: 1, y: -1 };
const NW: Pt = Pt { x: -1, y: -1 };
const S: Pt = Pt { x: 0, y: 1 };
const SE: Pt = Pt { x: 1, y: 1 };
const SW: Pt = Pt { x: -1, y: 1 };
const W: Pt = Pt { x: -1, y: 0 };
const E: Pt = Pt { x: 1, y: 0 };

fn has_xmas(grid: &HashMap<Pt, char>, pt: &Pt, dir: Pt) -> bool {
    let xmas = vec!['X', 'M', 'A', 'S'];

    for i in 0..xmas.len() {
        let new_pt = Pt {
            x: pt.x + dir.x * i as i32,
            y: pt.y + dir.y * i as i32,
        };
        // Out of bounds
        if !grid.contains_key(&new_pt) {
            return false;
        }
        let c = *grid.get(&new_pt).unwrap();
        if c != xmas[i] {
            return false;
        }
    }
    true
}

fn count_from_x(grid: &HashMap<Pt, char>, pt: &Pt) -> i32 {
    let mut count = 0;
    let dirs = vec![N, NE, NW, S, SE, SW, W, E];

    for dir in dirs {
        if has_xmas(grid, pt, dir) {
            count += 1;
        }
    }

    count
}

fn count_xmas(grid: &HashMap<Pt, char>) -> i32 {
    let mut count = 0;
    // Find Xs
    for (p, c) in grid {
        if *c == 'X' {
            count += count_from_x(&grid, &p)
        }
    }
    count
}

fn count_mas(grid: &HashMap<Pt, char>, pt: &Pt) -> i32 {
    // Has to be diagonals;
    let corners = [
        Pt {
            x: pt.x - 1,
            y: pt.y - 1,
        },
        Pt {
            x: pt.x + 1,
            y: pt.y - 1,
        },
        Pt {
            x: pt.x - 1,
            y: pt.y + 1,
        },
        Pt {
            x: pt.x + 1,
            y: pt.y + 1,
        },
    ];

    // Validate we're within bounds;
    for c in corners {
        if !grid.contains_key(&c) {
            return 0;
        }
    }

    let left = vec![
        *grid.get(&corners[0]).unwrap(),
        *grid.get(&corners[3]).unwrap(),
    ];
    let right = vec![
        *grid.get(&corners[1]).unwrap(),
        *grid.get(&corners[2]).unwrap(),
    ];
    let mut has_left = false;
    let mut has_right = false;
    if (left[0] == 'M' && left[1] == 'S') || (left[0] == 'S' && left[1] == 'M') {
        has_left = true;
    }
    if (right[0] == 'M' && right[1] == 'S') || (right[0] == 'S' && right[1] == 'M') {
        has_right = true;
    }

    if has_right && has_left {
        return 1;
    }

    0
}

fn count_masx(grid: &HashMap<Pt, char>) -> i32 {
    let mut count = 0;
    // Find Xs
    for (p, c) in grid {
        if *c == 'A' {
            count += count_mas(&grid, &p);
        }
    }
    count
}
fn main() {
    let mut now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut y = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for c in l {
            grid.insert(Pt { x, y }, c);
            x += 1;
        }
        y += 1;
    }
    // Part 1
    println!("Part 1: {}", count_xmas(&grid));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", count_masx(&grid));
    println!("Done in: {:?}!", now.elapsed());
}
