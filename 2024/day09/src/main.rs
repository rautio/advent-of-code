use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Memory {
    id: i32,
    is_free: bool,
}

fn print_memory(mem: &Vec<Memory>) {
    let mut res = String::new();
    for m in mem {
        if m.is_free {
            res.push_str(".");
        } else {
            res.push_str(&m.id.to_string());
        }
    }
    println!("{}", res);
}

fn sort_memory(mem: &mut Vec<Memory>) {
    for i in 1..mem.len() {
        let cur_index = mem.len() - i;
        let cur = &mem[cur_index];
        let free_index = mem.iter().position(|&r| r.is_free).unwrap();
        if !cur.is_free && free_index < cur_index {
            mem.swap(free_index, cur_index);
        }
    }
}

fn checksum(mem: &Vec<Memory>) -> i64 {
    let mut sum: i64 = 0;

    for (i, m) in mem.into_iter().enumerate() {
        if !m.is_free {
            sum += (i as i64) * (m.id as i64);
        }
    }

    sum
}

fn main() {
    let mut now = Instant::now();
    let mut input: Vec<Memory> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut id = 0;
        for (i, c) in line.chars().into_iter().enumerate() {
            let count = c.to_string().parse::<i32>().unwrap();
            for _ in 0..count {
                input.push(Memory {
                    id,
                    is_free: i % 2 != 0,
                })
            }
            if i % 2 == 0 {
                // File
                id += 1;
            }
        }
    }
    sort_memory(&mut input);
    // Part 1
    println!("Part 1: {}", checksum(&input));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
