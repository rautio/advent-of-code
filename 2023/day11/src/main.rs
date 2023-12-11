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
    max_x: usize,
    max_y: usize,
    existing_paths: &HashMap<Vec<G>, u32>,
) -> u32 {
    let mut distances: HashMap<G, u32> = HashMap::new();
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
        // Up
        if cur_g.y > 0 && start.y < cur_g.y {
            // Only move in the given direction if start is in that direction
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
            if g.x <= max_x && g.y <= max_y && !distances.contains_key(&g) && !seen.contains_key(&g)
            {
                let cur_distance = distances.get(&cur_g).unwrap() + 1;
                distances.insert(g, cur_distance);
                queue.push_back(g);
            }
        }
        seen.insert(cur_g, true);
    }
    return *distances.get(&start).unwrap();
}

fn main() {
    let now = Instant::now();
    let mut raw_galaxies: Vec<G> = Vec::new();
    let mut galaxies: Vec<G> = Vec::new(); // Expanded galaxies
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
                raw_galaxies.push(G { x, y });
                col_has_g.insert(x, true);
                row_has_g.insert(y, true);
            }
        }
    }
    let mut max_x = 0;
    let mut max_y = 0;
    // Expand
    for g in &raw_galaxies {
        let mut x = g.x;
        let mut y = g.y;
        for i in 0..g.x {
            if col_has_g.contains_key(&i) && !col_has_g.get(&i).unwrap() {
                // It's empty
                x += 1;
            }
        }
        for i in 0..g.y {
            if row_has_g.contains_key(&i) && !row_has_g.get(&i).unwrap() {
                // It's empty
                y += 1;
            }
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        galaxies.push(G { x, y });
    }
    let mut sum = 0;
    let mut shortest_paths: HashMap<Vec<G>, u32> = HashMap::new();
    for (j, g) in galaxies.iter().enumerate() {
        for i in j + 1..galaxies.len() {
            let shortest = shortest_path(*g, galaxies[i], max_x, max_y, &shortest_paths);
            shortest_paths.insert(vec![*g, galaxies[i]], shortest);
            sum += shortest;
        }
    }
    // Part 1
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
