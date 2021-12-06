use crate::shared;
use anyhow::Result;

const DAY: i32 = 1;
const WINDOW_SIZE: usize = 3;

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    println!("Day {} = {} :: {}", DAY, part1()?, part2()?);

    Ok(())
}

fn part1() -> Result<i32> {
    let lines = shared::read_input(DAY)?;
    let (count, _) = lines.fold(Ok((0, i32::MAX)), |acc: Result<(i32, i32)>, line| {
        let (count, prev) = acc?;
        let cur = line?.parse::<i32>()?;
        if prev >= cur {
            Ok((count, cur))
        } else {
            Ok((count + 1, cur))
        }
    })?;

    Ok(count)
}

fn part2() -> Result<i32> {
    let lines = shared::read_input(DAY)?;

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
