// Advent of Code 2022
// Day 7/Part 1
// https://adventofcode.com/2022/day/7
use std::collections::HashMap;
use std::io::{self, BufRead};

fn dfs(l: &Vec<String>, i: &mut usize, dir: String, sizes: &mut HashMap<String, u32>) -> u32 {
    let mut size: u32 = 0;

    loop {
        if *i >= l.len() {
            break;
        }
        let mut cmd: Vec<&str> = l[*i].split(" ").collect();
        cmd.push("#");
        *i += 1;

        match (cmd[0], cmd[1], cmd[2]) {
            ("$", "cd", "..") => break,
            ("$", "cd", "/") => {}
            ("$", "cd", _) => size += dfs(l, i, dir.clone() + cmd[2] + "/", sizes),
            ("$", "ls", _) | ("dir", _, _) => {}
            _ => size += cmd[0].parse::<u32>().unwrap(),
        };
    }

    sizes.insert(dir, size);
    return size;
}

#[allow(dead_code)]
pub fn d7a() -> io::Result<()> {
    let stdin = io::stdin();
    let l: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut i: usize = 0;

    dfs(&l, &mut i, "/".to_string(), &mut sizes);

    let result: u32 = sizes
        .iter()
        .filter(|&(_, v)| *v <= 100000)
        .map(|(_, v)| v)
        .sum();

    println!("{}", result);

    Ok(())
}
