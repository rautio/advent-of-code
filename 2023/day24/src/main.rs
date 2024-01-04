use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
struct Pt {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    m: f64,
    b: f64,
    start: Pt,
    vel: Pt,
}

// y = mx + b
// b = y - mx
fn get_line(pt: Pt, vel: Pt) -> Line {
    let m: f64 = vel.y / vel.x;
    let b = pt.y - m * pt.x;
    return Line {
        m: m,
        b: b,
        start: pt,
        vel: vel,
    };
}

fn can_intersect(l1: &Line, l2: &Line) -> bool {
    if l1.m == l2.m {
        return false;
    }
    return true;
}

fn get_dist(a: &Pt, b: &Pt) -> f64 {
    return ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt();
}

fn is_past(p: &Pt, l: &Line) -> bool {
    let after_start = Pt {
        x: l.start.x + l.vel.x,
        y: l.start.y + l.vel.y,
    };
    if get_dist(p, &after_start) > get_dist(p, &l.start) {
        return true;
    }
    return false;
}

// x = (b2-b1)/(m1-m2)
// y = m1x + b1
fn get_intersect(l1: &Line, l2: &Line) -> Pt {
    let mut p = Pt { x: 0.0, y: 0.0 };
    p.x = (l2.b - l1.b) / (l1.m - l2.m);
    p.y = l1.m * p.x + l1.b;
    return p;
}

fn parse_input(input_file: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for line in read_to_string(input_file).unwrap().lines() {
        let splits = line.split("@").collect::<Vec<&str>>();
        let coor = splits[0].trim().split(",").collect::<Vec<&str>>();
        let vel = splits[1].trim().split(",").collect::<Vec<&str>>();
        let p = Pt {
            x: coor[0].trim().parse().unwrap(),
            y: coor[1].trim().parse().unwrap(),
            // z: coor[2].trim().parse().unwrap(),
        };
        let v = Pt {
            x: vel[0].trim().parse().unwrap(),
            y: vel[1].trim().parse().unwrap(),
            // z: vel[2].trim().parse().unwrap(),
        };
        let eq = get_line(p, v);
        lines.push(eq);
    }
    return lines;
}

fn solve_part1(lines: &Vec<Line>, min: Pt, max: Pt) -> usize {
    let mut intersects = 0;
    for (i, l1) in lines.into_iter().enumerate() {
        for l2 in &lines[i + 1..] {
            if can_intersect(l1, l2) {
                let int = get_intersect(l1, l2);
                if int.x >= min.x
                    && int.y >= min.y
                    && int.x <= max.x
                    && int.y <= max.y
                    && !is_past(&int, l1)
                    && !is_past(&int, l2)
                {
                    intersects += 1;
                }
            }
        }
    }
    return intersects;
}

fn main() {
    let mut now = Instant::now();
    let lines = parse_input("./input.txt");
    // let min = Pt { x: 7.0, y: 7.0 };
    // let max = Pt { x: 27.0, y: 27.0 };
    let min = Pt {
        x: 200000000000000.0,
        y: 200000000000000.0,
    };
    let max = Pt {
        x: 400000000000000.0,
        y: 400000000000000.0,
    };
    println!("Part 1: {}", solve_part1(&lines, min, max));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    // println!("Part 2: {}", solve_part2(&bricks));
    // println!("Done in: {:?}!", now.elapsed());
}
