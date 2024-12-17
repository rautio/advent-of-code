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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cursor {
    p: Pt,
    score: i32,
    dir: Pt,
    path: Vec<(Pt, Pt)>,
}
impl Cursor {
    pub fn new(p: Pt, dir: Pt, score: i32) -> Self {
        Cursor {
            p,
            score,
            dir,
            path: vec![],
        }
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

fn can_move(p: Pt, grid: &HashMap<Pt, char>) -> bool {
    grid.contains_key(&p) && *grid.get(&p).unwrap() != '#'
}

fn print_path(path: Vec<(Pt, Pt)>, grid: &HashMap<Pt, char>) {
    let mut p = Pt::new(0, 0);
    let dirs: Vec<Pt> = vec![E, S, W, N];
    let chars: Vec<char> = vec!['>', 'v', '<', '^'];
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let path_idx = path.iter().position(|&x| x.0 == p);
            if path_idx.is_none() || *grid.get(&p).unwrap() == 'S' {
                line.push(*grid.get(&p).unwrap());
            } else {
                let path = path[path_idx.unwrap()];
                let dir_idx = dirs.iter().position(|&x| x == path.1).unwrap();
                line.push(chars[dir_idx]);
            }
            p.x += 1;
        }
        p.x = 0;
        p.y += 1;
        println!("{}", line);
    }
}

fn min_score(grid: &HashMap<Pt, char>, start: Pt, end: Pt) -> i32 {
    let dirs: Vec<Pt> = vec![E, S, W, N];
    let mut seen: HashMap<(Pt, Pt), bool> = HashMap::new();
    let mut scores: HashMap<Pt, i32> = HashMap::new();
    scores.insert(start, 0);
    seen.insert((start, E), true);
    let mut cursors = VecDeque::from([Cursor::new(start, E, 0)]);
    let mut min_score = 0;
    let mut final_path: Vec<(Pt, Pt)> = Vec::new();
    while cursors.len() > 0 {
        let cur = cursors.pop_front().unwrap();
        if cur.p == end {
            if min_score == 0 {
                min_score = cur.score;
                final_path = cur.path.clone();
            } else if min_score > cur.score {
                min_score = cur.score;
                final_path = cur.path.clone();
            }
        }
        // Either move forward or rotate.
        let forward = Cursor::new(add(cur.p, cur.dir), cur.dir, cur.score + 1);
        let dir_idx = dirs.iter().position(|&r| r == cur.dir).unwrap();
        let counter_dir_idx = if dir_idx == 0 {
            dirs.len() - 1
        } else {
            (dir_idx - 1) % dirs.len()
        };
        let rotate_clockwise =
            Cursor::new(cur.p, dirs[(dir_idx + 1) % dirs.len()], cur.score + 1000);
        let rotate_counter_clockwise = Cursor::new(cur.p, dirs[counter_dir_idx], cur.score + 1000);
        let next = vec![forward, rotate_clockwise, rotate_counter_clockwise];
        for mut n in next {
            if can_move(n.p, grid) {
                if !seen.contains_key(&(n.p, n.dir)) || *scores.get(&n.p).unwrap() > n.score {
                    seen.insert((n.p, n.dir), true);
                    scores.insert(n.p, n.score);
                    n.path = cur.path.clone();
                    n.path.push((cur.p, cur.dir));
                    cursors.push_back(n);
                }
            }
        }
    }
    print_path(final_path, grid);
    min_score
}

fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut y = 0;
    let mut start = Pt::new(0, 0);
    let mut end = Pt::new(0, 0);
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for ch in l {
            if ch == 'S' {
                start = Pt::new(x, y);
            }
            if ch == 'E' {
                end = Pt::new(x, y);
            }
            grid.insert(Pt::new(x, y), ch);
            x += 1;
        }
        y += 1;
    }
    println!("Part 1: {}", min_score(&grid, start, end));
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
