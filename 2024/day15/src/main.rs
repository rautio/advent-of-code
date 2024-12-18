use std::collections::HashMap;
use std::collections::VecDeque;
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

const N: Pt = Pt { x: 0, y: -1 };
const E: Pt = Pt { x: 1, y: 0 };
const S: Pt = Pt { x: 0, y: 1 };
const W: Pt = Pt { x: -1, y: 0 };

fn add(p1: Pt, p2: Pt) -> Pt {
    Pt {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn print_large_warehouse(robot: Pt, boxes: &Vec<(Pt, Pt)>, grid: &HashMap<Pt, char>) {
    let mut p = Pt::new(0, 0);
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let box_idx0 = boxes.iter().position(|&x| x.0 == p);
            let box_idx1 = boxes.iter().position(|&x| x.1 == p);
            if !box_idx0.is_none() {
                line.push('[');
            } else if !box_idx1.is_none() {
                line.push(']');
            } else if p == robot {
                line.push('@');
            } else {
                line.push(*grid.get(&p).unwrap());
            }
            p.x += 1;
        }
        p.x = 0;
        p.y += 1;
        println!("{}", line);
    }
}

fn print_warehouse(robot: Pt, boxes: &Vec<Pt>, grid: &HashMap<Pt, char>) {
    let mut p = Pt::new(0, 0);
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let box_idx = boxes.iter().position(|&x| x == p);
            if !box_idx.is_none() {
                line.push('O');
            } else if p == robot {
                line.push('@');
            } else {
                line.push(*grid.get(&p).unwrap());
            }
            p.x += 1;
        }
        p.x = 0;
        p.y += 1;
        println!("{}", line);
    }
}

fn can_move(cur: Pt, grid: &HashMap<Pt, char>) -> bool {
    grid.contains_key(&cur) && *grid.get(&cur).unwrap() != '#'
}

fn move_boxes(
    start: Pt,
    start_boxes: Vec<Pt>,
    moves: &Vec<Pt>,
    grid: &HashMap<Pt, char>,
) -> (Vec<Pt>, Pt) {
    let mut boxes: HashMap<Pt, Pt> = HashMap::new();
    for b in start_boxes {
        boxes.insert(b, b);
    }
    let mut robot: Pt = start;

    for m in moves {
        let mut checking = VecDeque::from([robot]);
        let mut move_boxes: Vec<(Pt, Pt)> = Vec::new();
        let mut can_move_robot = false;
        while checking.len() > 0 {
            let next = add(checking.pop_front().unwrap(), *m);
            if boxes.contains_key(&next) {
                move_boxes.push((next, add(next, *m)));
                checking.push_back(next);
            } else if can_move(next, grid) {
                can_move_robot = true;
            }
        }
        if can_move_robot {
            robot = add(robot, *m);
            for (old_key, new_key) in move_boxes.into_iter().rev() {
                if let _ = boxes.remove(&old_key) {
                    boxes.insert(new_key, new_key);
                }
            }
        }
    }

    (boxes.values().cloned().collect(), robot)
}

fn sum_gps(boxes: &Vec<Pt>) -> i32 {
    let mut sum = 0;

    for b in boxes {
        sum += b.x + 100 * b.y;
    }

    sum
}

fn sum_large_gps(boxes: &Vec<(Pt, Pt)>) -> i32 {
    let mut sum = 0;

    for b in boxes {
        sum += b.0.x + 100 * b.0.y;
    }

    sum
}

