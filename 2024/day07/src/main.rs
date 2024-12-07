use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Eq {
    result: i64,
    numbers: Vec<i64>,
}

fn could_be_true(eq: &Eq) -> bool {
    let result = eq.result;
    let mut nums = eq.numbers.clone();
    if nums.len() == 0 {
        return result == 0;
    }
    if nums.len() == 1 {
        return result == nums[0];
    }
    // Can add or multiply
    let last = nums.pop().unwrap();
    let add = Eq {
        result: result - last,
        numbers: nums.clone(),
    };
    let is_divisible = result % last == 0; // If not the number gets cast to i64 anyway
    let mul = Eq {
        result: result / last,
        numbers: nums.clone(),
    };
    return could_be_true(&add) || (is_divisible && could_be_true(&mul));
}

fn count_true(eqs: &Vec<Eq>) -> i64 {
    let mut count = 0;

    for eq in eqs {
        if could_be_true(eq) {
            count += eq.result;
        }
    }

    count
}

fn main() {
    let mut now = Instant::now();
    let binding = read_to_string("./src/input.txt").unwrap();
    let mut eqs: Vec<Eq> = Vec::new();
    for line in binding.lines() {
        let mut s = line.split(':');
        let result = s.next().unwrap().parse::<i64>().unwrap();
        let nums: Vec<i64> = s
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        eqs.push(Eq {
            result,
            numbers: nums,
        })
    }
    // Part 1
    println!("Part 1: {}", count_true(&eqs));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
