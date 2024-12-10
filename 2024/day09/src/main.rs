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

fn sort_files(mem: &mut Vec<Memory>) {
    let mut cur_file: Vec<FileBlock> = Vec::new();
    let mut moved: HashMap<i32, bool> = HashMap::new();
    let mut cur_id = -1; // Starting at the right guarantees non-zero starting id
    for i in 1..=mem.len() {
        // If new index or free, check if we have a file
        let cur_index = mem.len() - i;
        let cur = mem[cur_index];
        if (cur.is_free || cur.id != cur_id) && cur_file.len() > 0 {
            // We have a block, check for matches.
            if cur_file.len() > 0 && !moved.contains_key(&cur_id) {
                let mut free_space: Vec<FileBlock> = Vec::new();
                for j in 0..cur_index + 1 {
                    let block = mem[j];
                    if block.is_free {
                        free_space.push(FileBlock {
                            mem: block,
                            index: j,
                        });
                    } else {
                        if free_space.len() > 0 && free_space.len() >= cur_file.len() {
                            for c in 0..cur_file.len() {
                                let cur_idx = cur_file[c].index;
                                let free_idx = free_space[c].index;
                                if free_idx >= cur_idx {
                                    break;
                                }
                                mem.swap(free_idx, cur_idx);
                            }
                            break;
                        }
                        free_space = Vec::new();
                    }
                }
                moved.insert(cur_id, true);
            }
            // Reset block
            cur_file = Vec::new();
        }
        if !cur.is_free {
            cur_file.push(FileBlock {
                mem: cur,
                index: cur_index,
            });
            cur_id = cur.id;
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
    let mut memory1: Vec<Memory> = Vec::new();
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
    sort_files(&mut memory2);
    println!("Part 2: {}", checksum(&memory2));
    println!("Done in: {:?}!", now.elapsed());
}
