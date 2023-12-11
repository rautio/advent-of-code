use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

// Galaxy
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct G {
    x: usize,
    y: usize,
}

fn shortest_path(
    start: G,
    end: G,
    existing_paths: &HashMap<Vec<G>, u64>,
    row_has_g: &HashMap<usize, bool>,
    col_has_g: &HashMap<usize, bool>,
    skip_amount: u64,
) -> u64 {
    let mut distances: HashMap<G, u64> = HashMap::new();
    let mut seen: HashMap<G, bool> = HashMap::new();
    distances.insert(end, 0);
    // Possible moves
    let mut queue: VecDeque<G> = VecDeque::new();
    queue.push_back(end);
    while queue.len() > 0 {
        let cur_g = queue.pop_front().unwrap();
        // As soon as we see the start, return because its the shortest path.
        if cur_g.x == start.x && cur_g.y == start.y {
            return *distances.get(&cur_g).unwrap();
        }
        // If we know the distance from this point to start, return that
        if existing_paths.contains_key(&vec![start, cur_g]) {
            return *distances.get(&cur_g).unwrap()
                + existing_paths.get(&vec![start, cur_g]).unwrap();
        }
        if existing_paths.contains_key(&vec![cur_g, start]) {
            return *distances.get(&cur_g).unwrap()
                + existing_paths.get(&vec![cur_g, start]).unwrap();
        }

        let mut possible_paths: Vec<G> = Vec::new();
        // Only move in the given direction if start is in that direction
        // Up
        if cur_g.y > 0 && start.y < cur_g.y {
            possible_paths.push(G {
                x: cur_g.x,
                y: cur_g.y - 1,
            });
        }
        // Left
        if cur_g.x > 0 && start.x < cur_g.x {
            possible_paths.push(G {
                x: cur_g.x - 1,
                y: cur_g.y,
            });
        }
        // Down
        if start.y > cur_g.y {
            possible_paths.push(G {
                x: cur_g.x,
                y: cur_g.y + 1,
            });
        }
        // Right
        if start.x > cur_g.x {
            possible_paths.push(G {
                x: cur_g.x + 1,
                y: cur_g.y,
            });
        }
        for g in possible_paths {
            if !distances.contains_key(&g) && !seen.contains_key(&g) {
                let mut dis = 1;
                if g.y != cur_g.y && row_has_g.contains_key(&g.y) && !row_has_g.get(&g.y).unwrap() {
                    dis += skip_amount - 1;
                }
                if g.x != cur_g.x && col_has_g.contains_key(&g.x) && !col_has_g.get(&g.x).unwrap() {
                    dis += skip_amount - 1;
                }
                let cur_distance = distances.get(&cur_g).unwrap() + dis;
                distances.insert(g, cur_distance);
                queue.push_back(g);
            }
        }
        seen.insert(cur_g, true);
    }
    return *distances.get(&start).unwrap();
}

fn sum_shortest_paths(
    galaxies: &Vec<G>,
    row_has_g: &HashMap<usize, bool>,
    col_has_g: &HashMap<usize, bool>,
    skip_amount: u64,
) -> u64 {
    let mut sum: u64 = 0;
    let mut shortest_paths: HashMap<Vec<G>, u64> = HashMap::new();
    for (j, g) in galaxies.iter().enumerate() {
        for i in j + 1..galaxies.len() {
            let shortest = shortest_path(
                *g,
                galaxies[i],
                &shortest_paths,
                row_has_g,
                col_has_g,
                skip_amount,
            );
            shortest_paths.insert(vec![*g, galaxies[i]], shortest);
            sum += shortest;
        }
    }
    return sum;
}

fn main() {
    let mut now = Instant::now();
    let mut galaxies: Vec<G> = Vec::new();
    let mut row_has_g: HashMap<usize, bool> = HashMap::new();
    let mut col_has_g: HashMap<usize, bool> = HashMap::new();
    for (y, line) in read_to_string("./input.txt").unwrap().lines().enumerate() {
        if !row_has_g.contains_key(&y) {
            row_has_g.insert(y, false);
        }
        let l: Vec<char> = line.chars().collect();
        for (x, c) in l.iter().enumerate() {
            if !col_has_g.contains_key(&x) {
                col_has_g.insert(x, false);
            }
            if *c == '#' {
                galaxies.push(G { x, y });
                col_has_g.insert(x, true);
                row_has_g.insert(y, true);
            }
        }
    }
    // Part 1
    println!(
        "Part 1: {}",
        sum_shortest_paths(&galaxies, &row_has_g, &col_has_g, 2)
    );
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    now = Instant::now();
    println!(
        "Part 2: {}",
        sum_shortest_paths(&galaxies, &row_has_g, &col_has_g, 1000000)
    );
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
