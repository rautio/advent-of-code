use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut lat = 1;
    let mut area: i32 = 1;
    let mut lat_2: i64 = 1;
    let mut area_2: i64 = 1;
    for line in read_to_string("./input.txt").unwrap().lines() {
        let str: Vec<&str> = line.split(" ").collect();
        let distance = str[1].parse::<u32>().unwrap();
        let dis = distance as i32;
        match str[0] {
            "R" => {
                area -= dis * (lat);
            }
            "L" => {
                area += dis * (lat + 1);
            }
            "U" => {
                lat -= dis;
            }
            "D" => {
                area += dis;
                lat += dis;
            }
            _ => {
                panic!("no match")
            }
        };
        let s2: &str = &str[2][2..8];
        let h: &str = &s2[..5];
        let hex_dis = i64::from_str_radix(h, 16).unwrap();
        let dir: &str = &s2[5..6];
        match dir {
            "0" => {
                area_2 -= hex_dis * (lat_2);
            }
            "2" => {
                area_2 += hex_dis * (lat_2 + 1);
            }
            "3" => {
                lat_2 -= hex_dis;
            }
            "1" => {
                area_2 += hex_dis;
                lat_2 += hex_dis;
            }
            _ => {
                panic!("no match")
            }
        }
    }
    println!("Part 1: {}", area);
    println!("Part 2: {}", area_2);
    println!("Done in: {:.2?}!", now.elapsed());
}
