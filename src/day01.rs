use crate::shared;
use anyhow::Result;

const DAY: i32 = 1;
const WINDOW_SIZE: usize = 3;

pub fn solve() -> Result<()> {
    let values: Vec<i32> = shared::read_input(DAY)?
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("Day {} = {} :: {}", DAY, part1(&values), part2(&values));

    Ok(())
}

fn part1(values: &[i32]) -> i32 {
    let (count, _) = values.iter().fold((0, i32::MAX), |(count, prev), &cur| {
        if prev >= cur {
            (count, cur)
        } else {
            (count + 1, cur)
        }
    });

    count
}

fn part2(values: &[i32]) -> i32 {
    let mut windows: Vec<Vec<i32>> = Vec::new();
    let mut prev = i32::MAX;
    let mut count = 0_i32;
    for &val in values {
        if windows.len() < WINDOW_SIZE {
            windows.push(Vec::new());
        }
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

    count
}
