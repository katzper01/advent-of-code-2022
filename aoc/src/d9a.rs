// Advent of Code 2022
// Day 9/Part 1
// https://adventofcode.com/2022/day/9
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn d9a() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let dx: HashMap<&str, i32> = HashMap::from([("U", 0), ("R", 1), ("D", 0), ("L", -1)]);
    let dy: HashMap<&str, i32> = HashMap::from([("U", 1), ("R", 0), ("D", -1), ("L", 0)]);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for line in lines {
        let s = line.unwrap();
        let (dir, cnt_str) = s.split(" ").collect_tuple().unwrap();
        let cnt = cnt_str.parse::<u32>().unwrap();

        for _ in 0..cnt {
            a += dx[dir];
            b += dy[dir];

            if cmp::max((a - c).abs(), (b - d).abs()) >= 2 {
                c = a - dx[dir];
                d = b - dy[dir];
            }

            visited.insert((c, d));
        }
    }

    println!("{}", visited.len());

    Ok(())
}
