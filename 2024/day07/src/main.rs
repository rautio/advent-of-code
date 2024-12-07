use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Eq {
    result: i64,
    numbers: Vec<i64>,
    total: i64,
}

fn could_be_true(eq: &Eq, concatenate: bool) -> bool {
    let result = eq.result;
    let mut nums = eq.numbers.clone();
    let total = eq.total;
    if nums.len() == 0 {
        return result == total;
    }
    let next = nums.remove(0);
    let add = Eq {
        result: result,
        numbers: nums.clone(),
        total: total + next,
    };
    let mut mul_total = total;
    if mul_total == 0 {
        mul_total = 1;
    }
    let mul = Eq {
        result: result,
        numbers: nums.clone(),
        total: mul_total * next,
    };
    let mut concat_possible = false;
    if concatenate && total > 0 {
        let concat_nums = nums.clone();
        let new_num = vec![total.to_string(), next.to_string()]
            .join("")
            .parse::<i64>()
            .unwrap();

        let mut new_nums: VecDeque<i64> = VecDeque::from(concat_nums);
        new_nums.push_front(new_num);
        let concat = Eq {
            result,
            numbers: Vec::from(new_nums),
            total: 0,
        };
        concat_possible = could_be_true(&concat, concatenate);
    }
    return could_be_true(&add, concatenate) || could_be_true(&mul, concatenate) || concat_possible;
}

fn count_true(eqs: &Vec<Eq>, concatenate: bool) -> i64 {
    let mut count = 0;

    for eq in eqs {
        if could_be_true(eq, concatenate) {
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
            total: 0,
        })
    }
    // Part 1
    println!("Part 1: {}", count_true(&eqs, false));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", count_true(&eqs, true));
    println!("Done in: {:?}!", now.elapsed());
}
