use crate::shared;
use anyhow::Result;

const DAY: i32 = ?;

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<i32>> {
    let lines = shared::read_input(DAY)?;
}

fn part1(input: &[i32]) -> i32 {
    -1
}

fn part2(input: &[i32]) -> i32 {
    -1
}
