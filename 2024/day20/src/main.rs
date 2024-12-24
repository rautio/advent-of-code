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
    time: i32,
    path: Vec<(Pt, i32)>,
}
impl Cursor {
    pub fn new(p: Pt) -> Self {
        Cursor {
            p,
            time: 0,
            path: vec![],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct CheatCursor {
    p: Pt,
    path: Vec<Pt>,
}
impl CheatCursor {
    pub fn new(p: Pt) -> Self {
        CheatCursor { p, path: vec![] }
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

fn print_path(path: Vec<(Pt, i32)>, grid: &HashMap<Pt, char>) {
    let mut p = Pt::new(0, 0);
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let path_idx = path.iter().position(|&x| x.0 == p);
            if !path_idx.is_none() {
                line.push('O');
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

fn min_path(start: Pt, end: Pt, grid: &HashMap<Pt, char>) -> (Vec<(Pt, i32)>, i32) {
    let mut seen: HashMap<Pt, i32> = HashMap::new();
    seen.insert(start, 0);
    let mut cursors = VecDeque::from([Cursor::new(start)]);
    let mut min_time = 0;
    let mut final_path: Vec<(Pt, i32)> = Vec::new();
    let mut paths: HashMap<(Pt, i32), Vec<(Pt, i32)>> = HashMap::new();
    while cursors.len() > 0 {
        let cur = cursors.pop_front().unwrap();
        let path_key = (cur.p, cur.time);
        if paths.contains_key(&path_key) {
            paths
                .get_mut(&path_key)
                .unwrap()
                .append(&mut cur.path.clone());
        } else {
            paths.insert(path_key, cur.path.clone());
        }
        if cur.p == end {
            // TODO: There could be multiple possible final paths with the same score
            if min_time == 0 {
                min_time = cur.time;
                final_path = cur.path.clone();
            } else if min_time > cur.time {
                min_time = cur.time;
                final_path = cur.path.clone();
            }
        }
        // Either move forward or rotate.
        let next = vec![N, E, S, W]
            .into_iter()
            .map(|x| add(cur.p, x))
            .filter(|p| grid.contains_key(&p));
        for n in next {
            if can_move(n, grid) {
                if !seen.contains_key(&n) || *seen.get(&n).unwrap() >= cur.time {
                    seen.insert(n, cur.time + 1);
                    let mut new_path = cur.path.clone();
                    new_path.push((cur.p, cur.time));
                    cursors.push_back(Cursor {
                        time: cur.time + 1,
                        p: n,
                        path: new_path,
                    });
                }
            }
        }
    }

    final_path.push((end, min_time));

    (final_path, min_time)
}

fn calc_distance(start: Pt, end: Pt) -> i32 {
    (start.x - end.x).abs() + (start.y - end.y).abs()
}

fn get_cheat_paths(start: Pt, end: Pt, length: i32, grid: &HashMap<Pt, char>) -> Vec<Vec<Pt>> {
    let mut seen: HashMap<Pt, i32> = HashMap::new();
    seen.insert(start, 0);
    let mut cursors = VecDeque::from([CheatCursor::new(start)]);
    let mut possible_paths: Vec<Vec<Pt>> = Vec::new();
    while cursors.len() > 0 {
        let cur = cursors.pop_front().unwrap();
        if cur.p == end && cur.path.len() as i32 == length {
            possible_paths.push(cur.path.clone());
        } else {
            let next = vec![N, E, S, W]
                .into_iter()
                .map(|x| add(cur.p, x))
                .filter(|p| {
                    grid.contains_key(&p)
                        && (*grid.get(&p).unwrap() == '#' || *p == end)
                        && !cur.path.contains(p)
                });
            for n in next {
                let mut new_path = cur.path.clone();
                new_path.push(n);
                if new_path.len() as i32 <= length {
                    cursors.push_back(CheatCursor {
                        p: n,
                        path: new_path,
                    });
                }
            }
        }
    }

    possible_paths
}

fn calc_cheats(
    path: &Vec<(Pt, i32)>,
    distance: i32,
    grid: &HashMap<Pt, char>,
) -> Vec<((Pt, Pt), i32)> {
    let mut cheats: Vec<((Pt, Pt), i32)> = Vec::new();
    let mut seen: HashMap<(Pt, Pt), (Pt, Pt)> = HashMap::new();

    for i in 0..path.len() {
        for j in i..path.len() {
            let p1 = path[i];
            let p2 = path[j];
            if p2.1 > p1.1 {
                let d = calc_distance(p1.0, p2.0);
                if d == distance && !seen.contains_key(&(p1.0, p2.0)) {
                    let paths = get_cheat_paths(p1.0, p2.0, distance, grid);
                    if paths.len() > 0 {
                        // saved = p2.time - p1.time - (cheat_path).len()
                        cheats.push(((p1.0, p2.0), p2.1 - p1.1 - distance));
                    }
                }
            }
            seen.insert((p1.0, p2.0), (p1.0, p2.0));
            seen.insert((p2.0, p1.0), (p2.0, p1.0));
        }
    }

    cheats
}

fn calc_num_cheats(start: Pt, end: Pt, grid: &HashMap<Pt, char>, min_cheat: i32) -> i32 {
    let mut total_cheats = 0;
    let (no_cheat_path, _) = min_path(start, end, &grid);
    let cheats: Vec<((Pt, Pt), i32)> = calc_cheats(&no_cheat_path, 2, grid);
    // let mut cheats_map: HashMap<i32, i32> = HashMap::new();
    for c in cheats {
        // let p1 = c.0 .0;
        // let p2 = c.0 .1;
        let saved = c.1;
        if saved >= min_cheat {
            // cheats_map.entry(saved).and_modify(|c| *c += 1).or_insert(1);
            total_cheats += 1;
        }
    }
    // let mut v: Vec<(i32, i32)> = cheats_map.into_iter().collect();
    // v.sort_by(|x, y| y.1.cmp(&x.1));
    // for (cheat, count) in v {
    //     println!("{:?} cheats that save {:?} picoseconds", count, cheat);
    // }
    total_cheats
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
    println!("Part 1: {}", calc_num_cheats(start, end, &grid, 100));
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
