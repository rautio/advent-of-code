use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Beam {
    x: i32,
    y: i32,
    dir: Pt,
}

const UP: Pt = Pt { x: 0, y: -1 };
const DOWN: Pt = Pt { x: 0, y: 1 };
const LEFT: Pt = Pt { x: -1, y: 0 };
const RIGHT: Pt = Pt { x: 1, y: 0 };

fn get_next_beams(next: &Pt, dir: Pt, c: char) -> Vec<Beam> {
    let mut beams: Vec<Beam> = Vec::new();
    if c == '|' {
        if dir == RIGHT || dir == LEFT {
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: UP,
            });
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: DOWN,
            });
        } else {
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: dir,
            });
        }
    } else if c == '-' {
        if dir == UP || dir == DOWN {
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: LEFT,
            });
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: RIGHT,
            });
        } else {
            beams.push(Beam {
                x: next.x,
                y: next.y,
                dir: dir,
            });
        }
    } else if c == '/' {
        let mut d = UP;
        if dir == UP {
            d = RIGHT;
        } else if dir == DOWN {
            d = LEFT;
        } else if dir == RIGHT {
            d = UP;
        } else if dir == LEFT {
            d = DOWN;
        }
        beams.push(Beam {
            x: next.x,
            y: next.y,
            dir: d,
        });
    } else if c == '\\' {
        let mut d = UP;
        if dir == UP {
            d = LEFT;
        } else if dir == DOWN {
            d = RIGHT;
        } else if dir == RIGHT {
            d = DOWN;
        } else if dir == LEFT {
            d = UP;
        }
        beams.push(Beam {
            x: next.x,
            y: next.y,
            dir: d,
        });
    } else if c == '.' {
        beams.push(Beam {
            x: next.x,
            y: next.y,
            dir: dir,
        })
    }
    return beams;
}

fn walk_grid(grid: HashMap<Pt, char>, max_x: usize, max_y: usize) -> HashMap<Pt, bool> {
    let mut energized: HashMap<Pt, bool> = HashMap::new();
    let mut seen: HashMap<Beam, bool> = HashMap::new();
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(Beam {
        x: -1,
        y: 0,
        dir: RIGHT,
    });
    let mut i = 0;
    energized.insert(Pt { x: 0, y: 0 }, true);
    let mut prev: Beam = Beam {
        x: -1,
        y: 0,
        dir: RIGHT,
    };
    while beams.len() > 0 {
        let b = beams.pop_front().unwrap();
        let next = Pt {
            x: b.x + b.dir.x,
            y: b.y + b.dir.y,
        };
        if next.x >= 0
            && next.y >= 0
            && next.x <= max_x as i32
            && next.y <= max_y as i32
            && !seen.contains_key(&b)
        {
            energized.insert(next, true);
            seen.insert(b, true);
            let c = grid.get(&next).unwrap();
            let next_beams = get_next_beams(&next, b.dir, *c);
            for nb in next_beams {
                beams.push_back(nb);
            }
        }
    }
    return energized;
}

fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in read_to_string("./input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
    {
        if y > max_y {
            max_y = y;
        }
        for (x, c) in line.chars().into_iter().enumerate() {
            if x > max_x {
                max_x = x;
            }
            grid.insert(
                Pt {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        }
    }
    let energized = walk_grid(grid, max_x, max_y);
    println!("Part 1: {}", energized.len());
    // println!("Part 2: {}", sum_boxes(boxes));
    println!("Done in: {:.2?}!", now.elapsed());
}
