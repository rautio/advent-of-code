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

fn print_grid(robot: Pt, boxes: &Vec<Pt>, grid: &HashMap<Pt, char>) {
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

fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut boxes: Vec<Pt> = Vec::new();
    let mut robot: Pt = Pt::new(0, 0);
    let mut y = 0;
    let mut parsing_grid = true;
    let mut moves: Vec<Pt> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        if line == "" {
            parsing_grid = false;
        }
        if parsing_grid {
            let mut x = 0;
            for c in line.chars().collect::<Vec<char>>() {
                if c == '#' {
                    grid.insert(Pt::new(x, y), c);
                } else {
                    grid.insert(Pt::new(x, y), '.');
                }
                if c == '@' {
                    robot = Pt::new(x, y);
                }
                if c == 'O' {
                    boxes.push(Pt::new(x, y));
                }
                x += 1;
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
    print_grid(robot, &boxes, &grid);
    let (moved_boxes, moved_robot) = move_boxes(robot, boxes.clone(), &moves, &grid);
    println!("-----");
    print_grid(moved_robot, &moved_boxes, &grid);
    println!("Part 1: {}", sum_gps(&moved_boxes));
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
