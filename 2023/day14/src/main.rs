use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

fn rotate(r: Vec<Pt>, cubed: &HashMap<Pt, bool>, max: Pt, dir: Pt) -> Vec<Pt> {
    let mut new: Vec<Pt> = Vec::new();
    let mut rounded: Vec<Pt> = r;
    let mut new_rounded: HashMap<Pt, bool> = HashMap::new();
    // Sort rounded in the direction we're rotating
    if dir.y != 0 {
        rounded.sort_by(|a, b| a.x.cmp(&b.x));
        if dir.y == -1 {
            rounded.sort_by(|a, b| a.y.cmp(&b.y));
        } else {
            rounded.sort_by(|a, b| b.y.cmp(&a.y));
        }
    }
    if dir.x != 0 {
        rounded.sort_by(|a, b| a.y.cmp(&b.y));
        if dir.x == -1 {
            rounded.sort_by(|a, b| a.x.cmp(&b.x));
        } else {
            rounded.sort_by(|a, b| b.x.cmp(&a.x));
        }
    }
    for r in rounded {
        let mut new_r = r;
        loop {
            let next = Pt {
                x: new_r.x + dir.x,
                y: new_r.y + dir.y,
            };
            if cubed.contains_key(&next)
                || new_rounded.contains_key(&next)
                || next.x < 0
                || next.x > max.x
                || next.y < 0
                || next.y > max.y
            {
                break;
            }
            new_r = next;
        }
        new.push(new_r);
        if new_rounded.contains_key(&new_r) {
            panic!("overlapping rocks: {:?} ", new_r);
        }
        new_rounded.insert(new_r, true);
    }
    return new;
}

fn rotate_north(rounded: Vec<Pt>, cubed: &HashMap<Pt, bool>, max: Pt) -> Vec<Pt> {
    return rotate(rounded, cubed, max, Pt { x: 0, y: -1 });
}

fn weigh_load(rounded: &Vec<Pt>, max: Pt) -> i32 {
    let mut sum = 0;
    for r in rounded {
        sum += (max.y - r.y) + 1;
    }
    return sum;
}

fn create_key(pts: &Vec<Pt>, dir: Pt) -> String {
    let mut s = String::from("");
    s += &dir.x.to_string();
    s.push('-');
    s += &dir.y.to_string();
    s.push(':');
    for p in pts {
        s += &p.x.to_string();
        s.push(',');
        s += &p.y.to_string();
        s.push('&');
    }
    return s;
}

fn _print_rocks(rounded: Vec<Pt>, cubed: &HashMap<Pt, bool>, max: Pt) {
    let mut round: HashMap<Pt, bool> = HashMap::new();
    for r in rounded {
        if round.contains_key(&r) {
            panic!("Overlapping rocks: {:?}", r);
        }
        round.insert(r, true);
    }
    for y in 0..max.y + 1 {
        let mut line = String::from("");
        for x in 0..max.x + 1 {
            let p = Pt { x: x, y: y };
            if round.contains_key(&p) {
                line.push('O');
            } else if cubed.contains_key(&p) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn main() {
    let mut now = Instant::now();
    let mut rounded: Vec<Pt> = Vec::new();
    let mut rotated: Vec<Pt> = Vec::new();
    let mut max = Pt { x: 0, y: 0 };
    let mut cubed: HashMap<Pt, bool> = HashMap::new();
    for (_y, line) in read_to_string("./input.txt")
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
            if c == 'O' {
                rounded.push(Pt { x: x, y: y });
                rotated.push(Pt { x: x, y: y });
            }
            if c == '#' {
                cubed.insert(Pt { x: x, y: y }, true);
            }
        }
    }
    let north = rotate_north(rounded, &cubed, max);
    println!("Part 1: {}", weigh_load(&north, max));
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    now = Instant::now();
    // Part 2
    let dirs = vec![
        Pt { x: 0, y: -1 }, // north
        Pt { x: -1, y: 0 }, // west
        Pt { x: 0, y: 1 },  // south
        Pt { x: 1, y: 0 },  // east
    ];
    let mut seen: HashMap<String, usize> = HashMap::new();
    let l = 1000000000;
    let mut i = 0;
    while i < l {
        for dir in &dirs {
            rotated = rotate(rotated, &cubed, max, *dir);
        }
        let key = create_key(&rotated, Pt { x: 0, y: 0 });
        if seen.contains_key(&key) {
            // We found a repeating pattern
            let prev_i = seen.get(&key).unwrap();
            let diff = i - prev_i;
            let remaining = l - i;
            let times = remaining / diff;
            i = i + diff * times;
        }
        seen.insert(key.clone(), i);
        i += 1;
    }
    println!("Part 2: {}", weigh_load(&rotated, max));
    println!("Done in: {:.2?}!", now.elapsed());
}
