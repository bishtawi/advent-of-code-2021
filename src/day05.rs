use crate::shared;
use anyhow::Result;
use std::collections::HashMap;

const DAY: i32 = 5;

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_hor_or_vert(self: &Line) -> bool {
        self.is_hor() || self.is_vert()
    }

    fn is_hor(self: &Line) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vert(self: &Line) -> bool {
        self.start.0 == self.end.0
    }

    fn all_points(self: &Line) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let hor = self.is_hor();
        let vert = self.is_vert();

        if hor {
            let min = i32::min(self.start.0, self.end.0);
            let max = i32::max(self.start.0, self.end.0);
            for x in min..=max {
                points.push((x, self.start.1));
            }
        } else if vert {
            let min = i32::min(self.start.1, self.end.1);
            let max = i32::max(self.start.1, self.end.1);
            for y in min..=max {
                points.push((self.start.0, y));
            }
        } else {
            // diag
            let x_inc = self.start.0 < self.end.0;
            let y_inc = self.start.1 < self.end.1;
            let mut x = self.start.0;
            let mut y = self.start.1;
            while x != self.end.0 {
                points.push((x, y));
                if x_inc {
                    x += 1;
                } else {
                    x -= 1;
                }
                if y_inc {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
            assert_eq!(y, self.end.1);
            points.push((x, y));
        }

        points
    }
}

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    let vents = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&vents), part2(&vents));

    Ok(())
}

fn parse() -> Result<Vec<Line>> {
    let lines = shared::read_input(DAY)?;
    Ok(lines
        .map(|line| {
            let line = line.unwrap();
            let chunks: Vec<_> = line.split(" -> ").collect();
            assert_eq!(chunks.len(), 2);
            let start: Vec<i32> = chunks[0]
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect();
            let end: Vec<i32> = chunks[1]
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect();
            assert_eq!(start.len(), 2);
            assert_eq!(end.len(), 2);
            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        })
        .collect())
}

fn part1(lines: &[Line]) -> usize {
    let mut overlaps: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines.iter().filter(|l| l.is_hor_or_vert()) {
        for point in line.all_points() {
            let entry = overlaps.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    overlaps.iter().filter(|(_, &v)| v >= 2).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut overlaps: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        for point in line.all_points() {
            let entry = overlaps.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    overlaps.iter().filter(|(_, &v)| v >= 2).count()
}
