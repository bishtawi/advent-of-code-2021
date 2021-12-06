#![deny(clippy::all, clippy::pedantic)]

use anyhow::Result;
use phf::phf_map;

mod day01;
mod shared;

static DAYS: phf::Map<i32, fn(i32) -> Result<()>> = phf_map! {
    1_i32 => day01::solve,
};

fn main() {
    for (day, solve) in &DAYS {
        solve(*day).unwrap_or_else(|e| println!("Unable to solve day {}: {}", day, e));
    }
}
