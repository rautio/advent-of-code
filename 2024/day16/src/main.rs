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
    path: Vec<(Pt, Pt, i32)>,
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

fn print_path(path: Vec<(Pt, Pt, i32)>, grid: &HashMap<Pt, char>, use_o: bool) {
    let mut p = Pt::new(0, 0);
    let dirs: Vec<Pt> = vec![E, S, W, N];
    let chars: Vec<char> = vec!['>', 'v', '<', '^'];
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let path_idx = path.iter().position(|&x| x.0 == p);
            if path_idx.is_none() || *grid.get(&p).unwrap() == 'S' {
                line.push(*grid.get(&p).unwrap());
            } else if !use_o {
                let path = path[path_idx.unwrap()];
                let dir_idx = dirs.iter().position(|&x| x == path.1).unwrap();
                line.push(chars[dir_idx]);
            } else {
                line.push('O');
            }
            p.x += 1;
        }
        p.x = 0;
        p.y += 1;
        println!("{}", line);
    }
}

fn min_score(grid: &HashMap<Pt, char>, start: Pt, end: Pt) -> (i32, usize) {
    let dirs: Vec<Pt> = vec![N, E, S, W];
    let mut seen: HashMap<(Pt, Pt), i32> = HashMap::new();
    seen.insert((start, E), 0);
    let mut cursors = VecDeque::from([Cursor::new(start, E, 0)]);
    let mut min_score = 0;
    let mut final_path: Vec<(Pt, Pt, i32)> = Vec::new();
    let mut paths: HashMap<(Pt, Pt, i32), Vec<(Pt, Pt, i32)>> = HashMap::new();
    while cursors.len() > 0 {
        let cur = cursors.pop_front().unwrap();
        let path_key = (cur.p, cur.dir, cur.score);
        if paths.contains_key(&path_key) {
            paths
                .get_mut(&path_key)
                .unwrap()
                .append(&mut cur.path.clone());
        } else {
            paths.insert(path_key, cur.path.clone());
        }
        if cur.p == end {
            // There are 4 possible directions for the end
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
                if !seen.contains_key(&(n.p, n.dir)) || *seen.get(&(n.p, n.dir)).unwrap() >= n.score
                {
                    seen.insert((n.p, n.dir), n.score);
                    n.path = cur.path.clone();
                    n.path.push((cur.p, cur.dir, cur.score));
                    cursors.push_back(n);
                }
            }
        }
    }
    // print_path(final_path.clone(), grid, false);
    let mut pts: HashMap<Pt, bool> = HashMap::new();
    let mut full_path: Vec<(Pt, Pt, i32)> = Vec::new();
    for p in final_path {
        if paths.contains_key(&p) {
            for x in paths.get(&p).unwrap().clone() {
                pts.insert(x.0, true);
                full_path.push(x);
            }
        }
    }
    // print_path(full_path.clone(), grid, true);
    (min_score, pts.keys().len() + 2)
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
    let (min_score, best_seats) = min_score(&grid, start, end);
    println!("Part 1: {}", min_score);
    println!("Part 2: {}", best_seats);
    println!("Done in: {:?}!", now.elapsed());
}
