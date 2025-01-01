use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}
impl Pt {
    pub fn new(x: i32, y: i32) -> Self {
        Pt { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cursor {
    pos: Pt,
    seq: Vec<char>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Dir {
    pos: Pt,
    c: char,
}

const N: Dir = Dir {
    pos: Pt { x: 0, y: -1 },
    c: '^',
};
const E: Dir = Dir {
    pos: Pt { x: 1, y: 0 },
    c: '>',
};
const S: Dir = Dir {
    pos: Pt { x: 0, y: 1 },
    c: 'v',
};
const W: Dir = Dir {
    pos: Pt { x: -1, y: 0 },
    c: '<',
};

fn add(p1: Pt, p2: Pt) -> Pt {
    Pt {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn get_num_pad() -> HashMap<Pt, char> {
    HashMap::from([
        (Pt { x: 0, y: 0 }, '7'),
        (Pt { x: 1, y: 0 }, '8'),
        (Pt { x: 2, y: 0 }, '9'),
        (Pt { x: 0, y: 1 }, '4'),
        (Pt { x: 1, y: 1 }, '5'),
        (Pt { x: 2, y: 1 }, '6'),
        (Pt { x: 0, y: 2 }, '1'),
        (Pt { x: 1, y: 2 }, '2'),
        (Pt { x: 2, y: 2 }, '3'),
        (Pt { x: 1, y: 3 }, '0'),
        (Pt { x: 2, y: 3 }, 'A'),
    ])
}

fn get_dir_pad() -> HashMap<Pt, char> {
    HashMap::from([
        (Pt { x: 1, y: 0 }, '^'),
        (Pt { x: 2, y: 0 }, 'A'),
        (Pt { x: 0, y: 1 }, '<'),
        (Pt { x: 1, y: 1 }, 'v'),
        (Pt { x: 2, y: 1 }, '>'),
    ])
}

fn dist(p1: Pt, p2: Pt) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

/**
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

/**
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

fn score_seq(seq: Vec<char>) -> usize {
    let mut steps = 0;

    let mut cur = 'A';
    let mut second_seq: Vec<char> = Vec::new();
    for (_, target) in seq.into_iter().enumerate() {
        let mut new_seq = generate_seq(cur, target, get_dir_pad())[0].clone();
        // two-layer scoring

        cur = target;
        second_seq.append(&mut new_seq);
    }
    cur = 'A';
    for (_, target) in second_seq.into_iter().enumerate() {
        let new_seq = generate_seq(cur, target, get_dir_pad())[0].clone();
        // two-layer scoring
        cur = target;
        steps += new_seq.len();
    }

    steps
}

fn generate_seq(cur: char, target: char, grid: HashMap<Pt, char>) -> Vec<Vec<char>> {
    let mut c = Pt::new(0, 0);
    let mut t = Pt::new(0, 0);
    let mut seq: Vec<Vec<char>> = Vec::new();
    for (key, val) in &grid {
        if *val == cur {
            c = *key;
        }
        if *val == target {
            t = *key;
        }
    }
    let mut cursors = VecDeque::from([Cursor {
        pos: c,
        seq: vec![],
    }]);

    while let Some(cur) = cursors.pop_front() {
        if cur.pos == t {
            let mut cur_seq = cur.seq.clone();
            cur_seq.push('A');
            seq.push(cur_seq);
        }
        let d = dist(cur.pos, t);
        let next = vec![N, E, S, W]
            .into_iter()
            .map(|dir| Dir {
                pos: add(dir.pos, cur.pos),
                c: dir.c,
            })
            .filter(|dir| grid.contains_key(&dir.pos) && dist(dir.pos, t) < d);
        for n in next {
            let mut seq = cur.seq.clone();
            seq.push(n.c);
            cursors.push_back(Cursor { pos: n.pos, seq });
        }
    }

    seq
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct KeyPad {
    cursor: Pt,
    grid: HashMap<Pt, char>,
}

fn move_pt(start: Pt, dir: char) -> Pt {
    match dir {
        '^' => add(start, N.pos),
        '<' => add(start, W.pos),
        'v' => add(start, S.pos),
        '>' => add(start, E.pos),
        _ => panic!("unknown command"),
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct PressCursor {
    keypad: usize,
    c: char,
}

fn press_seq(seq: String, num_robots: usize) -> String {
    let mut output: Vec<char> = Vec::new();
    let mut keypads: Vec<KeyPad> = Vec::new();
    for _ in 0..num_robots {
        keypads.push(KeyPad {
            cursor: Pt { x: 2, y: 0 },
            grid: get_dir_pad(),
        });
    }
    keypads.push(KeyPad {
        cursor: Pt { x: 2, y: 3 },
        grid: get_num_pad(),
    });
    for c in seq.chars() {
        let mut moves = VecDeque::from([PressCursor { keypad: 0, c }]);
        while let Some(cur) = moves.pop_front() {
            let k = cur.keypad;
            if cur.c == 'A' {
                // Add next keypad
                if !keypads[k].grid.contains_key(&keypads[k].cursor) {
                    panic!("Out of bounds!");
                }
                let new_c = *keypads[k].grid.get(&keypads[k].cursor).unwrap();
                if k < keypads.len() - 1 {
                    moves.push_back(PressCursor {
                        keypad: k + 1,
                        c: new_c,
                    });
                } else {
                    output.push(new_c);
                }
            } else {
                keypads[k].cursor = move_pt(keypads[k].cursor, cur.c);
            }
        }
    }
    String::from_iter(output)
}

fn seq_cached(
    code: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, code.clone())) {
        return cached;
    }

    let grid = if depth == 0 {
        get_num_pad()
    } else {
        get_dir_pad()
    };

    let mut cur = 'A';
    let mut res = 0;

    for (_, target) in code.chars().enumerate() {
        let mut seqs: Vec<Vec<char>> = generate_seq(cur, target, grid.clone());
        seqs.sort_by(|a, b| score_seq(a.clone()).cmp(&score_seq(b.clone())));
        if depth == max_depth {
            res += seqs[0].len();
        } else {
            res += seq_cached(
                String::from_iter(seqs[0].clone()),
                depth + 1,
                max_depth,
                cache,
            );
        }
        cur = target;
    }

    cache.insert((depth, code), res);
    res
}

fn seq(code: &str, dir_pads: usize) -> String {
    let mut dir_memo: HashMap<(char, char), Vec<char>> = HashMap::new();
    // Minimize 0-2, 2-7, 7-9, 9-A
    let mut final_seq: Vec<char> = Vec::new();
    let mut cur_numpad = 'A';
    for (_, target) in code.chars().enumerate() {
        // NumPad
        let mut seqs: Vec<Vec<char>> = generate_seq(cur_numpad, target, get_num_pad());
        seqs.sort_by(|a, b| score_seq(a.clone()).cmp(&score_seq(b.clone())));
        let mut seq = seqs[0].clone();
        for _ in 0..dir_pads {
            let mut new_seq: Vec<char> = Vec::new();
            let mut dir_cur = 'A';
            for (_, dir_target) in seq.clone().into_iter().enumerate() {
                let mut dir_seq: Vec<char> = Vec::new();
                if dir_memo.contains_key(&(dir_cur, dir_target)) {
                    dir_seq = dir_memo.get(&(dir_cur, dir_target)).unwrap().clone();
                } else {
                    let mut dir_seqs = generate_seq(dir_cur, dir_target, get_dir_pad());
                    dir_seqs.sort_by(|a, b| score_seq(a.clone()).cmp(&score_seq(b.clone())));
                    dir_seq = dir_seqs[0].clone();
                    dir_memo.insert((dir_cur, dir_target), dir_seq.clone());
                }
                new_seq.append(&mut dir_seq);
                // Iterate cur seq
                dir_cur = dir_target;
            }
            seq = new_seq;
        }
        // Push final seq.
        final_seq.append(&mut seq);
        cur_numpad = target;
    }
    String::from_iter(final_seq)
}

fn solve_part1(codes: &Vec<&str>) -> i32 {
    let mut complexity = 0;
    let mut cached_complexity = 0;
    let mut cache = HashMap::new();

    for code in codes {
        let s_cache = seq_cached(code.to_string(), 0, 2, &mut cache);
        let s = seq(code, 2).len();
        let num = &code[..code.len() - 1].parse::<i32>().unwrap();
        complexity += s as i32 * num;
        cached_complexity += s_cache as i32 * num;
    }

    assert_eq!(cached_complexity, complexity);

    cached_complexity
}

fn solve_part2(codes: &Vec<&str>) -> i64 {
    let mut complexity = 0;

    let mut cache = HashMap::new();
    for code in codes {
        let s = seq_cached(code.to_string(), 0, 25, &mut cache);
        let num = &code[..code.len() - 1].parse::<i64>().unwrap();
        complexity += s as i64 * num;
    }

    complexity
}

fn main() {
    let mut now = Instant::now();
    let mut codes: Vec<&str> = Vec::new();
    let binding = read_to_string("./src/input.txt").unwrap();
    for line in binding.lines() {
        codes.push(line);
    }
    // Part 1
    println!("Part 1: {}", solve_part1(&codes));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", solve_part2(&codes));
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
    */

    /**
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */
    #[test]
    fn test_generate_seq() {
        assert_eq!(
            generate_seq('A', '2', get_num_pad()),
            vec!["^<A", "<^A"]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            generate_seq('^', '>', get_dir_pad()),
            vec![">vA", "v>A"]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            String::from_iter(generate_seq('0', '7', get_num_pad())[0].clone()),
            "^^^<A"
        );
        assert_eq!(
            generate_seq('4', 'A', get_num_pad()),
            vec![">>vvA", ">v>vA", ">vv>A", "v>>vA", "v>v>A"]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            String::from_iter(generate_seq('A', '0', get_num_pad())[0].clone()),
            "<A"
        );
        assert_eq!(
            String::from_iter(generate_seq('0', '2', get_num_pad())[0].clone()),
            "^A"
        );
        assert_eq!(
            String::from_iter(generate_seq('A', '2', get_num_pad())[0].clone()),
            "^<A"
        );
        assert_eq!(
            String::from_iter(generate_seq('A', '<', get_dir_pad())[0].clone()),
            "v<<A"
        );
        assert_eq!(
            String::from_iter(generate_seq('A', 'v', get_dir_pad())[0].clone()),
            "v<A"
        );
        assert_eq!(
            generate_seq('3', '7', get_num_pad()),
            vec!["^^<<A", "^<^<A", "^<<^A", "<^^<A", "<^<^A", "<<^^A"]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
    }
    #[test]
    fn test_score_seq() {
        assert_eq!(score_seq(">>^A".chars().collect::<Vec<char>>()), 20);
        assert_eq!(score_seq(">^>A".chars().collect::<Vec<char>>()), 26);
        let mut v = vec!["^^<<A", "^<^<A", "^<<^A", "<^^<A", "<^<^A", "<<^^A"]
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        v.sort_by(|a, b| score_seq(a.clone()).cmp(&score_seq(b.clone())));
        assert_eq!(
            v,
            vec!["<<^^A", "^^<<A", "^<<^A", "<^^<A", "<^<^A", "^<^<A"]
                .into_iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            v.into_iter().map(|s| score_seq(s)).collect::<Vec<usize>>(),
            vec![23, 29, 29, 37, 37, 43]
        );
        assert_eq!(score_seq("<v<AA>^AA>A".chars().collect::<Vec<char>>()), 63);
        assert_eq!(score_seq("<AAv<AA>>^A".chars().collect::<Vec<char>>()), 69);
    }

    #[test]
    fn test_seq() {
        assert_eq!(seq("029A", 0), "<A^A^^>AvvvA");
        assert_eq!(seq("029A", 1).len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
        assert_eq!(seq("96783", 1), "<AAA>A<vA^>Av<<AA>^A>AvA^A<vAA>A^A");
        assert_eq!(
            seq("029A", 2).len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            seq("980A", 2).len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
        assert_eq!(
            seq("179A", 2).len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            seq("456A", 2).len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );

        assert_eq!(
            seq("379A", 2).len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_press_seq() {
        assert_eq!(press_seq("<A^A>^^AvvvA".to_string(), 0), "029A");
        assert_eq!(
            press_seq("<AAA>Av<A^>A<Av<AA>^>AvA^AvA<AA^>A".to_string(), 1),
            "96783"
        );
        assert_eq!(
            press_seq("v<<A>^>A<A>A<AA>vA^Av<AAA^>A".to_string(), 1),
            "029A"
        );

        assert_eq!(
            press_seq("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string(), 1),
            "029A"
        );
        assert_eq!(
            press_seq(
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string(),
                2
            ),
            "029A"
        );
        assert_eq!(
            press_seq(
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(),
                2
            ),
            "379A"
        );
        assert_eq!(
            press_seq(
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".to_string(),
                2
            ),
            "980A"
        );
        assert_eq!(
            press_seq(
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(),
                2
            ),
            "179A"
        );
        assert_eq!(
            press_seq(
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".to_string(),
                2
            ),
            "456A"
        );
    }

    #[test]
    fn test_integration() {
        assert_eq!(press_seq(seq("123", 0), 0), "123");
        assert_eq!(press_seq(seq("123", 1), 1), "123");
        assert_eq!(press_seq(seq("96783", 2), 2), "96783");
        assert_eq!(press_seq(seq("1027", 3), 3), "1027");
        assert_eq!(press_seq(seq("52A", 4), 4), "52A");
    }

    #[test]
    fn test_seq_cached() {
        let mut cache = HashMap::new();
        assert_eq!(
            seq_cached(String::from("0A"), 0, 0, &mut cache),
            seq("0A", 0).len()
        );
    }
}
