use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Memory {
    id: i32,
    is_free: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct FileBlock {
    mem: Memory,
    index: usize,
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Block {
    id: i32,
    len: i32,
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
        if !cur.is_free {
            if free_index >= cur_index {
                break;
            }
            mem.swap(free_index, cur_index);
        }
    }
}

fn sort_disk(disk: &mut Vec<Block>) {
    let mut rpointer = disk.len() - 1;
    while rpointer > 0 {
        let mut lpointer = 0;
        let right = disk[rpointer];
        if (!right.is_free) {
            while lpointer < rpointer {
                let left = disk[lpointer];
                // Fill and replace
                if left.len > right.len {
                    disk[rpointer] = Block {
                        id: 0,
                        is_free: true,
                        len: right.len,
                    };
                    disk[lpointer] = right;
                    disk.insert(
                        lpointer,
                        Block {
                            id: 0,
                            is_free: true,
                            len: left.len - right.len,
                        },
                    );
                    rpointer += 1; // We expanded the length
                }
                // Swap
                if left.len == right.len {
                    disk.swap(rpointer, lpointer);
                }
                lpointer += 1;
            }
        }
        rpointer -= 1
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

fn checksum_disk(disk: &Vec<Block>) -> i64 {
    let mut mem: Vec<Memory> = Vec::new();
    for (i, f) in disk.into_iter().enumerate() {
        for j in 0..f.len {
            mem.push(Memory {
                id: f.id,
                is_free: f.is_free,
            });
        }
    }

    checksum(&mem)
}

fn main() {
    let mut now = Instant::now();
    let mut memory1: Vec<Memory> = Vec::new();
    let mut files: Vec<File> = Vec::new();
    let mut disk: Vec<Block> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut id = 0;
        for (i, c) in line.chars().into_iter().enumerate() {
            let count = c.to_string().parse::<i32>().unwrap();
            for _ in 0..count {
                memory1.push(Memory {
                    id,
                    is_free: i % 2 != 0,
                })
            }
            disk.push(Block {
                id,
                len: count,
                is_free: i % 2 != 0,
            });
            if i % 2 == 0 {
                // File
                id += 1;
            }
        }
    }
    let mut memory2 = memory1.clone();
    sort_memory(&mut memory1);
    // Part 1
    println!("Part 1: {}", checksum(&memory1));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    sort_disk(&mut disk);
    println!("Part 2: {}", checksum_disk(&disk));
    println!("Done in: {:?}!", now.elapsed());
}
