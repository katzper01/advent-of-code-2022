// Advent of Code 2022
// Day 5/Part 1
// https://adventofcode.com/2022/day/5
use scanf::sscanf;
use std::io::{self, BufRead};

fn parse_initial_state(initial_state: &mut Vec<String>) -> Vec<Vec<char>> {
    let last_line: String = initial_state.pop().unwrap();

    let number_of_stacks: usize = last_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .len();

    let mut stacks: Vec<Vec<char>> = vec![];

    while stacks.len() < number_of_stacks {
        stacks.push(vec![]);
    }

    for line in initial_state.iter().rev() {
        for i in 0..number_of_stacks {
            let c: char = line.chars().nth(1 + i * 4).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

#[allow(dead_code)]
pub fn d5a() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut initial_state: Vec<String> = vec![];
    let mut instructions: Vec<String> = vec![];

    for line in lines {
        let line_string: String = line.unwrap();

        if line_string.len() > 0 {
            if line_string.chars().next().unwrap() == 'm' {
                instructions.push(line_string);
            } else {
                initial_state.push(line_string);
            }
        }
    }

    let mut stacks: Vec<Vec<char>> = parse_initial_state(&mut initial_state);

    for line in instructions {
        let mut n: u32 = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        if sscanf!(&line, "move {} from {} to {}", n, x, y).is_ok() {
            for _ in 0..n {
                let el: char = stacks[x - 1].pop().unwrap();
                stacks[y - 1].push(el);
            }
        }
    }

    let result: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect();

    println!("{}", result);

    Ok(())
}
