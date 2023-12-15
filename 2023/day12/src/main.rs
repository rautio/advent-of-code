use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref DOTS: Regex = Regex::new(r"(\.)+").unwrap();
}

fn split_row(row: &str) -> Vec<&str> {
    let mut strs: Vec<&str> = DOTS.split(row).collect();
    strs.retain(|&s| s != "");
    return strs;
}

fn create_key(row: &str, nums: &str) -> String {
    let mut s = String::from(row);
    s.push('-');
    s += nums;
    return s;
}

fn find_arrangements(r: &str, nums: &str, cache: &mut HashMap<String, u64>) -> u64 {
    let row_split_raw = split_row(r);
    let row = row_split_raw.join(".");
    let key = create_key(&row, nums);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    // There are no more unknowns
    if !row.chars().any(|c| c == '?') {
        if is_valid(&row, nums) {
            cache.insert(key, 1);
            return 1;
        }
        cache.insert(key, 0);
        return 0;
    }
    let mut row_split: VecDeque<&str> = VecDeque::from(row_split_raw);
    row_split.retain(|&s| s != "");
    let count_hashes = row.matches("#").count() as u32;
    let mut num_split: VecDeque<&str> = nums.split(",").collect::<VecDeque<&str>>();
    num_split.retain(|&s| s != "");
    let num_total: u32 = num_split.iter().map(|&i| i.parse::<u32>().unwrap()).sum();
    if count_hashes > num_total {
        cache.insert(key, 0);
        return 0;
    }
    if num_split.len() > 0 && row_split.len() > 0 {
        let fs = row_split.pop_front().unwrap();
        let ns = num_split.pop_front().unwrap();
        if !fs.chars().any(|c| c == '?') {
            if fs.len() as u32 != ns.parse().unwrap() {
                cache.insert(key, 0);
                return 0;
            } else if num_split.len() > 0 && row_split.len() > 0 {
                // The first one matches!
                let newr: Vec<&str> = row_split.into();
                let newn: Vec<&str> = num_split.into();
                let c = find_arrangements(&newr.join("."), &newn.join(","), cache);
                cache.insert(key, c);
                return c;
            }
        }
    }
    let mut sum = 0;
    // Dot '.'
    let mut dot_row = row.replacen("?", ".", 1);
    dot_row = dot_row.trim_start_matches(".").to_string();
    let dots = find_arrangements(&dot_row, nums, cache);
    cache.insert(create_key(&dot_row, nums), dots);
    sum += dots;
    // Hash '#'
    let hash_row = row.replacen("?", "#", 1);
    let hashes = find_arrangements(&hash_row, nums, cache);
    cache.insert(create_key(&hash_row, nums), hashes);
    sum += hashes;
    return sum;
}

fn is_valid(s: &str, nums: &str) -> bool {
    let groups: Vec<u32> = nums
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let working_groups: Vec<&str> = s.split(|c| c != '#').filter(|s| !s.is_empty()).collect();
    if working_groups.len() != groups.len() {
        return false;
    }
    for i in 0..working_groups.len() {
        if working_groups[i].len() as u32 != groups[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let now = Instant::now();
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        let s = line.split(' ').collect::<Vec<&str>>();
        let row = s[0];
        let nums = s[1];
        let mut cache: HashMap<String, u64> = HashMap::new();
        let strs = split_row(row);
        sum_1 += find_arrangements(&strs.join("."), &nums, &mut cache);
        let mut row_repeated = String::from(row);
        let mut nums_repeated = String::from(nums);
        for _i in 0..4 {
            row_repeated += "?";
            row_repeated += &row;
            nums_repeated += ",";
            nums_repeated += &nums;
        }
        let strs_repeated = split_row(&row_repeated);
        sum_2 += find_arrangements(&strs_repeated.join("."), &nums_repeated, &mut cache);
    }
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
