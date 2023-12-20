use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn match_cond(cur: u32, comp: String, amount: u32) -> bool {
    if comp == "<" {
        return cur < amount;
    }
    return cur > amount;
}

fn sum_accepted(parts: Vec<Part>) -> u32 {
    let mut sum = 0;
    for p in parts {
        sum += p.x;
        sum += p.m;
        sum += p.s;
        sum += p.a;
    }
    return sum;
}

fn get_accepted_parts(parts: Vec<Part>, workflows: HashMap<String, String>) -> Vec<Part> {
    let mut accepted: Vec<Part> = Vec::new();
    let test_re: Regex = Regex::new(r"(x|m|a|s)(<|>)(.+)*").unwrap();
    for p in parts {
        let mut cur = String::from("in");
        while cur != "A" && cur != "R" {
            let flow = workflows.get(&cur).unwrap();
            let conditions = flow.split(",");
            for c in conditions {
                if c.contains(":") {
                    let splits: Vec<&str> = c.split(":").collect();
                    let test = splits[0];
                    let res = splits[1].to_string();
                    let test_cap = test_re.captures(test).unwrap();
                    let sub_part = &test_cap[1];
                    let comp = test_cap[2].to_string();
                    let amount = test_cap[3].parse().unwrap();
                    match sub_part {
                        "x" => {
                            if match_cond(p.x, comp, amount) {
                                cur = res;
                                break;
                            }
                        }
                        "m" => {
                            if match_cond(p.m, comp, amount) {
                                cur = res;
                                break;
                            }
                        }
                        "a" => {
                            if match_cond(p.a, comp, amount) {
                                cur = res;
                                break;
                            }
                        }
                        "s" => {
                            if match_cond(p.s, comp, amount) {
                                cur = res;
                                break;
                            }
                        }
                        _ => {}
                    }
                } else {
                    // End!
                    cur = c.to_string();
                }
            }
        }
        if cur == "A" {
            accepted.push(p);
        }
    }
    return accepted;
}

fn main() {
    let now = Instant::now();
    let mut workflows: HashMap<String, String> = HashMap::new();
    let mut cap_work = true;
    let workflow_re: Regex = Regex::new(r"(.+)*\{(.+)*\}").unwrap();
    let part_re: Regex = Regex::new(r"\{(.+)*\}").unwrap();
    let mut parts: Vec<Part> = Vec::new();
    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            cap_work = false;
        } else if cap_work {
            let cap = workflow_re.captures(line).unwrap();
            let name = String::from(&cap[1]);
            let conditions = String::from(&cap[2]);
            workflows.insert(name, conditions);
        } else {
            let cap = part_re.captures(line).unwrap();
            let mut new_part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            for p in String::from(&cap[1]).split(",") {
                let part: Vec<&str> = p.split("=").collect();
                match part[0] {
                    "x" => {
                        new_part.x = part[1].parse().unwrap();
                    }
                    "m" => {
                        new_part.m = part[1].parse().unwrap();
                    }
                    "a" => {
                        new_part.a = part[1].parse().unwrap();
                    }
                    "s" => {
                        new_part.s = part[1].parse().unwrap();
                    }
                    _ => {}
                }
            }
            parts.push(new_part);
        }
    }
    let accepted = get_accepted_parts(parts, workflows);
    println!("Part 1: {}", sum_accepted(accepted));
    // println!("Part 2: {}", area_2);
    println!("Done in: {:.2?}!", now.elapsed());
}
