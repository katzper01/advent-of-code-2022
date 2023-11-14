// Advent of Code 2022
// Day 8/Part 2
// https://adventofcode.com/2022/day/8
use std::cmp;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn d8b() -> io::Result<()> {
    let stdin = io::stdin();
    let a: Vec<Vec<i32>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .map(|l: Vec<char>| l.iter().map(|&c| c as i32 - 48).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let n = a.len();
    let m = a[0].len();

    let in_bounds = |x: i32, y: i32| -> bool {
        return x >= 0 && x < m as i32 && y >= 0 && y < n as i32;
    };

    let mut result: u32 = 0;
    let dx = vec![0, 1, 0, -1];
    let dy = vec![-1, 0, 1, 0];

    for i in 0..n {
        for j in 0..m {
            let mut vis = 1;
            for k in 0..4 {
                let mut cnt = 0;
                let mut x = j as i32 + dx[k];
                let mut y = i as i32 + dy[k];
                while in_bounds(x, y) {
                    let xi = x as usize;
                    let yi = y as usize;
                    cnt += 1;
                    if a[yi][xi] >= a[i][j] {
                        break;
                    };
                    x += dx[k];
                    y += dy[k];
                }
                vis *= cnt;
            }
            result = cmp::max(result, vis);
        }
    }

    println!("{}", result);

    Ok(())
}
