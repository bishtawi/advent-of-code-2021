#![deny(clippy::all, clippy::pedantic)]

use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod shared;

const SOLVERS: &[fn() -> Result<()>] = &[day01::solve, day02::solve, day03::solve];

fn main() -> Result<()> {
    for solver in SOLVERS {
        solver()?;
    }

    Ok(())
}
