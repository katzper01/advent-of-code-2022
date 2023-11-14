// Advent of Code 2022
// Day 6/Part 2
// https://adventofcode.com/2022/day/6
use std::collections::HashSet;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn d6b() -> io::Result<()> {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();

    let mut result: usize = 0;

    for i in 14..=line.len() {
        let s: HashSet<char> =
            HashSet::from(line.chars().skip(i - 14).take(14).collect::<HashSet<_>>());
        if s.len() == 14 {
            result = i;
            break;
        }
    }

    println!("{}", result);

    Ok(())
}
