// Advent of Code 2022
// Day 8/Part 1
// https://adventofcode.com/2022/day/8
use std::collections::HashSet;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn d8a() -> io::Result<()> {
    let stdin = io::stdin();
    let a: Vec<Vec<i32>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .map(|l: Vec<char>| l.iter().map(|&c| c as i32 - 48).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let n: usize = a.len();
    let m: usize = a[0].len();

    let mut visible: HashSet<usize> = HashSet::new();
    let mut max: i32;

    let mut update = |i: usize, j: usize, max: &mut i32| {
        if a[i][j] > *max {
            visible.insert(i * m + j);
            *max = a[i][j];
        }
    };

    for i in 0..n {
        max = -1;
        for j in 0..m {
            update(i, j, &mut max);
        }

        max = -1;
        for j in (0..m).rev() {
            update(i, j, &mut max);
        }
    }

    for j in 0..m {
        max = -1;
        for i in 0..n {
            update(i, j, &mut max);
        }
    }

    for j in 0..m {
        max = -1;
        for i in (0..n).rev() {
            update(i, j, &mut max);
        }
    }

    println!("{}", visible.len());

    Ok(())
}
