#![deny(clippy::all, clippy::pedantic)]

use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day09;
mod shared;

const SOLVERS: &[fn() -> Result<()>] = &[
    day09::solve,
    day07::solve,
    day06::solve,
    day05::solve,
    day04::solve,
    day03::solve,
    day02::solve,
    day01::solve,
];

fn main() -> Result<()> {
    for solver in SOLVERS {
        solver()?;
    }

    Ok(())
}
