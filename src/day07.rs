use crate::shared;
use anyhow::Result;

const DAY: i32 = 7;

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<i32>> {
    let contents = shared::read_input_as_string(DAY)?;
    let mut values: Vec<i32> = contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    values.sort_unstable();
    Ok(values)
}

fn part1(input: &[i32]) -> i32 {
    let median = input[input.len() / 2];
    input.iter().map(|n| i32::abs(n - median)).sum()
}

fn part2(input: &[i32]) -> i32 {
    let mut min = i32::MAX;
    for n in *input.first().unwrap()..=*input.last().unwrap() {
        let cost: i32 = input.iter().map(|&p| cost(n, p)).sum();
        if min > cost {
            min = cost;
        }
    }
    min
}

fn cost(n: i32, p: i32) -> i32 {
    let val = i32::abs(p - n);
    ((val * val) + val) / 2
}
