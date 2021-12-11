use crate::shared;
use anyhow::{bail, Result};

const DAY: i32 = 3;

pub fn solve() -> Result<()> {
    let lines: Vec<String> = shared::read_input(DAY)?
        .map(std::result::Result::unwrap)
        .collect();
    println!("Day {} = {} :: {}", DAY, part1(&lines)?, part2(&lines)?);

    Ok(())
}

fn part1(lines: &[String]) -> Result<i32> {
    let mut counts: Vec<usize> = Vec::new();

    for line in lines.iter() {
        if counts.is_empty() {
            (0..line.len()).for_each(|_| counts.push(0));
        } else {
            assert_eq!(counts.len(), line.len());
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => continue,
                '1' => counts[i] += 1,
                e => bail!("Unexpected character: {}", e),
            }
        }
    }

    let mut gamma_bits = String::new();
    let mut epsilon_bits = String::new();
    for value in counts {
        if value * 2 >= lines.len() {
            gamma_bits.push('1');
            epsilon_bits.push('0');
        } else {
            gamma_bits.push('0');
            epsilon_bits.push('1');
        }
    }

    let gamma = i32::from_str_radix(&gamma_bits, 2)?;
    let epsilon = i32::from_str_radix(&epsilon_bits, 2)?;

    Ok(gamma * epsilon)
}

fn part2(lines: &[String]) -> Result<i32> {
    let mut oxygen = lines.to_vec();
    for i in 0..oxygen[0].len() {
        if oxygen.len() == 1 {
            break;
        }
        let bit = get_most_common_bit(&oxygen, i)?;
        oxygen.retain(|e| e.chars().nth(i).unwrap() == bit);
    }

    let mut co2 = lines.to_vec();
    for i in 0..co2[0].len() {
        if co2.len() == 1 {
            break;
        }
        let bit = get_most_common_bit(&co2, i)?;
        co2.retain(|e| e.chars().nth(i).unwrap() != bit);
    }

    assert_eq!(oxygen.len(), 1);
    assert_eq!(co2.len(), 1);

    let oxygen = i32::from_str_radix(&oxygen[0], 2)?;
    let co2 = i32::from_str_radix(&co2[0], 2)?;

    Ok(oxygen * co2)
}

fn get_most_common_bit(lines: &[String], index: usize) -> Result<char> {
    let mut count = 0;
    for line in lines {
        match line.chars().nth(index) {
            Some('0') => continue,
            Some('1') => count += 1,
            x => bail!("Unexpected character {:?}", x),
        }
    }
    if count * 2 >= lines.len() {
        Ok('1')
    } else {
        Ok('0')
    }
}
