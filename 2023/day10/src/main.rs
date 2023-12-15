use core::ops::Range;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

// up, down, left, right
// (0,-1), (0,1), (-1,0), (1,0)
const UP: Pt = Pt { x: 0, y: -1 };
const DOWN: Pt = Pt { x: 0, y: 1 };
const LEFT: Pt = Pt { x: -1, y: 0 };
const RIGHT: Pt = Pt { x: 1, y: 0 };

fn find_pipe(grid: &HashMap<Pt, char>, start: Pt) -> (u32, HashMap<Pt, bool>) {
    let start_next = get_next_steps(grid, start);
    let mut ptr_1 = start_next[0];
    let mut ptr_2 = start_next[1];
    let mut seen: HashMap<Pt, bool> = HashMap::new();
    seen.insert(start, true);
    let mut steps: u32 = 1;
    loop {
        if seen.contains_key(&ptr_1) || seen.contains_key(&ptr_2) {
            return (steps, seen);
        }
        if ptr_1 == ptr_2 {
            // Collision
            return (steps, seen);
        }
        let next_1 = get_next_steps(grid, ptr_1);
        if next_1.len() == 0 {
            return (steps, seen);
        }
        for p in next_1 {
            if !seen.contains_key(&p) {
                seen.insert(ptr_1, true);
                ptr_1 = p;
                break;
            }
        }
        if ptr_1 == ptr_2 {
            // Collision
            return (steps, seen);
        }
        let next_2 = get_next_steps(grid, ptr_2);
        if next_2.len() == 0 {
            return (steps, seen);
        }
        for p in next_2 {
            if !seen.contains_key(&p) {
                seen.insert(ptr_2, true);
                ptr_2 = p;
                break;
            }
        }
        steps += 1;
    }
}
fn get_next_steps(grid: &HashMap<Pt, char>, start: Pt) -> Vec<Pt> {
    let mut next: Vec<Pt> = Vec::new();
    // Up
    let up_p = Pt {
        x: start.x,
        y: start.y - 1,
    };
    let left_p = Pt {
        x: start.x - 1,
        y: start.y,
    };
    let right_p = Pt {
        x: start.x + 1,
        y: start.y,
    };
    let down_p = Pt {
        x: start.x,
        y: start.y + 1,
    };
    let cur_c = *grid.get(&start).unwrap();
    if grid.contains_key(&up_p) && can_move(cur_c, *grid.get(&up_p).unwrap(), UP) {
        next.push(up_p);
    }
    if grid.contains_key(&left_p) && can_move(cur_c, *grid.get(&left_p).unwrap(), LEFT) {
        next.push(left_p);
    }
    if grid.contains_key(&right_p) && can_move(cur_c, *grid.get(&right_p).unwrap(), RIGHT) {
        next.push(right_p);
    }
    if grid.contains_key(&down_p) && can_move(cur_c, *grid.get(&down_p).unwrap(), DOWN) {
        next.push(down_p);
    }
    return next;
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

// c is the next character if moved in the direction d
fn can_move(cur: char, next: char, d: Pt) -> bool {
    if d == UP
        && (cur == 'S' || cur == '|' || cur == 'J' || cur == 'L')
        && (next == '|' || next == '7' || next == 'F')
    {
        return true;
    }
    if d == DOWN
        && (cur == 'S' || cur == '|' || cur == '7' || cur == 'F')
        && (next == '|' || next == 'J' || next == 'L')
    {
        return true;
    }
    if d == RIGHT
        && (cur == 'S' || cur == '-' || cur == 'L' || cur == 'F')
        && (next == '-' || next == '7' || next == 'J')
    {
        return true;
    }
    if d == LEFT
        && (cur == 'S' || cur == '-' || cur == 'J' || cur == '7')
        && (next == '-' || next == 'L' || next == 'F')
    {
        return true;
    }
    return false;
}

fn get_x_line_crosses(
    p: &Pt,
    grid: &HashMap<Pt, char>,
    pipe: &HashMap<Pt, bool>,
    range: Range<i32>,
) -> i32 {
    let mut line_crosses: i32 = 0;
    for x in range {
        let new_p = Pt { x: x, y: p.y };
        if pipe.contains_key(&new_p) && *pipe.get(&new_p).unwrap() {
            let c = *grid.get(&new_p).unwrap();
            if c == '|' || c == 'L' || c == 'J' {
                line_crosses += 1;
            }
        }
    }
    return line_crosses;
}

// Using ray casting to determine if a point is inside the polygonial pipe shape or not
fn find_enclosed_area(grid: &HashMap<Pt, char>, pipe: &HashMap<Pt, bool>, max_x: i32) -> usize {
    let mut enclosed_points: Vec<&Pt> = Vec::new();
    for (p, _c) in grid.into_iter() {
        let is_pipe = pipe.contains_key(&p) && *pipe.get(&p).unwrap();
        if !is_pipe {
            if get_x_line_crosses(p, grid, pipe, p.x + 1..max_x) % 2 == 1
                && get_x_line_crosses(p, grid, pipe, 0..p.x) % 2 == 1
            {
                enclosed_points.push(p);
                continue;
            }
        }
    }
    return enclosed_points.len();
}

struct Pos {
    pre: Vec<char>,
    post: Vec<char>,
    res: char,
}

fn fill_start(grid: &HashMap<Pt, char>, start: Pt) -> HashMap<Pt, char> {
    let mut new_grid: HashMap<Pt, char> = grid.clone();
    let start_next = get_next_steps(grid, start);
    let post = *grid.get(&start_next[0]).unwrap();
    let pre = *grid.get(&start_next[1]).unwrap();
    let pos: Vec<Pos> = vec![
        Pos {
            pre: vec!['|', 'L', 'J'],
            post: vec!['|', 'F', '7'],
            res: '|',
        },
        Pos {
            pre: vec!['-', 'L', 'F'],
            post: vec!['-', 'J', '7'],
            res: '-',
        },
        Pos {
            pre: vec!['|', 'L', 'J'],
            post: vec!['-', 'J', '7'],
            res: 'F',
        },
        Pos {
            pre: vec!['-', '7', 'F'],
            post: vec!['|', 'J', '7'],
            res: 'L',
        },
        Pos {
            pre: vec!['-', '7', 'F'],
            post: vec!['|', 'F', 'L'],
            res: 'J',
        },
        Pos {
            pre: vec!['-', 'F', 'L'],
            post: vec!['|', 'J', 'L'],
            res: '7',
        },
    ];
    let mut start_val = 'a';
    for p in pos {
        if p.pre.contains(&pre) && p.post.contains(&post) {
            start_val = p.res;
        }
    }
    if start_val == 'a' {
        panic!("Didnt find start!");
    }
    new_grid.insert(start, start_val);
    return new_grid;
}

fn main() {
    let mut now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut start = Pt { x: 0, y: 0 };
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    for (_y, line) in read_to_string("./input.txt").unwrap().lines().enumerate() {
        let l: Vec<char> = line.chars().collect();
        for (_x, c) in l.iter().enumerate() {
            let x = _x as i32;
            let y = _y as i32;
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            grid.insert(Pt { x: x, y: y }, *c);
            if *c == 'S' {
                start = Pt { x: x, y: y };
            }
        }
    }
    // Part 1
    let (furthest, pipe) = find_pipe(&grid, start);
    println!("Part 1: {}", furthest);
    let mut elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    // Part 2
    now = Instant::now();
    let new_grid = fill_start(&grid, start);
    println!("Part 2: {}", find_enclosed_area(&new_grid, &pipe, max_x));
    elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
