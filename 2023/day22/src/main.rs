use std::collections::HashMap;
use std::fs::read_to_string;
use std::mem;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Brick {
    start: Pt,
    end: Pt,
    dir: Pt,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pt {
    x: u32,
    y: u32,
    z: u32,
}

const X: Pt = Pt { x: 1, y: 0, z: 0 };
const Y: Pt = Pt { x: 0, y: 1, z: 0 };
const Z: Pt = Pt { x: 0, y: 0, z: 1 };

fn add_pt(a: &Pt, b: &Pt) -> Pt {
    return Pt {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    };
}

fn parse_input(input_file: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for line in read_to_string(input_file).unwrap().lines() {
        let splits = line.split("~").collect::<Vec<&str>>();
        let s = splits[0].split(",").collect::<Vec<&str>>();
        let e = splits[1].split(",").collect::<Vec<&str>>();
        let start = Pt {
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
            z: s[2].parse().unwrap(),
        };
        let end = Pt {
            x: e[0].parse().unwrap(),
            y: e[1].parse().unwrap(),
            z: e[2].parse().unwrap(),
        };
        // Start coordinate is always "lower" than the end coordinate
        let mut dir = X;
        if start.y != end.y {
            dir = Y;
        }
        if start.z != end.z {
            dir = Z;
        }
        let brick = Brick {
            start: start.clone(),
            end: end.clone(),
            dir: dir.clone(),
        };
        bricks.push(brick);
    }
    // Have lowest z index first
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    return bricks;
}

fn can_move(brick: &Brick, grid: &HashMap<Pt, usize>) -> bool {
    if brick.dir == Z {
        return brick.start.z > 1
            && !grid.contains_key(&Pt {
                x: brick.start.x,
                y: brick.start.y,
                z: brick.start.z - 1,
            });
    }
    if brick.dir == Y {
        for y in brick.start.y..brick.end.y + 1 {
            let new_pt = Pt {
                x: brick.start.x,
                y: y,
                z: brick.start.z - 1,
            };
            if new_pt.z < 1 || grid.contains_key(&new_pt) {
                return false;
            }
        }
        return true;
    }
    // Else its X
    for x in brick.start.x..brick.end.x + 1 {
        let new_pt = Pt {
            x: x,
            y: brick.start.y,
            z: brick.start.z - 1,
        };
        if new_pt.z < 1 || grid.contains_key(&new_pt) {
            return false;
        }
    }
    return true;
}

fn move_brick(
    index: usize,
    b: Vec<Brick>,
    g: HashMap<Pt, usize>,
) -> (Vec<Brick>, HashMap<Pt, usize>) {
    let mut bricks = b.clone();
    let mut grid = g.clone();
    // Delete in hashmap
    let mut brick = bricks[index].clone();
    let mut iter = brick.start.clone();
    while iter != add_pt(&brick.end, &brick.dir.clone()) {
        grid.remove(&iter);
        iter = add_pt(&iter, &brick.dir);
    }
    brick.start.z -= 1;
    brick.end.z -= 1;
    iter = brick.start.clone();
    while iter != add_pt(&brick.end, &brick.dir.clone()) {
        grid.insert(iter.clone(), index);
        iter = add_pt(&iter, &brick.dir);
    }
    let _old = mem::replace(&mut bricks[index], brick);
    // Add back in hashmap
    return (bricks, grid);
}

fn settle_bricks(b: &Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut bricks = b.clone();
    let mut grid: HashMap<Pt, usize> = HashMap::new();
    let mut moved_bricks: HashMap<usize, bool> = HashMap::new();

    for (i, brick) in b.into_iter().enumerate() {
        let mut iter = brick.start.clone();
        while iter != add_pt(&brick.end, &brick.dir.clone()) {
            grid.insert(iter.clone(), i);
            iter = add_pt(&iter, &brick.dir.clone());
        }
    }

    let mut moved = true;
    while moved {
        let mut had_move = false;
        for (i, brick) in bricks.clone().into_iter().enumerate() {
            if can_move(&brick, &grid) {
                had_move = true;
                moved_bricks.insert(i, true);
                let (new_bricks, new_grid) = move_brick(i, bricks, grid);
                bricks = new_bricks;
                grid = new_grid;
            }
        }
        moved = had_move;
    }
    return (bricks, moved_bricks.len());
}

fn solve_part1(bricks: &Vec<Brick>) -> u32 {
    let (settled_bricks, _moves) = settle_bricks(bricks);
    let mut removable_bricks = 0;
    for i in 0..settled_bricks.len() {
        let mut new_bricks = settled_bricks.clone();
        new_bricks.remove(i);
        let (_b, moves) = settle_bricks(&new_bricks);
        if moves == 0 {
            removable_bricks += 1;
        }
    }
    return removable_bricks;
}

fn solve_part2(bricks: &Vec<Brick>) -> usize {
    let (settled_bricks, _moves) = settle_bricks(bricks);
    let mut moved_bricks = 0;
    for i in 0..settled_bricks.len() {
        let mut new_bricks = settled_bricks.clone();
        new_bricks.remove(i);
        let (_b, moves) = settle_bricks(&new_bricks);
        moved_bricks += moves;
    }
    return moved_bricks;
}
fn main() {
    let mut now = Instant::now();
    let bricks = parse_input("./input.txt");
    println!("Part 1: {}", solve_part1(&bricks));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {}", solve_part2(&bricks));
    println!("Done in: {:?}!", now.elapsed());
}
