use crate::shared;
use anyhow::{bail, Result};

const DAY: i32 = 10;

pub fn solve() -> Result<()> {
    let (illegals, incompletes) = parse()?;
    println!(
        "Day {} = {} :: {}",
        DAY,
        part1(&illegals),
        part2(&incompletes)
    );

    Ok(())
}

fn parse() -> Result<(Vec<char>, Vec<Vec<char>>)> {
    let lines = shared::read_input(DAY)?;

    let mut illegals: Vec<char> = Vec::new();
    let mut incompletes: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.unwrap().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    let b = stack.pop();
                    if b.is_none() || b.unwrap() != '(' {
                        illegals.push(c);
                        stack.clear();
                        break;
                    }
                }
                ']' => {
                    let b = stack.pop();
                    if b.is_none() || b.unwrap() != '[' {
                        illegals.push(c);
                        stack.clear();
                        break;
                    }
                }
                '}' => {
                    let b = stack.pop();
                    if b.is_none() || b.unwrap() != '{' {
                        illegals.push(c);
                        stack.clear();
                        break;
                    }
                }
                '>' => {
                    let b = stack.pop();
                    if b.is_none() || b.unwrap() != '<' {
                        illegals.push(c);
                        stack.clear();
                        break;
                    }
                }
                x => bail!("Unexpected char {}", x),
            }
        }

        if !stack.is_empty() {
            incompletes.push(stack);
        }
    }

    Ok((illegals, incompletes))
}

fn part1(illegals: &[char]) -> i32 {
    illegals
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("wont happen"),
        })
        .sum()
}

fn part2(incompletes: &[Vec<char>]) -> i64 {
    let mut scores: Vec<i64> = incompletes
        .iter()
        .map(|incomplete| {
            incomplete.iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("wont happen"),
                    }
            })
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
