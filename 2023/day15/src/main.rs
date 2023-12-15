use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

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

fn main() {
    let mut now = Instant::now();
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(",").collect();
        for s in splits {
            let h = get_hash(s);
            sum += h;
        }
    }
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
    now = Instant::now();
    // println!("Part 2: {}", weigh_load(&rotated, max));
    // println!("Done in: {:.2?}!", now.elapsed());
}
