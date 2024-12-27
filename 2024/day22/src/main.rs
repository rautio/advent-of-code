use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

fn prune(num: i64) -> i64 {
    num % 16777216
}

fn iterate(start: i64) -> i64 {
    let mut secret = start;
    let mut res = secret * 64;
    let mut m = mix(res, secret);
    secret = prune(m);
    res = secret / 32;
    m = mix(res, secret);
    secret = prune(m);
    res = secret * 2048;
    m = mix(res, secret);
    secret = prune(m);
    secret
}

fn iterate_secret(start: i64, iterations: i64) -> i64 {
    let mut secret = start;
    for _ in 0..iterations {
        secret = iterate(secret);
    }
    secret
}

fn part1(secrets: &Vec<i64>) -> i64 {
    let mut sum = 0;

    for s in secrets {
        let new_secret = iterate_secret(*s, 2000);
        sum += new_secret;
    }

    sum
}

fn part2(buyers: &Vec<i64>) -> i32 {
    let mut bananas: HashMap<VecDeque<i32>, i32> = HashMap::new();
    for s in buyers {
        let mut buyer_bananas: HashMap<VecDeque<i32>, i32> = HashMap::new();
        let mut pattern: VecDeque<i32> = VecDeque::new();
        let mut secret = *s;
        let mut price = (secret % 10) as i32;
        secret = iterate(secret);
        for _ in 1..2001 {
            let new_price = (secret % 10) as i32;
            pattern.push_back(new_price - price);
            price = new_price;
            if pattern.len() == 4 {
                if !buyer_bananas.contains_key(&pattern) {
                    buyer_bananas.insert(pattern.clone(), new_price);
                }
                pattern.pop_front();
            }
            secret = iterate(secret);
        }
        for (k, v) in buyer_bananas.into_iter() {
            *bananas.entry(k).or_default() += v;
        }
    }

    let mut max = 0;
    for (_, v) in bananas.into_iter() {
        if v > max {
            max = v;
        }
    }
    max
}
fn main() {
    let mut now = Instant::now();
    let mut secrets: Vec<i64> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        secrets.push(line.parse::<i64>().unwrap());
    }
    // Part 1
    println!("Part 1: {}", part1(&secrets));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", part2(&secrets));
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }
    #[test]
    fn test_iterate_secret() {
        assert_eq!(iterate_secret(123, 1), 15887950);
        assert_eq!(iterate_secret(123, 5), 1553684);
        assert_eq!(iterate_secret(123, 10), 5908254);
    }
}
