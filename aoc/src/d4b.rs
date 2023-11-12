// Advent of Code 2022
// Day 4/Part 2
// https://adventofcode.com/2022/day/4
use scanf::sscanf;
use std::io::{self, BufRead};

fn ranges_overlap(a: &u32, b: &u32, c: &u32, d: &u32) -> bool {
    return (a <= c && b >= c) || (c <= a && d >= a);
}

#[allow(dead_code)]
pub fn d4b() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut result: u32 = 0;

    for line in lines {
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        let mut c: u32 = 0;
        let mut d: u32 = 0;

        if sscanf!(&line.unwrap(), "{}-{},{}-{}", a, b, c, d).is_ok() {
            if ranges_overlap(&a, &b, &c, &d) {
                result += 1;
            }
        }
    }

    println!("{}", result);

    Ok(())
}
