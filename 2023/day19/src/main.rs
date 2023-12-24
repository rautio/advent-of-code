use core::ops::Range;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn match_cond(cur: u64, comp: String, amount: u64) -> bool {
    if comp == "<" {
        return cur < amount;
    }
    return cur > amount;
}

fn sum_accepted(parts: Vec<Part>) -> u64 {
    let mut sum = 0;
    for p in parts {
        sum += p.x;
        sum += p.m;
        sum += p.s;
        sum += p.a;
    }
    return sum;
}

fn get_accepted_parts(parts: Vec<Part>, workflows: &HashMap<String, String>) -> Vec<Part> {
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

fn filter_range(r: Range<usize>, amount: u64, comp: String, is_reverse: bool) -> Range<usize> {
    let mut new_r = r.clone();
    let mut adj = 1;
    if is_reverse {
        adj = 0;
    }
    if comp == "<" {
        new_r = r.start..amount as usize - adj;
    } else {
        new_r = amount as usize + adj..r.end;
    }
    if new_r.start > new_r.end {
        return 0..0;
    }
    return new_r;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    workflow: String,
    combinations: Vec<Range<usize>>, // x,m,a,s
}

fn range_len(r: &Range<usize>) -> usize {
    return r.end - r.start + 1;
}

fn mul_ranges(v: &Vec<Range<usize>>) -> i64 {
    let mut total: i64 = 1;
    for r in v {
        total = total * range_len(&r) as i64;
    }
    return total;
}

fn range_intersect(a: &Range<usize>, b: &Range<usize>) -> usize {
    if a.start > b.end || b.start > a.end {
        return 0;
    }
    let mut lesser_end = a.end;
    if b.end < a.end {
        lesser_end = b.end;
    }
    let mut bigger_start = a.start;
    if b.start > a.start {
        bigger_start = b.start;
    }
    return lesser_end - bigger_start + 1;
}

fn get_combinations(workflows: &HashMap<String, String>) -> i64 {
    let test_re: Regex = Regex::new(r"(x|m|a|s)(<|>)(.+)*").unwrap();
    let f = 1..4000;
    let mut states: VecDeque<State> = VecDeque::new();
    states.push_front(State {
        workflow: String::from("in"),
        combinations: vec![f.clone(), f.clone(), f.clone(), f.clone()],
    });
    let mut final_states: Vec<State> = Vec::new();
    while states.len() > 0 {
        let mut state = states.pop_front().unwrap();
        let cur = state.workflow.clone();
        let x = state.combinations[0].clone();
        let m = state.combinations[1].clone();
        let a = state.combinations[2].clone();
        let s = state.combinations[3].clone();
        if cur == "A" {
            final_states.push(State {
                workflow: cur,
                combinations: vec![x.clone(), m.clone(), a.clone(), s.clone()],
            });
        } else if cur != "R" {
            let flow = workflows.get(&cur).unwrap();
            let conditions = flow.split(",");
            for c in conditions {
                let x = state.combinations[0].clone();
                let m = state.combinations[1].clone();
                let a = state.combinations[2].clone();
                let s = state.combinations[3].clone();
                if c.contains(":") {
                    let splits: Vec<&str> = c.split(":").collect();
                    let test = splits[0];
                    let next_workflow = splits[1].to_string();
                    let test_cap = test_re.captures(test).unwrap();
                    let sub_part = &test_cap[1];
                    let comp = test_cap[2].to_string();
                    let reverse_comp = if comp == "<" { ">" } else { "<" };
                    let amount = test_cap[3].parse().unwrap();
                    match sub_part {
                        "x" => {
                            // It matched - continue
                            let x_match = filter_range(x.clone(), amount, comp, false);
                            // No match - use the opposite
                            let x_split =
                                filter_range(x.clone(), amount, reverse_comp.to_string(), true);
                            let new_state = State {
                                workflow: next_workflow,
                                combinations: vec![x_match, m.clone(), a.clone(), s.clone()],
                            };
                            state.combinations = vec![x_split, m.clone(), a.clone(), s.clone()];
                            states.push_back(new_state);
                        }
                        "m" => {
                            // It matched - continue
                            let m_match = filter_range(m.clone(), amount, comp, false);
                            // No match - use the opposite
                            let m_split =
                                filter_range(m.clone(), amount, reverse_comp.to_string(), true);
                            let new_state = State {
                                workflow: next_workflow,
                                combinations: vec![x.clone(), m_match, a.clone(), s.clone()],
                            };
                            state.combinations = vec![x.clone(), m_split, a.clone(), s.clone()];
                            states.push_back(new_state);
                        }
                        "a" => {
                            // It matched - continue
                            let a_match = filter_range(a.clone(), amount, comp, false);
                            // No match - use the opposite
                            let a_split =
                                filter_range(a.clone(), amount, reverse_comp.to_string(), true);
                            let new_state = State {
                                workflow: next_workflow,
                                combinations: vec![x.clone(), m.clone(), a_match, s.clone()],
                            };
                            state.combinations = vec![x.clone(), m.clone(), a_split, s.clone()];
                            states.push_back(new_state);
                        }
                        "s" => {
                            // It matched - continue
                            let s_match = filter_range(s.clone(), amount, comp, false);
                            // No match - use the opposite
                            let s_split =
                                filter_range(s.clone(), amount, reverse_comp.to_string(), true);
                            let new_state = State {
                                workflow: next_workflow,
                                combinations: vec![x.clone(), m.clone(), a.clone(), s_match],
                            };
                            state.combinations = vec![x.clone(), m.clone(), a.clone(), s_split];
                            states.push_back(new_state);
                        }
                        _ => {}
                    }
                } else {
                    // End of the line!
                    state.workflow = c.to_string();
                }
            }
            states.push_back(state);
        }
    }
    let mut total: i64 = 0;
    let mut seen: Vec<&State> = Vec::new();
    for fs in &final_states {
        let total_combinations = mul_ranges(&fs.combinations);
        total += total_combinations as i64;
        // Determine if this combo overlaps with any previous ones
        if seen.len() > 0 {
            for v in &seen {
                let mut intersects = 1;
                for i in 0..4 {
                    intersects *= range_intersect(&fs.combinations[i], &v.combinations[i]);
                }
                if intersects != 1 {
                    total -= intersects as i64;
                }
            }
        }
        seen.push(fs);
    }
    return total;
}

fn main() {
    let mut now = Instant::now();
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
    let accepted = get_accepted_parts(parts, &workflows);
    println!("Part 1: {}", sum_accepted(accepted));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {:?}", get_combinations(&workflows));
    println!("Done in: {:.2?}!", now.elapsed());
}
