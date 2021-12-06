use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type InputLines = std::io::Lines<std::io::BufReader<std::fs::File>>;

pub fn read_input(day: i32) -> Result<InputLines> {
    let path = format!("resources/day{:02}.txt", day);
    let input = File::open(&path)?;
    let buf_read = BufReader::new(input);
    Ok(buf_read.lines())
}
