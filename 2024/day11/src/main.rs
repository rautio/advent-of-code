use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn blink_once(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (carve1, carve2) = stone_str.split_at(stone_str.len() / 2);
        return vec![
            carve1.parse::<usize>().unwrap(),
            carve2.parse::<usize>().unwrap(),
        ];
    }
    vec![stone * 2024]
}

fn blink_all(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut res: HashMap<usize, usize> = HashMap::new();

    for stone in stones {
        let new_stones = blink_once(stone.0);
        for new_stone in new_stones {
            res.entry(new_stone)
                .and_modify(|count| *count += stone.1)
                .or_insert(stone.1);
        }
    }

    res
}

fn blink_times(stones: HashMap<usize, usize>, num_blinks: usize) -> HashMap<usize, usize> {
    let mut freqs = stones.clone();
    for _ in 0..num_blinks {
        freqs = blink_all(freqs);
    }
    freqs
}

fn main() {
    let mut now = Instant::now();
    let mut stones: Vec<usize> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        stones = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }
    let freqs: HashMap<usize, usize> =
        stones
            .iter()
            .copied()
            .fold(HashMap::default(), |mut map, val| {
                map.entry(val).and_modify(|count| *count += 1).or_insert(1);
                map
            });
    // Part 1
    println!(
        "Part 1: {}",
        blink_times(freqs.clone(), 25).values().sum::<usize>()
    );
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2:{}", blink_times(freqs, 75).values().sum::<usize>());
    println!("Done in: {:?}!", now.elapsed());
}
