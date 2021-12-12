use crate::shared;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

const DAY: i32 = 12;

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<HashMap<String, Vec<String>>> {
    let lines = shared::read_input(DAY)?;
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line?;
        let chunks: Vec<&str> = line.split('-').collect();
        assert_eq!(chunks.len(), 2);
        let dests = edges.entry(chunks[0].to_string()).or_insert_with(Vec::new);
        dests.push(chunks[1].to_string());
        let dests = edges.entry(chunks[1].to_string()).or_insert_with(Vec::new);
        dests.push(chunks[0].to_string());
    }
    Ok(edges)
}

fn part1(edges: &HashMap<String, Vec<String>>) -> i32 {
    let visited: HashSet<String> = HashSet::new();
    visit1(edges, visited, "start")
}

fn visit1(edges: &HashMap<String, Vec<String>>, mut visited: HashSet<String>, next: &str) -> i32 {
    if next == "end" {
        return 1;
    }
    if visited.contains(next) && next.to_lowercase() == next {
        return 0;
    }
    visited.insert(next.to_string());
    edges[next]
        .iter()
        .map(|n| visit1(edges, visited.clone(), n))
        .sum()
}

fn part2(edges: &HashMap<String, Vec<String>>) -> i32 {
    let visited: HashSet<String> = HashSet::new();
    visit2(edges, visited, "start", false)
}

fn visit2(
    edges: &HashMap<String, Vec<String>>,
    mut visited: HashSet<String>,
    next: &str,
    mut dup: bool,
) -> i32 {
    if next == "end" {
        return 1;
    }
    if visited.contains(next) && next.to_lowercase() == next {
        if dup || next == "start" {
            return 0;
        }
        dup = true;
    }
    visited.insert(next.to_string());
    edges[next]
        .iter()
        .map(|n| visit2(edges, visited.clone(), n, dup))
        .sum()
}