fn can_move_boxes(
    start: Pt,
    dir: Pt,
    boxes: &Vec<(Pt, Pt)>,
    grid: &HashMap<Pt, char>,
    moved_boxes: &mut HashMap<(Pt, Pt), (Pt, Pt)>,
) -> bool {
    let box_idx = boxes.iter().position(|&x| x.0 == start || x.1 == start);
    if box_idx.is_none() {
        if *grid.get(&start).unwrap() == '.' {
            return true;
        } else if *grid.get(&start).unwrap() == '#' {
            return false;
        }
    }
    // Otherwise its a box
    let b = boxes[box_idx.unwrap()];
    moved_boxes.insert(b, b);
    if add(b.0, dir) == b.1 || add(b.1, dir) == b.0 {
        return can_move_boxes(add(add(start, dir), dir), dir, boxes, grid, moved_boxes);
    }
    return can_move_boxes(add(b.0, dir), dir, boxes, grid, moved_boxes)
        && can_move_boxes(add(b.1, dir), dir, boxes, grid, moved_boxes);
}

fn move_large_boxes(
    start: Pt,
    start_boxes: Vec<(Pt, Pt)>,
    moves: &Vec<Pt>,
    grid: &HashMap<Pt, char>,
) -> (Vec<(Pt, Pt)>, Pt) {
    let mut boxes = start_boxes.clone();
    let mut robot: Pt = start;

    for m in moves {
        let mut moved_boxes: HashMap<(Pt, Pt), (Pt, Pt)> = HashMap::new();
        let next = add(robot, *m);
        let can_move = can_move_boxes(next, *m, &boxes, grid, &mut moved_boxes);
        if can_move {
            robot = next;
            for b in moved_boxes.values() {
                let box_idx = boxes.iter().position(|&x| x == *b).unwrap();
                let bx = boxes[box_idx];
                boxes[box_idx] = (add(bx.0, *m), add(bx.1, *m));
            }
        }
    }

    (boxes, robot)
}

fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut large_grid: HashMap<Pt, char> = HashMap::new();
    let mut boxes: Vec<Pt> = Vec::new();
    let mut large_boxes: Vec<(Pt, Pt)> = Vec::new();
    let mut robot: Pt = Pt::new(0, 0);
    let mut large_robot: Pt = Pt::new(0, 0);
    let mut y = 0;
    let mut parsing_grid = true;
    let mut moves: Vec<Pt> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        if line == "" {
            parsing_grid = false;
        }
        if parsing_grid {
            let mut x = 0;
            let mut large_x = 0;
            for c in line.chars().collect::<Vec<char>>() {
                if c == '#' {
                    grid.insert(Pt::new(x, y), c);
                    large_grid.insert(Pt::new(large_x, y), c);
                    large_grid.insert(Pt::new(large_x + 1, y), c);
                } else {
                    grid.insert(Pt::new(x, y), '.');
                    large_grid.insert(Pt::new(large_x, y), '.');
                    large_grid.insert(Pt::new(large_x + 1, y), '.');
                }
                if c == '@' {
                    robot = Pt::new(x, y);
                    large_robot = Pt::new(large_x, y);
                }
                if c == 'O' {
                    boxes.push(Pt::new(x, y));
                    large_boxes.push((Pt::new(large_x, y), Pt::new(large_x + 1, y)));
                }
                x += 1;
                large_x += 2;
            }
            y += 1;
        } else {
            for c in line.chars().collect::<Vec<char>>() {
                moves.push(match c {
                    '^' => N,
                    '>' => E,
                    'v' => S,
                    '<' => W,
                    _ => panic!("oh no"),
                })
            }
        }
    }
    // print_warehouse(robot, &boxes, &grid);
    let (moved_boxes, _) = move_boxes(robot, boxes.clone(), &moves, &grid);
    // print_warehouse(moved_robot, &moved_boxes, &grid);
    println!("Part 1: {}", sum_gps(&moved_boxes));
    // print_large_warehouse(large_robot, &large_boxes, &large_grid);
    let (moved_large_boxes, _) =
        move_large_boxes(large_robot, large_boxes.clone(), &moves, &large_grid);
    // print_large_warehouse(moved_large_robot, &moved_large_boxes, &large_grid);
    println!("Part 2: {}", sum_large_gps(&moved_large_boxes));
    println!("Done in: {:?}!", now.elapsed());
}
