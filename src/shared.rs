use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

pub type InputLines = Lines<BufReader<File>>;

pub fn read_input(day: i32) -> Result<InputLines> {
    let buf_read = buf_reader(day)?;
    Ok(buf_read.lines())
}

pub fn read_input_as_string(day: i32) -> Result<String> {
    let mut buf_read = buf_reader(day)?;
    let mut buffer = String::new();
    buf_read.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn buf_reader(day: i32) -> Result<BufReader<File>> {
    let path = format!("resources/day{:02}.txt", day);
    let input = File::open(&path)?;
    Ok(BufReader::new(input))
}
