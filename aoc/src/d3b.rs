// Advent of Code 2022
// Day 3/Part 2
// https://adventofcode.com/2022/day/3
use itertools::Itertools;
use std::io::{self, BufRead};

fn line_to_vec(line: &String) -> Vec<u32> {
    return line
        .chars()
        .map(|c| match c {
            'a'..='z' => (c as u32) - ('a' as u32) + 1,
            'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            _ => 0,
        })
        .collect();
}

#[allow(dead_code)]
pub fn d3b() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut result: u32 = 0;

    for (a, b, c) in lines.tuples() {
        let (first, second, third): (Vec<u32>, Vec<u32>, Vec<u32>) = vec![a, b, c]
            .iter()
            .map(|line| line.as_ref().unwrap())
            .map(|line| line_to_vec(line))
            .collect_tuple()
            .unwrap();

        result += first
            .iter()
            .filter(|el| second.contains(el) && third.contains(el))
            .last()
            .unwrap();
    }

    println!("{}", result);

    Ok(())
}
