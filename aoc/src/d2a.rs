// Advent of Code 2022
// Day 2/Part 1
// https://adventofcode.com/2022/day/2
use std::io::{self, BufRead};

#[allow(dead_code)]
fn d2a() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut result: u32 = 0;

    for line in lines {
        let symbols: Vec<char> = line
            .unwrap()
            .split(" ")
            .map(str::to_string)
            .map(|s| s.chars().next().expect("string is empty"))
            .collect();

        let they: u32 = (*symbols.get(0).unwrap() as u32) - ('A' as u32);
        let me: u32 = (*symbols.get(1).unwrap() as u32) - ('X' as u32);

        result += me + 1;

        if me == they {
            result += 3;
        } else if (me + 2) % 3 == they {
            result += 6;
        }
    }

    println!("{}", result);

    Ok(())
}
