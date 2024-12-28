use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type Graph = HashMap<String, HashMap<String, String>>;

fn solve_part1(graph: &Graph) -> i32 {
    let mut triples: HashMap<Vec<String>, bool> = HashMap::new();
    for (first, v) in graph.iter() {
        for (second, _) in v.iter() {
            for (third, v_third) in graph.iter() {
                if first != third {
                    if v_third.contains_key(first) && v_third.contains_key(second) {
                        let mut names: Vec<String> =
                            vec![first.to_string(), second.to_string(), third.to_string()];
                        names.sort();
                        triples.insert(
                            names,
                            first.starts_with("t")
                                || second.starts_with("t")
                                || third.starts_with("t"),
                        );
                    }
                }
            }
        }
    }
    let mut count = 0;
    for (_, is_t) in triples {
        if is_t {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut now = Instant::now();
    // td-yn ->
    // td: { yn: yn }
    // yn: { td: td }
    let mut graph: Graph = HashMap::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split('-').collect();
        graph
            .entry(splits[0].to_string())
            .or_insert_with(HashMap::new)
            .insert(splits[1].to_string(), splits[1].to_string());
        graph
            .entry(splits[1].to_string())
            .or_insert_with(HashMap::new)
            .insert(splits[0].to_string(), splits[0].to_string());
    }
    // Part 1
    println!("Part 1: {}", solve_part1(&graph));
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
    fn test_foo() {}
}
