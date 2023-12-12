use std::fs::read_to_string;
use std::time::Instant;

fn find_next(nums: &Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = Vec::new();
    let mut all_zero = true;
    let mut diff = 0;
    let len = nums.len();
    for (i, n) in nums.into_iter().enumerate() {
        if i == len - 1 {
            continue;
        }
        diff = nums[i + 1] - n;
        diffs.push(diff);
        if diff != 0 {
            all_zero = false;
        }
    }
    if all_zero {
        return nums[&len - 1];
    }
    let res = find_next(&diffs);
    return nums[&len - 1] + res;
}

fn main() {
    let mut now = Instant::now();
    let mut sum = 0;
    // Parse all the numbers and parts
    for line in read_to_string("./input.txt").unwrap().lines() {
        let mut nums: Vec<i32> = Vec::new();
        for (_i, n) in line.split(' ').enumerate() {
            nums.push(n.parse::<i32>().unwrap());
        }
        sum += find_next(&nums);
    }
    // Part 1
    println!("Part 1: {}", sum);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
