use std::fs::read_to_string;
use std::time::Instant;

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

fn prune(num: i64) -> i64 {
    num % 16777216
}

fn iterate_secret(start: i64, iterations: i64) -> i64 {
    let mut secret = start;
    for _ in 0..iterations {
        let mut res = secret * 64;
        let mut m = mix(res, secret);
        secret = prune(m);
        res = secret / 32;
        m = mix(res, secret);
        secret = prune(m);
        res = secret * 2048;
        m = mix(res, secret);
        secret = prune(m);
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
    println!("Part 2: {}", 0);
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
