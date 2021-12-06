use crate::shared;
use anyhow::Result;

const DAY: i32 = ?;

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    println!("Day {} = {} :: {}", DAY, part1()?, part2()?);

    Ok(())
}

fn part1() -> Result<i32> {
    let _lines = shared::read_input(DAY)?;

    Ok(0)
}

fn part2() -> Result<i32> {
    let _lines = shared::read_input(DAY)?;

    Ok(0)
}
