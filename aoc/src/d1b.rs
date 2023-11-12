// Advent of Code 2022
// Day 1/Part 2
// https://adventofcode.com/2022/day/1
use std::io::{self, BufRead};

#[allow(dead_code)]
fn d1b() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut sum: i32 = 0;
    let mut v: Vec<i32> = Vec::new();

    for line in lines {
        let content: String = line.unwrap();

        if content.len() > 0 {
            let calories = content.parse::<i32>().unwrap();
            sum += calories;
        } else {
            v.push(sum);
            sum = 0;
        }
    }

    v.sort();

    let mut result: i32 = 0;

    for i in 1..4 {
        result += v.get(v.len() - i).unwrap();
    }

    println!("{}", result);

    Ok(())
}
