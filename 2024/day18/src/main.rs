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

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    p: Pt,
    steps: i32,
    path: HashMap<Pt, i32>,
}
impl State {
    pub fn new(p: Pt, steps: i32, path: HashMap<Pt, i32>) -> Self {
        State { p, steps, path }
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

fn print_grid(grid: &HashMap<Pt, char>, path: Vec<Pt>) {
    let mut p = Pt::new(0, 0);
    while grid.contains_key(&p) {
        let mut line = String::new();
        while grid.contains_key(&p) {
            let path_idx = path.iter().position(|&x| x == p);
            if path_idx.is_none() {
                line.push(*grid.get(&p).unwrap());
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

fn min_steps(bytes: Vec<Pt>, bytes_to_simulate: usize, max: Pt) -> i32 {
    // Create grid
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut p = Pt::new(0, 0);
    while p.y <= max.y {
        while p.x <= max.x {
            grid.insert(p, '.');
            p.x += 1;
        }
        p.x = 0;
        p.y += 1;
    }
    for b in 0..bytes_to_simulate {
        grid.insert(bytes[b], '#');
    }
    // print_grid(&grid, vec![]);

    let mut steps: HashMap<Pt, i32> = HashMap::new();
    let mut start_state = State::new(Pt::new(0, 0), 0, HashMap::new());
    start_state.path.insert(Pt::new(0, 0), 0);
    let mut states: VecDeque<State> = VecDeque::from([start_state]);

    while states.len() > 0 {
        let state = states.pop_front().unwrap();
        let cur_p = state.p;
        let next: Vec<Pt> = vec![N, E, S, W]
            .into_iter()
            .map(|n| add(cur_p, n)) // All possible directions
            .filter(|n| !state.path.contains_key(&n)) // Cant go back to where we were
            .filter(|n| grid.contains_key(&n) && *grid.get(&n).unwrap() == '.') // Has to be a free spot
            .filter(|n| !steps.contains_key(&n) || *steps.get(&n).unwrap() > state.steps + 1) // No point in checking a spot we've alrady seen
            .collect();
        for n in next {
            let mut n_path = state.path.clone();
            n_path.insert(n, state.steps + 1);
            let n_state = State::new(n, state.steps + 1, n_path);
            steps.insert(n, n_state.steps);
            states.push_back(n_state);
        }
    }

    *steps.get(&max).unwrap_or(&0)
}

fn main() {
    let now = Instant::now();
    let mut bytes: Vec<Pt> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(',').collect();
        bytes.push(Pt::new(
            splits[0].parse::<i32>().unwrap(),
            splits[1].parse::<i32>().unwrap(),
        ));
    }
    println!(
        "Part 1: {}",
        min_steps(bytes.clone(), 1024, Pt::new(70, 70))
    );
    for i in 1024..bytes.len() {
        if min_steps(bytes.clone(), i, Pt::new(70, 70)) == 0 {
            println!("Part 2: {},{}", bytes[i - 1].x, bytes[i - 1].y);
            break;
        }
    }
    println!("Done in: {:?}!", now.elapsed());
}
