use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Lens {
    l: String,
    n: u32,
}

fn get_hash(s: &str) -> u32 {
    let mut cur = 0;
    for c in s.chars() {
        cur += c as u32;
        cur *= 17;
        let r = cur % 256;
        cur = r;
    }
    return cur;
}

fn sum_boxes(boxes: Vec<Vec<Lens>>) -> u32 {
    let mut sum: u32 = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (s, lens) in b.into_iter().enumerate() {
            sum += (i as u32 + 1) * (s as u32 + 1) * lens.n;
        }
    }
    return sum;
}

fn main() {
    let now = Instant::now();
    let mut sum = 0;
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _i in 0..256 {
        boxes.push(Vec::new());
    }
    for line in read_to_string("./input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(",").collect();
        for s in splits {
            // Part 1
            let h = get_hash(s);
            sum += h;
            // Part 2
            if s.chars().any(|c| c == '=') {
                let splts: Vec<&str> = s.split("=").collect();
                let idx = get_hash(splts[0]) as usize;
                let l = Lens {
                    l: splts[0].to_string(),
                    n: splts[1].parse::<u32>().unwrap(),
                };
                let vec = &boxes[idx];
                let mut new_vec: Vec<Lens> = Vec::new();
                let mut found: bool = false;
                for v in vec {
                    let lc = l.clone();
                    if v.l == lc.l {
                        new_vec.push(lc);
                        found = true;
                    } else {
                        new_vec.push(v.clone());
                    }
                }
                if !found {
                    new_vec.push(l);
                }
                boxes[idx] = new_vec;
            } else {
                // Otherwise its -
                let splts: Vec<&str> = s.split("-").collect();
                let idx = get_hash(splts[0]) as usize;
                let label = splts[0].to_string();
                let vec = &boxes[idx];
                let mut new_vec: Vec<Lens> = Vec::new();
                for v in vec {
                    if v.l != label {
                        new_vec.push(v.clone());
                    }
                }
                boxes[idx] = new_vec;
            }
        }
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_boxes(boxes));
    println!("Done in: {:.2?}!", now.elapsed());
}
