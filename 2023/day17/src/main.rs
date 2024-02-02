use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Marker {
    x: i32,
    y: i32,
    dir: Pt,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Ptr {
    x: i32,
    y: i32,
    dir: Pt,
    distance: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    x: i32,
    y: i32,
    dir: Pt,
    distance: usize,
    loss: u32,
}

// up, down, left, right
// (0,-1), (0,1), (-1,0), (1,0)
const UP: Pt = Pt { x: 0, y: -1 };
const DOWN: Pt = Pt { x: 0, y: 1 };
const LEFT: Pt = Pt { x: -1, y: 0 };
const RIGHT: Pt = Pt { x: 1, y: 0 };

fn parse_grid(file_name: &str) -> (HashMap<Pt, u32>, Pt) {
    let mut grid: HashMap<Pt, u32> = HashMap::new();
    let mut max = Pt { x: 0, y: 0 };
    for (_y, line) in read_to_string(file_name)
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
    {
        let y = _y as i32;
        if y > max.y {
            max.y = y;
        }
        for (_x, c) in line.chars().into_iter().enumerate() {
            let x = _x as i32;
            if x > max.x {
                max.x = x;
            }
            grid.insert(Pt { x: x, y: y }, c.to_digit(10).unwrap());
        }
    }
    return (grid, max);
}

fn get_possible(cur: State, max: Pt) -> Vec<State> {
    let mut res: Vec<State> = Vec::new();
    let backwards = Pt {
        x: cur.x - cur.dir.x,
        y: cur.y - cur.dir.y,
    };
    let possible = vec![
        State {
            x: cur.x,
            y: cur.y + 1,
            dir: DOWN,
            distance: 1,
            loss: 0,
        },
        State {
            x: cur.x,
            y: cur.y - 1,
            dir: UP,
            distance: 1,
            loss: 0,
        },
        State {
            x: cur.x - 1,
            y: cur.y,
            dir: LEFT,
            distance: 1,
            loss: 0,
        },
        State {
            x: cur.x + 1,
            y: cur.y,
            dir: RIGHT,
            distance: 1,
            loss: 0,
        },
    ];
    for p in possible {
        if p.x == backwards.x && p.y == backwards.y {
            // can't go back
            continue;
        } else if p.x <= max.x && p.y <= max.y && p.x >= 0 && p.y >= 0 {
            if p.dir == cur.dir {
                // TODO: Optimize by getting all possible options in the same dir
                res.push(State {
                    x: p.x,
                    y: p.y,
                    dir: p.dir,
                    distance: cur.distance + p.distance,
                    loss: 0,
                });
            } else {
                res.push(p);
            }
        }
    }
    return res;
}

fn get_next_steps(cur: Marker, steps_taken: u32, max: Pt) -> Vec<Marker> {
    let mut steps: Vec<Marker> = Vec::new();
    let backwards = Pt {
        x: cur.x - cur.dir.x,
        y: cur.y - cur.dir.y,
    };
    let possible = vec![
        Marker {
            x: cur.x,
            y: cur.y + 1,
            dir: DOWN,
        },
        Marker {
            x: cur.x,
            y: cur.y - 1,
            dir: UP,
        },
        Marker {
            x: cur.x - 1,
            y: cur.y,
            dir: LEFT,
        },
        Marker {
            x: cur.x + 1,
            y: cur.y,
            dir: RIGHT,
        },
    ];
    // if cur.x == 7 && cur.y == 1 {
    //     println!("backwards: {:?}", backwards);
    //     println!("possible: {:?} ", possible);
    // }
    for p in possible {
        if p.x == backwards.x && p.y == backwards.y {
            // can't go back
            continue;
        } else if p.x <= max.x && p.y <= max.y && p.x >= 0 && p.y >= 0 {
            if p.dir == cur.dir {
                if (steps_taken < 3) {
                    steps.push(p);
                }
            } else {
                steps.push(p);
            }
        }
    }
    return steps;
}

fn print_grid(grid: &HashMap<Pt, u32>, prev: &HashMap<Pt, Pt>, max: Pt) {
    for _y in 0..max.y + 1 {
        let y = _y as i32;
        let mut line: String = String::from("");
        for _x in 0..max.x + 1 {
            let x = _x as i32;
            let p = Pt { x: x, y: y };
            if !prev.contains_key(&p) {
                line += &grid.get(&p).unwrap().to_string();
            } else {
                let dir = *prev.get(&p).unwrap();
                if (dir == RIGHT) {
                    line.push('>');
                } else if dir == UP {
                    line.push('^');
                } else if dir == LEFT {
                    line.push('<');
                } else if dir == DOWN {
                    line.push('v');
                }
            }
        }
        println!("{}", line);
    }
}

fn create_key(cur: &Marker, steps_taken: u32, dir: Pt) -> String {
    let mut key = String::from("");
    key += &cur.x.to_string();
    key.push(',');
    key += &cur.y.to_string();
    key.push('-');
    key += &cur.dir.x.to_string();
    key.push(',');
    key += &cur.dir.y.to_string();
    key.push('-');
    key += &steps_taken.to_string();
    key.push(':');
    key += &dir.x.to_string();
    key.push(',');
    key += &dir.y.to_string();
    return key;
}

fn get_key(s: State) -> String {
    let mut key = String::from("");
    key += &s.x.to_string();
    key.push(',');
    key += &s.y.to_string();
    key.push('-');
    key += &s.dir.x.to_string();
    key.push(',');
    key += &s.dir.y.to_string();
    key.push('-');
    key += &s.distance.to_string();
    return key;
}

fn solve(grid: &HashMap<Pt, u32>, start: State, max_steps: usize, max: Pt) -> u32 {
    let mut states: HashMap<Pt, State> = HashMap::new();
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut seen: HashMap<String, bool> = HashMap::new();
    queue.push_back(start);
    seen.insert(get_key(start), true);
    while queue.len() > 0 {
        println!("queue len: {}", queue.len());
        let cur = queue.pop_front().unwrap();
        let neighbors = get_possible(cur, max);
        if cur.x == max.x && cur.y == max.y {
            return cur.loss;
        }
        for n in neighbors {
            if n.distance < max_steps {
                let pt = Pt { x: n.x, y: n.y };
                let new_loss = *grid.get(&pt).unwrap();
                let new_state = State {
                    x: n.x,
                    y: n.y,
                    dir: n.dir,
                    distance: n.distance,
                    loss: cur.loss + new_loss,
                };
                if !states.contains_key(&pt) {
                    states.insert(pt, new_state);
                    queue.push_back(new_state);
                } else {
                    let next_state = states.get(&pt).unwrap();
                    if cur.loss + new_loss < next_state.loss {
                        // states.get_mut(&pt).unwrap().loss += cur.loss + new_loss;
                        states.insert(pt, new_state);
                        queue.push_back(new_state);
                    }
                }
                seen.insert(get_key(n), true);
            }
        }
    }
    return states.get(&max).unwrap().loss;
}

fn main() {
    let mut now = Instant::now();
    let (grid, max) = parse_grid("./input.txt");
    println!("grid: {:?}", grid.len());
    println!("max: {:?}", max);
    println!(
        "Part 1: {}",
        solve(
            &grid,
            State {
                x: 0,
                y: 0,
                dir: RIGHT,
                distance: 0,
                loss: 0,
            },
            3,
            max,
        )
    );
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    // println!("Part 2: {}", 0);
    println!("Done in: {:.2?}!", now.elapsed());
}
