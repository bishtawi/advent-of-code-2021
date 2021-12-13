use crate::shared;
use anyhow::{bail, Result};
use std::collections::HashSet;

const DAY: i32 = 13;

type Point = (i32, i32);

enum Fold {
    X(i32),
    Y(i32),
}

pub fn solve() -> Result<()> {
    let (points, folds) = parse()?;
    println!(
        "Day {} = {} :: {}",
        DAY,
        part1(&points, &folds),
        part2(&points, &folds)
    );

    Ok(())
}

fn parse() -> Result<(HashSet<Point>, Vec<Fold>)> {
    let lines = shared::read_input(DAY)?;
    let mut flag = false;
    let mut folds: Vec<Fold> = Vec::new();
    let mut points: HashSet<Point> = HashSet::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            flag = true;
            continue;
        }
        if flag {
            let chunks: Vec<&str> = line.split('=').collect();
            assert_eq!(chunks.len(), 2);
            let val = chunks[1].parse::<i32>()?;
            let fold = match chunks.get(0) {
                Some(&"fold along y") => Fold::Y(val),
                Some(&"fold along x") => Fold::X(val),
                _ => bail!("Unexpected fold"),
            };
            folds.push(fold);
        } else {
            let chunks: Vec<&str> = line.split(',').collect();
            assert_eq!(chunks.len(), 2);
            let x = chunks[0].parse::<i32>()?;
            let y = chunks[1].parse::<i32>()?;
            points.insert((x, y));
        }
    }
    Ok((points, folds))
}

fn part1(points: &HashSet<(i32, i32)>, folds: &[Fold]) -> usize {
    apply_fold(points, folds.first().unwrap()).len()
}

fn apply_fold(points: &HashSet<(i32, i32)>, fold: &Fold) -> HashSet<(i32, i32)> {
    let mut new_points: HashSet<(i32, i32)> = HashSet::new();
    for &point in points {
        let new_point = match fold {
            Fold::X(x) => {
                if point.0 > *x {
                    (x - (point.0 - x), point.1)
                } else {
                    point
                }
            }
            Fold::Y(y) => {
                if point.1 > *y {
                    (point.0, y - (point.1 - y))
                } else {
                    point
                }
            }
        };
        new_points.insert(new_point);
    }
    new_points
}

fn part2(points: &HashSet<(i32, i32)>, folds: &[Fold]) -> &'static str {
    let mut points = points.clone();
    for fold in folds {
        points = apply_fold(&points, fold);
    }
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for (x, y) in &points {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y;
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    "JGAJEFKU"
}
