use crate::shared;
use anyhow::{bail, Result};

const DAY: i32 = 18;

#[derive(Clone, Debug)]
enum Type {
    Open,
    Close,
    Comma,
    Value(u32),
}

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input)?, part2(&input)?);

    Ok(())
}

fn parse() -> std::io::Result<Vec<Vec<Type>>> {
    let lines = shared::read_input(DAY)?;
    lines
        .map(|l| {
            l.map(|l| {
                l.chars()
                    .map(|c| match c {
                        '[' => Type::Open,
                        ']' => Type::Close,
                        ',' => Type::Comma,
                        x => Type::Value(x.to_digit(10).unwrap()),
                    })
                    .collect()
            })
        })
        .collect()
}

#[allow(clippy::too_many_lines)] // Normally I would break this function into multiple small ones but for AoC its not worth the refactor
fn part1(items: &[Vec<Type>]) -> Result<u32> {
    let mut current: Vec<Type> = Vec::new();
    for item in items {
        if current.is_empty() {
            current = item.clone();
            continue;
        }
        let mut combined = vec![Type::Open];
        combined.append(&mut current);
        combined.push(Type::Comma);
        combined.extend_from_slice(item);
        combined.push(Type::Close);

        // Reduce
        loop {
            // Explode
            let mut count = 0;
            let mut index = usize::MAX;
            for (i, c) in combined.iter().enumerate() {
                match c {
                    Type::Open => count += 1,
                    Type::Close => count -= 1,
                    _ => continue,
                };
                if count == 5 {
                    index = i;
                    break;
                }
            }
            if index < combined.len() {
                let left = match &combined[index + 1] {
                    Type::Value(val) => *val,
                    x => bail!("Unexpected type {:?}", x),
                };
                let right = match &combined[index + 3] {
                    Type::Value(val) => *val,
                    x => bail!("Unexpected type {:?}", x),
                };
                combined.remove(index); // [
                combined.remove(index); // left
                combined.remove(index); // ,
                combined.remove(index); // right
                combined[index] = Type::Value(0); // ]

                for i in (0..index).rev() {
                    match combined.get(i) {
                        None => bail!("shouldnt happen"),
                        Some(Type::Open | Type::Close | Type::Comma) => continue,
                        Some(Type::Value(val)) => {
                            combined[i] = Type::Value(val + left);
                            break;
                        }
                    }
                }

                for i in index + 1..combined.len() {
                    match combined.get(i) {
                        None => bail!("shouldnt happen"),
                        Some(Type::Open | Type::Close | Type::Comma) => continue,
                        Some(Type::Value(val)) => {
                            combined[i] = Type::Value(val + right);
                            break;
                        }
                    }
                }

                continue;
            }

            // Split
            let mut index = usize::MAX;
            for (i, c) in combined.iter().enumerate() {
                if let Type::Value(v) = c {
                    if v >= &10 {
                        index = i;
                        break;
                    }
                }
            }
            if index < combined.len() {
                let val = match &combined[index] {
                    Type::Value(val) => val,
                    x => bail!("Unexpected type {:?}", x),
                };
                let left = val / 2;
                let right = (val / 2) + if val % 2 == 0 { 0 } else { 1 };
                combined[index] = Type::Close;
                combined.insert(index, Type::Value(right));
                combined.insert(index, Type::Comma);
                combined.insert(index, Type::Value(left));
                combined.insert(index, Type::Open);
                continue;
            }

            break;
        }
        current = combined;
    }

    let mut stack: Vec<(u32, u32, bool)> = Vec::new();
    for c in current {
        match c {
            Type::Open => stack.push((0, 0, false)),
            Type::Close => {
                let (left, right, flag) = stack.pop().unwrap();
                assert!(flag);
                let val = left * 3 + right * 2;
                if stack.is_empty() {
                    return Ok(val);
                }
                let (left, right, flag) = stack.last_mut().unwrap();
                if *flag {
                    *right = val;
                } else {
                    *left = val;
                }
            }
            Type::Comma => {
                let (_, _, flag) = stack.last_mut().unwrap();
                assert!(!*flag);
                *flag = true;
            }
            Type::Value(val) => {
                let (left, right, flag) = stack.last_mut().unwrap();
                if *flag {
                    *right = val;
                } else {
                    *left = val;
                }
            }
        }
    }

    bail!("Parsing failed")
}

fn part2(input: &[Vec<Type>]) -> Result<u32> {
    let mut max = u32::MIN;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            let val = part1(&[input[i].clone(), input[j].clone()])?;
            if val > max {
                max = val;
            }

            let val = part1(&[input[j].clone(), input[i].clone()])?;
            if val > max {
                max = val;
            }
        }
    }

    Ok(max)
}
