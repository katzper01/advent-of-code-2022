// Advent of Code 2022
// Day 3/Part 1
// https://adventofcode.com/2022/day/3
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

#[allow(dead_code)]
pub fn d3a() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut result: u32 = 0;

    for line in lines {
        let mut first: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|c| match c {
                'a'..='z' => (c as u32) - ('a' as u32) + 1,
                'A'..='Z' => (c as u32) - ('A' as u32) + 27,
                _ => 0,
            })
            .collect();

        let second = first.split_off(first.len() / 2);

        let first_set: HashSet<u32> = HashSet::from_iter(first);
        let second_set: HashSet<u32> = HashSet::from_iter(second);

        result += first_set.intersection(&second_set).last().unwrap();
    }

    println!("{}", result);

    Ok(())
}
