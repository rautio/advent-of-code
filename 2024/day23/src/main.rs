use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type Graph = HashMap<String, HashMap<String, String>>;

fn find_triples(graph: &Graph) -> HashMap<Vec<String>, bool> {
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
    triples
}

fn solve_part1(graph: &Graph) -> i32 {
    let mut count = 0;
    let triples = find_triples(&graph);
    for (_, is_t) in triples {
        if is_t {
            count += 1;
        }
    }
    count
}

fn is_connected(start: String, end: String, graph: &Graph) -> bool {
    graph.get(&start).unwrap().contains_key(&end) && graph.get(&end).unwrap().contains_key(&start)
}

fn max_clique(graph: &Graph) -> Vec<String> {
    let triples: HashMap<Vec<String>, bool> = find_triples(&graph);
    let mut cliques: Vec<Vec<String>> = Vec::new();
    for (t, _) in triples.iter() {
        cliques.push(t.to_vec());
    }
    let mut max: Vec<String> = Vec::new();
    while cliques.len() > 0 {
        let mut new_cliques: Vec<Vec<String>> = Vec::new();
        let mut counted: HashMap<String, String> = HashMap::new();
        for cur in cliques {
            for (k, _) in graph.iter() {
                let mut connected = true;
                for c in &cur {
                    // Has to be connected to every other element in cur
                    if *c != *k {
                        if !is_connected(c.clone(), k.clone(), graph) {
                            connected = false;
                            break;
                        }
                    }
                }
                if connected && !cur.contains(k) && !counted.contains_key(k) {
                    counted.insert(k.to_string(), k.to_string());
                    let mut new_cur = cur.clone();
                    new_cur.push(k.clone());
                    new_cliques.push(new_cur.clone());
                    if new_cur.len() > max.len() {
                        max = new_cur;
                    }
                    break;
                }
            }
        }
        cliques = new_cliques;
    }

    max
}

fn solve_part2(graph: &Graph) -> String {
    let mut max = max_clique(graph);

    max.sort();
    max.iter().map(|x| x.to_string() + ",").collect::<String>()
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
    println!("Part 2: {}", solve_part2(&graph));
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected() {
        let c1: HashMap<String, String> = HashMap::from([
            (String::from("ab"), String::from("ab")),
            (String::from("ca"), String::from("ca")),
        ]);
        let c2: HashMap<String, String> = HashMap::from([
            (String::from("tk"), String::from("tk")),
            (String::from("ca"), String::from("ca")),
        ]);
        let c3: HashMap<String, String> = HashMap::from([(String::from("zk"), String::from("zk"))]);
        let graph: Graph = HashMap::from([
            (String::from("tk"), c1),
            (String::from("ab"), c2),
            (String::from("ca"), c3),
        ]);
        assert_eq!(
            is_connected(String::from("tk"), String::from("ab"), &graph),
            true
        );
        assert_eq!(
            is_connected(String::from("tk"), String::from("ca"), &graph),
            false
        );
    }
}
