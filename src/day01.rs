use crate::shared;
use anyhow::Result;

const WINDOW_SIZE: usize = 3;

#[allow(dead_code)]
pub fn solve(day: i32) -> Result<()> {
    println!("Day {} = {}::{}", day, part1(day)?, part2(day)?);

    Ok(())
}

fn part1(day: i32) -> Result<i32> {
    let lines = shared::read_input(day)?;
    let (count, _) = lines.fold((0, i32::MAX), |(count, prev), line| {
        let cur = line.unwrap().parse::<i32>().unwrap();
        if prev >= cur {
            (count, cur)
        } else {
            (count + 1, cur)
        }
    });

    Ok(count)
}

fn part2(day: i32) -> Result<i32> {
    let lines = shared::read_input(day)?;

    let mut windows: Vec<Vec<i32>> = Vec::new();
    let mut prev = i32::MAX;
    let mut count = 0_i32;
    for line in lines {
        if windows.len() < WINDOW_SIZE {
            windows.push(Vec::new());
        }
        let val = line?.parse::<i32>()?;
        for window in &mut windows {
            window.push(val);
            if window.len() >= WINDOW_SIZE {
                let curr: i32 = window.iter().sum();
                if prev < curr {
                    count += 1;
                }
                prev = curr;
                window.clear();
            }
        }
    }

    Ok(count)
}
