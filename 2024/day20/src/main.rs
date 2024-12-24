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
        for mut n in next {
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

fn get_cheats(path: Vec<(Pt, i32)>, grid: &HashMap<Pt, char>) -> Vec<(Pt, Pt)> {
    let mut cheats: HashMap<(Pt, Pt), (Pt, Pt)> = HashMap::new();
    let mut real_path: HashMap<Pt, i32> = HashMap::new();
    for p in &path {
        real_path.insert(p.0, p.1);
    }
    for p in path {
        // 1 or 2 away has to be another part of the path that could cutoff a subsection
        let first = vec![N, E, S, W]
            .into_iter()
            .map(|x| add(p.0, x))
            .filter(|q| !real_path.contains_key(&q));
        for p1 in first {
            // p1 has to be '#' but p2 has to be != '#'
            let second = vec![N, E, S, W]
                .into_iter()
                .map(|x| add(p1, x))
                .filter(|q| {
                    real_path.contains_key(&q) && *real_path.get(&q).unwrap() > p.1 && *q != p.0
                });
            // Each point should have 1 optimal cheat based on which steps count is highest
            let mut max_steps = 0;
            let mut best_p2 = 
            for p2 in second {
                if !cheats.contains_key(&(p1, p2)) && !cheats.contains_key(&(p2, p1)) {
                    cheats.insert((p1, p2), (p1, p2));
                }
            }
        }
    }
    let mut dedupped = cheats.clone();
    for (k, c) in dedupped.clone().into_iter() {
        if dedupped.contains_key(&(c.1, c.0)) {
            dedupped.remove(&(c.1, c.0));
        }
    }

    cheats.values().cloned().collect()
}

fn cheat_min_path(
    start: Pt,
    end: Pt,
    cheat: (Pt, Pt),
    grid: &HashMap<Pt, char>,
) -> (Vec<(Pt, i32)>, i32) {
    let mut grid_copy = grid.clone();
    grid_copy.insert(cheat.0, '.');
    grid_copy.insert(cheat.1, '.');
    min_path(start, end, &grid_copy)
}

fn num_cheats(start: Pt, end: Pt, grid: &HashMap<Pt, char>, min_cheat: i32) -> i32 {
    let mut total_cheats = 0;
    let (mut no_cheat_path, min_time) = min_path(start, end, &grid);
    // print_path(no_cheat_path.clone(), grid);
    let cheats: Vec<(Pt, Pt)> = get_cheats(no_cheat_path, grid);
    let mut cheats_map: HashMap<i32, i32> = HashMap::new();
    for c in cheats {
        let (new_path, new_min_time) = cheat_min_path(start, end, c, grid);
        // Both points in cheat have to be in the new path
        let c1_idx = new_path.iter().position(|&q| q.0 == c.0);
        let c2_idx = new_path.iter().position(|&q| q.0 == c.1);
        if !c1_idx.is_none() && !c2_idx.is_none() {
            // if new_min_time < min_time {
            //     if min_time - new_min_time == 2 {
            //         println!("cheat: {:?} - saves: {}", c, min_time - new_min_time);
            //     }
            //     cheats_map
            //         .entry(min_time - new_min_time)
            //         .and_modify(|c| *c += 1)
            //         .or_insert(1);
            // }
            if min_time - new_min_time >= min_cheat {
                total_cheats += 1;
            }
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

    println!("min_time: {}", num_cheats(start, end, &grid, 100));
    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
