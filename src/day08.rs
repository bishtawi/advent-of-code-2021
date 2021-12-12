use crate::shared;
use anyhow::Result;
use std::collections::HashMap;

const DAY: i32 = 8;

struct Entry {
    _uniques: Vec<String>,
    msg: Vec<String>,
}

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    let input = parse()?;

    let _digit_count: HashMap<i32, i32> = HashMap::from([
        (0, 6),
        (1, 2),
        (2, 5),
        (3, 5),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 3),
        (8, 7),
        (9, 6),
    ]);

    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<Entry>> {
    let lines = shared::read_input(DAY)?;
    Ok(lines
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<&str> = line.split(" | ").collect();
            assert_eq!(split.len(), 2);
            Entry {
                _uniques: split[0].split(' ').map(sort_string).collect(),
                msg: split[1].split(' ').map(sort_string).collect(),
            }
        })
        .collect())
}

fn sort_string(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|e| e.msg.iter().filter(|s| is_unique(s)).count())
        .sum()
}

fn is_unique(string: &str) -> bool {
    let len = string.len();
    len == 2 || len == 4 || len == 3 || len == 7
}

fn part2(_input: &[Entry]) -> i32 {
    -1
}
