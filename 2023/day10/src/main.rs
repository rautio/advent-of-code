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

fn find_furthest_steps(grid: HashMap<Pt, char>, start: Pt) -> u32 {
    let start_next = get_next_steps(&grid, start);
    let mut ptr_1 = start_next[0];
    let mut ptr_2 = start_next[1];
    let mut seen_1: HashMap<Pt, bool> = HashMap::new();
    let mut seen_2: HashMap<Pt, bool> = HashMap::new();
    seen_1.insert(start, true);
    seen_2.insert(start, true);
    let mut steps: u32 = 1;
    loop {
        if seen_1.contains_key(&ptr_1) || seen_2.contains_key(&ptr_2) {
            return steps;
        }
        if ptr_1 == ptr_2 {
            // Collision
            return steps;
        }
        let next_1 = get_next_steps(&grid, ptr_1);
        if next_1.len() == 0 {
            return steps;
        }
        for p in next_1 {
            if !seen_1.contains_key(&p) {
                seen_1.insert(ptr_1, true);
                ptr_1 = p;
                break;
            }
        }
        if ptr_1 == ptr_2 {
            // Collision
            return steps;
        }
        let next_2 = get_next_steps(&grid, ptr_2);
        if next_2.len() == 0 {
            return steps;
        }
        for p in next_2 {
            if !seen_2.contains_key(&p) {
                seen_2.insert(ptr_2, true);
                ptr_2 = p;
                break;
            }
        }
        steps += 1;
    }
    return steps;
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

fn main() {
    let mut now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut start = Pt { x: 0, y: 0 };
    for (y, line) in read_to_string("./input.txt").unwrap().lines().enumerate() {
        let l: Vec<char> = line.chars().collect();
        for (x, c) in l.iter().enumerate() {
            grid.insert(
                Pt {
                    x: x as i32,
                    y: y as i32,
                },
                *c,
            );
            if *c == 'S' {
                start = Pt {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    // Part 1
    println!("Part 1: {}", find_furthest_steps(grid, start));
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
