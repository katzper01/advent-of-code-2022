// Advent of Code 2022
// Day 1/Part 1
// https://adventofcode.com/2022/day/1
use std::cmp;
use std::io::{self, BufRead};

#[allow(dead_code)]
fn d1a() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut result: i32 = 0;
    let mut sum: i32 = 0;

    for line in lines {
        let content: String = line.unwrap();

        if content.len() > 0 {
            let calories = content.parse::<i32>().unwrap();
            sum += calories;
        } else {
            result = cmp::max(result, sum);
            sum = 0;
        }
    }

    println!("{}", result);

    Ok(())
}
