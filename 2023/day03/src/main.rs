use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Number {
    x: u32,
    y: u32,
    num: String,
}

fn create_key(x: u32, y: u32) -> String {
    let mut res = String::new();
    res.push_str(&x.to_string());
    res.push_str(&String::from("-"));
    res.push_str(&y.to_string());
    return res;
}

fn main() {
    // Part 1
    let mut sum = 0;
    let mut nums = Vec::new();
    let mut y = 0;
    let mut x = 0;
    let mut parts = HashMap::new();
    let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
    // Parse all the numbers and parts
    for line in read_to_string("./input.txt").unwrap().lines() {
        let mut n = Number {
            x: 0,
            y: 0,
            num: String::new(),
        };
        let l: Vec<char> = line.chars().collect();
        for c in l {
            if c.is_digit(10) {
                if n.num == "" {
                    n.x = x;
                    n.y = y;
                    n.num = c.to_string();
                } else {
                    n.num.push(c);
                }
            } else if n.num != "" {
                nums.push(n);
                n = Number {
                    x: 0,
                    y: 0,
                    num: String::new(),
                };
            }
            if c != '.' && !c.is_digit(10) {
                parts.insert(create_key(x, y), c);
                if c == '*' {
                    gears.insert(create_key(x, y), Vec::new());
                }
            }
            x += 1;
        }
        if n.num != "" {
            nums.push(n);
        }
        x = 0;
        y += 1;
    }
    // Determine which numbers aren't associated with parts.
    for n in &nums {
        let mut keys: Vec<String> = Vec::new();
        // Start
        if n.x > 0 {
            if n.y > 0 {
                keys.push(create_key(n.x - 1, n.y - 1));
            }
            keys.push(create_key(n.x - 1, n.y));
            keys.push(create_key(n.x - 1, n.y + 1));
        }
        // Middle
        for i in 0..n.num.len() {
            if n.y > 0 {
                keys.push(create_key(n.x + i as u32, n.y - 1));
            }
            keys.push(create_key(n.x + i as u32, n.y + 1));
        }
        // End
        if n.y > 0 {
            keys.push(create_key(n.x + n.num.len() as u32, n.y - 1));
        }
        keys.push(create_key(n.x + n.num.len() as u32, n.y));
        keys.push(create_key(n.x + n.num.len() as u32, n.y + 1));
        // Test all keys
        for k in keys {
            if parts.contains_key(&k) {
                let new_num = n.num.parse::<u32>().unwrap();
                sum += new_num;
                let c = parts.get(&k).unwrap();
                if *c == '*' {
                    // It's a gear
                    gears.entry(k).or_insert_with(Vec::new).push(new_num);
                }
                break;
            }
        }
    }
    println!("Part 1: {}", sum);
    sum = 0;
    for g in gears.values() {
        if g.len() == 2 {
            sum += g[0] * g[1]
        }
    }
    println!("Part 2: {}", sum);
}
