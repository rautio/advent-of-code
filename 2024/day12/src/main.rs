use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Garden {
    pts: Vec<Pt>,
    plant: char,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
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

fn find_garden(start: Pt, grid: &HashMap<Pt, char>) -> Garden {
    let ch = *grid.get(&start).unwrap();
    let mut seen: HashMap<Pt, bool> = HashMap::new();
    let mut check = VecDeque::from([start]);
    seen.insert(start, true);

    while check.len() > 0 {
        let cur = check.pop_front().unwrap();
        let next: Vec<Pt> = vec![N, E, S, W]
            .into_iter()
            .map(|p| add(p, cur))
            .filter(|p| {
                grid.contains_key(&p) && !seen.contains_key(&p) && *grid.get(&p).unwrap() == ch
            })
            .collect();
        for n in next {
            check.push_back(n);
            seen.insert(n, true);
        }
    }

    Garden {
        pts: seen.keys().cloned().collect(),
        plant: ch,
    }
}

fn find_gardens(grid: &HashMap<Pt, char>) -> Vec<Garden> {
    let mut gardens: Vec<Garden> = Vec::new();
    let mut seen: HashMap<Pt, bool> = HashMap::new();

    for (pt, _) in grid.into_iter() {
        if !seen.contains_key(&pt) {
            // It's a new garden!
            let garden = find_garden(*pt, grid);
            for p in garden.pts.clone() {
                seen.insert(p, true);
            }
            gardens.push(garden);
        }
    }

    gardens
}

fn calc_perimeter(garden: Garden) -> usize {
    let mut perimeter = 0;
    let pts = garden.pts.clone();
    for cur in &pts {
        let neighbors = vec![N, E, S, W].into_iter().map(|p| add(p, *cur));
        let mut per = 4;
        for n in neighbors {
            if pts.contains(&n) {
                per -= 1;
            }
        }
        perimeter += per;
    }
    perimeter
}

fn fence_cost(grid: &HashMap<Pt, char>) -> usize {
    let gardens = find_gardens(grid);
    let mut total = 0;

    for garden in gardens {
        let perimeter = calc_perimeter(garden.clone());
        let area = garden.pts.len();
        total += perimeter * area;
    }

    total
}

fn main() {
    let now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut y = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for ch in l {
            grid.insert(Pt { x, y }, ch);
            x += 1;
        }
        y += 1;
    }
    println!("Part 1: {}", fence_cost(&grid));
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
