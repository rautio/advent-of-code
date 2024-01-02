use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

use rand::Rng;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-z]{3}): (.*)$").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node {
    id: String,
    vertices: Vec<String>, // Vertices from the original graph
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Edge {
    from: String,
    to: String,
}

fn merge_nodes(a: Node, b: Node, id: String) -> Node {
    let mut new_node = Node {
        id: id,
        vertices: vec![],
    };
    if a.vertices.len() == 0 {
        new_node.vertices.push(a.id);
    }
    if b.vertices.len() == 0 {
        new_node.vertices.push(b.id);
    }
    for v in a.vertices {
        new_node.vertices.push(v);
    }
    for v in b.vertices {
        new_node.vertices.push(v);
    }
    return new_node;
}

fn is_vertex_match(edge_v: String, node: &Node) -> bool {
    if edge_v == node.id {
        return true;
    }
    for v in &node.vertices {
        if edge_v == *v {
            return true;
        }
    }
    return false;
}

fn karger(nodes_input: &Vec<Node>, edges_input: &Vec<Edge>) -> (usize, usize) {
    let mut edges = edges_input.clone();
    let mut nodes: Vec<Node> = nodes_input.clone();
    // We are guaranteed 3 edges in the final cut
    while edges.len() > 3 {
        let mut super_node_counter = 1;
        edges = edges_input.clone(); // In case we need to loop again
        nodes = nodes_input.clone();
        while nodes.len() > 2 && edges.len() > 0 {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..edges.len());
            let mut from_index = 0;
            let mut to_index = 0;
            let mut seen_nodes: HashMap<String, bool> = HashMap::new();
            let edge = edges.swap_remove(index); // Select random edge and combine the two nodes
            for (i, n) in nodes.clone().into_iter().enumerate() {
                if n.id == edge.from {
                    from_index = i;
                } else if n.id == edge.to {
                    to_index = i;
                } else {
                    seen_nodes.insert(n.id, true);
                }
            }
            let node_a = nodes[from_index].clone();
            let node_b = nodes[to_index].clone();
            nodes.remove(from_index);
            if from_index < to_index {
                to_index -= 1;
            }
            nodes.remove(to_index);
            let new_node = merge_nodes(
                node_a.clone(),
                node_b.clone(),
                super_node_counter.to_string(),
            );
            nodes.push(new_node.clone());
            seen_nodes.insert(new_node.clone().id, true);
            let mut new_edges: Vec<Edge> = Vec::new();
            let mut seen_edges: HashMap<String, bool> = HashMap::new();
            for e in &edges {
                let mut new_edge = Edge {
                    from: e.from.clone(),
                    to: e.to.clone(),
                };
                if is_vertex_match(new_edge.from.clone(), &node_a)
                    || is_vertex_match(new_edge.from.clone(), &node_b)
                {
                    new_edge.from = new_node.id.clone();
                } else if is_vertex_match(new_edge.to.clone(), &node_a)
                    || is_vertex_match(new_edge.to.clone(), &node_b)
                {
                    new_edge.to = new_node.id.clone();
                }
                let key_a = get_edge_key(&new_edge.from, &new_edge.to);
                let key_b = get_edge_key(&new_edge.to, &new_edge.from);
                if new_edge.from != new_edge.to
                    // && !seen_edges.contains_key(&key_a)
                    // && !seen_edges.contains_key(&key_b)
                    && seen_nodes.contains_key(&new_edge.from)
                    && seen_nodes.contains_key(&new_edge.to)
                {
                    new_edges.push(new_edge);
                    seen_edges.insert(key_a.clone(), true);
                    seen_edges.insert(key_b.clone(), true);
                }
            }
            edges = new_edges;
            super_node_counter += 1;
        }
    }
    return (nodes[0].vertices.len(), nodes[1].vertices.len());
}

fn get_edge_key(source: &String, dest: &String) -> String {
    let mut key = String::from(source);
    key.push('-');
    key += &dest;
    return key;
}

fn parse_input(input_file: &str) -> (Vec<Node>, Vec<Edge>) {
    let mut seen_nodes: HashMap<String, bool> = HashMap::new();
    let mut seen_edges: HashMap<String, bool> = HashMap::new();
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    for line in read_to_string(input_file).unwrap().lines() {
        let cap = RE.captures(line).unwrap();
        let source = cap.get(1).unwrap().as_str().to_string();
        let dests = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(" ")
            .collect::<Vec<&str>>();
        if !seen_nodes.contains_key(&source) {
            nodes.push(Node {
                id: source.clone(),
                vertices: vec![],
            });
            seen_nodes.insert(source.clone(), true);
        }
        for _d in dests {
            let d = _d.to_string();
            if !seen_nodes.contains_key(&d) {
                nodes.push(Node {
                    id: d.clone(),
                    vertices: vec![],
                });
                seen_nodes.insert(d.clone(), true);
            }
            let key_a = get_edge_key(&source, &d);
            let key_b = get_edge_key(&d, &source);
            if !seen_edges.contains_key(&key_a) && !seen_edges.contains_key(&key_b) {
                edges.push(Edge {
                    from: source.clone(),
                    to: d.clone(),
                });
                seen_edges.insert(key_a.clone(), true);
                seen_edges.insert(key_b.clone(), true);
            }
        }
    }
    return (nodes, edges);
}

fn solve_part1(nodes: Vec<Node>, edges: Vec<Edge>) -> String {
    let mut outcomes: HashMap<String, u32> = HashMap::new();
    // Run 10 iterations since kargers is non deterministic
    // for _i in 0..10 {
    let (a, b) = karger(&nodes, &edges);
    if a != 0 && b != 0 {
        let mut key = get_edge_key(&a.to_string(), &b.to_string());
        if a < b {
            key = get_edge_key(&b.to_string(), &a.to_string());
        }
        *outcomes.entry(key.clone()).or_insert(0) += 1;
    }
    // Return most common outcome - most likely the right answer
    let top_char = outcomes.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    return top_char.0.clone();
}

fn main() {
    let now = Instant::now();
    let (nodes, edges) = parse_input("./input.txt");
    println!("Part 1: {}", solve_part1(nodes, edges));
    println!("Done in: {:?}!", now.elapsed());
}
