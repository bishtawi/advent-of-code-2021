use std::collections::HashSet;

use crate::shared;
use anyhow::Result;

const DAY: i32 = 9;

struct Map {
    data: Vec<Vec<u32>>,
}

impl Map {
    fn find_low_points(self: &Map) -> Vec<u32> {
        let mut low_points: Vec<u32> = Vec::new();
        for (i, r) in self.data.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                if self.is_low_point(i, j, v) {
                    low_points.push(v);
                }
            }
        }
        low_points
    }

    fn is_low_point(self: &Map, i: usize, j: usize, v: u32) -> bool {
        if i > 0 && self.data[i - 1][j] <= v {
            return false;
        }
        if j > 0 && self.data[i][j - 1] <= v {
            return false;
        }
        if i < self.data.len() - 1 && self.data[i + 1][j] <= v {
            return false;
        }
        if j < self.data[i].len() - 1 && self.data[i][j + 1] <= v {
            return false;
        }
        true
    }
}

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Map> {
    let lines = shared::read_input(DAY)?;
    let data: Vec<Vec<u32>> = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    Ok(Map { data })
}

fn part1(input: &Map) -> u32 {
    input.find_low_points().iter().map(|r| r + 1).sum()
}

fn part2(input: &Map) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<i32> = Vec::new();
    for (i, r) in input.data.iter().enumerate() {
        for (j, &v) in r.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }
            if v >= 9 {
                visited.insert((i, j));
                continue;
            }
            basins.push(crawl_basin(input, &mut visited, i, j));
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn crawl_basin(input: &Map, visited: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> i32 {
    if visited.contains(&(i, j)) {
        return 0;
    }
    visited.insert((i, j));
    if input.data[i][j] >= 9 {
        return 0;
    }

    let mut count = 1;

    if i > 0 {
        count += crawl_basin(input, visited, i - 1, j);
    }
    if j > 0 {
        count += crawl_basin(input, visited, i, j - 1);
    }
    if i < input.data.len() - 1 {
        count += crawl_basin(input, visited, i + 1, j);
    }
    if j < input.data[i].len() - 1 {
        count += crawl_basin(input, visited, i, j + 1);
    }

    count
}
