use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Hike {
    cur: Pt,
    path: Vec<Pt>,
}

// up, down, left, right
// (0,-1), (0,1), (-1,0), (1,0)
const UP: Pt = Pt { x: 0, y: -1 };
const DOWN: Pt = Pt { x: 0, y: 1 };
const LEFT: Pt = Pt { x: -1, y: 0 };
const RIGHT: Pt = Pt { x: 1, y: 0 };

fn parse_grid(input: &str) -> (HashMap<Pt, char>, Pt) {
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut end = Pt { x: -1, y: -1 };

    for (_y, line) in read_to_string(input)
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
    {
        let y = _y as i32;
        if y > end.y {
            end.y = y;
        }
        for (_x, c) in line.chars().into_iter().enumerate() {
            let x = _x as i32;
            if x > end.x {
                end.x = x;
            }
            grid.insert(Pt { x: x, y: y }, c);
        }
    }
    return (grid, end);
}

fn get_next_steps(grid: &HashMap<Pt, char>, cur: &Pt, path: Vec<Pt>) -> Vec<Pt> {
    // Check for steeps
    let cur_c = *grid.get(cur).unwrap();
    let mut dir_chars: HashMap<char, Pt> = HashMap::new();
    dir_chars.insert('v', DOWN);
    dir_chars.insert('^', UP);
    dir_chars.insert('<', LEFT);
    dir_chars.insert('>', RIGHT);

    if cur_c == '>' || cur_c == '<' || cur_c == '^' || cur_c == 'v' {
        let dir = dir_chars.get(&cur_c).unwrap();
        let next = Pt {
            x: cur.x + dir.x,
            y: cur.y + dir.y,
        };
        return vec![next];
    }

    // Normal steps
    let possible = vec![UP, DOWN, LEFT, RIGHT];
    let mut next_steps: Vec<Pt> = Vec::new();
    for dir in possible {
        let next = Pt {
            x: cur.x + dir.x,
            y: cur.y + dir.y,
        };
        if grid.contains_key(&next) && !path.contains(&next) {
            let next_c = *grid.get(&next).unwrap();
            if next_c == '.' {
                next_steps.push(next);
            } else if (next_c == '^' && dir != DOWN)
                || (next_c == 'v' && dir != UP)
                || (next_c == '>' && dir != LEFT)
                || (next_c == '<' && dir != RIGHT)
            {
                next_steps.push(next);
            }
        }
    }
    return next_steps;
}

fn calc_hikes(grid: &HashMap<Pt, char>, start: &Pt) -> Vec<Hike> {
    let mut hikes: VecDeque<Hike> = VecDeque::new();
    let mut final_hikes: Vec<Hike> = Vec::new();
    hikes.push_back(Hike {
        cur: *start,
        path: Vec::new(),
    });
    while hikes.len() > 0 {
        let h = hikes.pop_front().unwrap();
        let cur = h.cur.clone();
        let mut path = h.path.clone();
        let next_steps = get_next_steps(grid, &h.cur, path.clone());
        if next_steps.len() == 0 {
            final_hikes.push(h);
        }
        for next in next_steps {
            path.push(cur);
            let new_hike = Hike {
                cur: next,
                path: path.clone(),
            };
            hikes.push_back(new_hike);
        }
    }
    return final_hikes;
}

// fn print_grid(grid: &HashMap<Pt, char>, path: Vec<Pt>, max: Pt) {
//     for _y in 0..max.y + 1 {
//         let y = _y as i32;
//         let mut line = String::from("");
//         for _x in 0..max.x + 1 {
//             let x = _x as i32;
//             if path.contains(&Pt { x: x, y: y }) {
//                 line.push('O');
//             } else {
//                 line.push(*grid.get(&Pt { x: x, y: y }).unwrap());
//             }
//         }
//         println!("{}", line);
//     }
// }

fn solve_part1(grid: &HashMap<Pt, char>, end: &Pt) -> usize {
    let mut max_path = 0;
    let mut longest: Vec<Pt> = Vec::new();
    let hikes = calc_hikes(grid, &Pt { x: 1, y: 0 });
    for h in hikes {
        if h.path.len() >= max_path && h.cur == *end {
            max_path = h.path.len();
            longest = h.path.clone();
        }
    }
    longest.dedup();
    // print_grid(
    //     grid,
    //     longest.clone(),
    //     Pt {
    //         x: end.x + 1,
    //         y: end.y,
    //     },
    // );
    return longest.len();
}
fn main() {
    let mut now = Instant::now();
    let (grid, max) = parse_grid("./input.txt");
    let end = Pt {
        x: max.x - 1,
        y: max.y,
    };
    println!("Part 1: {}", solve_part1(&grid, &end));
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    //     now = Instant::now();
    //     println!("Part 2: {}", step(&grid, &start, 10000).len());
    //     println!("Done in: {:.2?}!", now.elapsed());
}
