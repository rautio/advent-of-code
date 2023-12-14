use lazy_static::lazy_static;
use memoize::memoize;
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

fn find_arrangements(row: &str, nums: &str, cache: &mut HashMap<String, u32>) -> u32 {
    // let row = strs.join(".");
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
    let mut row_split: VecDeque<&str> = row.split(".").collect::<VecDeque<&str>>();
    row_split.retain(|&s| s != "");
    let mut num_split: VecDeque<&str> = nums.split(",").collect::<VecDeque<&str>>();
    num_split.retain(|&s| s != "");
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
                return find_arrangements(&newr.join("."), &newn.join(","), cache);
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

fn build_possibles(row: &str) -> Vec<String> {
    let mut possibles: Vec<String> = Vec::new();
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(row.to_string());
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        if !cur.chars().any(|c| c == '?') {
            // All clear, add!
            possibles.push(cur);
        } else {
            // There is a ?
            let i = cur.find('?').unwrap();
            // It's a #!
            let mut temp1: Vec<char> = cur.chars().collect();
            if let Some(ch) = temp1.get_mut(i) {
                *ch = '#';
            }
            q.push_back(temp1.into_iter().collect());
            // It's a .!
            let mut temp2: Vec<char> = cur.chars().collect();
            if let Some(ch) = temp2.get_mut(i) {
                *ch = '.';
            }
            q.push_back(temp2.into_iter().collect());
        }
    }
    return possibles;
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
        let mut cache: HashMap<String, u32> = HashMap::new();
        let strs = split_row(row);
        sum_1 += find_arrangements(&strs.join("."), &nums, &mut cache);
        // let itr = 5;
        // let mut strs_repeated = strs.join(".");
        // let mut nums_repeated = String::from(nums);
        // for i in 0..4 {
        //     strs_repeated += "?";
        //     strs_repeated += &strs.join(".");
        //     nums_repeated += ",";
        //     nums_repeated += &nums;
        // }
        // println!("strs_repeated: {:?}", strs_repeated);
        // println!("nums_repeated: {:?}", nums_repeated);
        // let mut cache2: HashMap<String, u32> = HashMap::new();
        // sum_2 += find_arrangements(&strs_repeated, &nums_repeated, &mut cache2)
        // let possibles = build_possibles(row);
        // for p in possibles.iter() {
        //     if is_valid(p, &nums) {
        //         sum += 1;
        //     }
        // }
    }
    println!("Part 1: {}", sum_1);
    // println!("Part 2: {}", sum_2);
    let elapsed = now.elapsed();
    println!("Done in: {:.2?}!", elapsed);
}
