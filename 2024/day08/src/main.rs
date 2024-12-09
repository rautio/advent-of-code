use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

fn find_antinode(p1: &Pt, p2: &Pt) -> Pt {
    let diff = Pt {
        x: p2.x - p1.x,
        y: p2.y - p1.y,
    };

    Pt {
        x: p2.x + diff.x,
        y: p2.y + diff.y,
    }
}

fn find_antinodes(
    grid: &HashMap<Pt, char>,
    antennas: &HashMap<char, Vec<Pt>>,
) -> HashMap<Pt, char> {
    let mut antinodes: HashMap<Pt, char> = HashMap::new();

    for freq in antennas.keys() {
        let pts = antennas.get(&freq).unwrap();
        for (i, p1) in pts.into_iter().enumerate() {
            for p2 in &pts[i + 1..] {
                let antinode1 = find_antinode(p1, p2);
                let antinode2 = find_antinode(p2, p1);
                if grid.contains_key(&antinode1) {
                    antinodes.insert(antinode1, *freq);
                }
                if grid.contains_key(&antinode2) {
                    antinodes.insert(antinode2, *freq);
                }
            }
        }
    }

    antinodes
}

fn count_antinodes(grid: &HashMap<Pt, char>, antennas: &HashMap<char, Vec<Pt>>) -> usize {
    let antinodes = find_antinodes(grid, antennas);

    antinodes.keys().len()
}

fn main() {
    let mut now = Instant::now();
    let mut grid: HashMap<Pt, char> = HashMap::new();
    let mut antennas: HashMap<char, Vec<Pt>> = HashMap::new();
    let mut y = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut x = 0;
        let l: Vec<char> = line.chars().collect();
        for c in l {
            grid.insert(Pt { x, y }, c);
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push(Pt { x, y })
            }
            x += 1;
        }
        y += 1;
    }
    // Part 1
    println!("Part 1: {}", count_antinodes(&grid, &antennas));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
