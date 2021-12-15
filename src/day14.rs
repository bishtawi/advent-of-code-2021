use std::collections::HashMap;

use crate::shared;
use anyhow::Result;

const DAY: i32 = 14;

pub fn solve() -> Result<()> {
    let (start, rules) = parse()?;
    println!(
        "Day {} = {} :: {}",
        DAY,
        part1(&start, &rules),
        part2(&start, &rules)
    );

    Ok(())
}

fn parse() -> Result<(String, HashMap<String, char>)> {
    let mut lines = shared::read_input(DAY)?;
    let start = lines.next().unwrap()?;
    assert_eq!("", lines.next().unwrap()?);
    let rules: HashMap<String, char> = lines
        .map(|line| {
            let line = line.unwrap();
            let pair: Vec<&str> = line.split(" -> ").collect();
            assert_eq!(pair.len(), 2);
            assert_eq!(pair[0].len(), 2);
            assert_eq!(pair[1].len(), 1);
            (pair[0].to_owned(), pair[1].chars().next().unwrap())
        })
        .collect();
    Ok((start, rules))
}

fn part1(start: &str, rules: &HashMap<String, char>) -> i32 {
    // Naive approach which doesnt scale well enough to solve part2
    let mut current = start.to_owned();
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in current.chars() {
        let val = counts.entry(c).or_insert(0);
        *val += 1;
    }
    for _ in 0..10 {
        let mut new = current.chars().next().unwrap().to_string();
        for i in 0..current.len() - 1 {
            if let Some(&insertion) = rules.get(&current[i..=i + 1]) {
                new.push(insertion);
                let val = counts.entry(insertion).or_insert(0);
                *val += 1;
            }
            new.push(current.chars().nth(i + 1).unwrap());
        }
        current = new;
    }

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &count in counts.values() {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }

    max - min
}

fn part2(start: &str, rules: &HashMap<String, char>) -> u64 {
    // Proper algorithm that does scale
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    let mut pairs: HashMap<String, u64> = HashMap::new();
    for c in start.chars() {
        let val = char_counts.entry(c).or_insert(0);
        *val += 1;
    }
    for i in 0..start.len() - 1 {
        let count = pairs.entry(start[i..=i + 1].to_string()).or_insert(0);
        *count += 1;
    }
    for _ in 0..40 {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        for (pair, count) in pairs {
            if let Some(insertion) = rules.get(&pair) {
                let val = char_counts.entry(*insertion).or_insert(0);
                *val += count;
                let mut chars = pair.chars();
                let left_pair = format!("{}{}", chars.next().unwrap(), insertion);
                let right_pair = format!("{}{}", insertion, chars.next().unwrap());
                let pair_count = new_pairs.entry(left_pair).or_insert(0);
                *pair_count += count;
                let pair_count = new_pairs.entry(right_pair).or_insert(0);
                *pair_count += count;
            } else {
                let pair_count = new_pairs.entry(pair).or_insert(0);
                *pair_count += count;
            }
        }
        pairs = new_pairs;
    }
    let mut min = u64::MAX;
    let mut max = u64::MIN;
    for &count in char_counts.values() {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }

    max - min
}
