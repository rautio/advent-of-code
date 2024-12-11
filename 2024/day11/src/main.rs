use std::fs::read_to_string;
use std::time::Instant;

fn blink(stones: Vec<i64>) -> Vec<i64> {
    let mut res = stones.clone();
    let mut i = 0;
    while i < res.len() {
        let stone = res[i];
        let stone_str = stone.to_string();
        if stone == 0 {
            res[i] = 1;
        } else if stone_str.len() % 2 == 0 {
            let (carve1, carve2) = stone_str.split_at(stone_str.len() / 2);
            res[i] = carve1.parse::<i64>().unwrap();
            res.insert(i + 1, carve2.parse::<i64>().unwrap());
            i += 1;
        } else {
            res[i] = stone * 2024;
        }
        i += 1;
    }
    res
}

fn blink_times(stones: Vec<i64>, num_blinks: i64) -> Vec<i64> {
    let mut new_stones = stones.clone();

    for _ in 0..num_blinks {
        new_stones = blink(new_stones);
    }

    new_stones
}

fn main() {
    let mut now = Instant::now();
    let mut stones: Vec<i64> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        stones = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    }
    // Part 1
    println!("Part 1: {}", blink_times(stones.clone(), 25).len());
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
