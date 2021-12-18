use crate::shared;
use anyhow::Result;
use regex::Regex;

const DAY: i32 = 17;

struct Range {
    x: (i32, i32),
    y: (i32, i32),
}

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Range> {
    let line = shared::read_input_as_string(DAY)?;
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$")?;
    let cap = re.captures(&line).unwrap();
    Ok(Range {
        x: (cap[1].parse()?, cap[2].parse()?),
        y: (cap[3].parse()?, cap[4].parse()?),
    })
}

fn part1(input: &Range) -> i32 {
    (0..(-input.y.0)).sum()
}

fn part2(input: &Range) -> i32 {
    #[allow(clippy::cast_possible_truncation)]
    let min_x = ((-1.0 + f64::from(1 + 4 * 2 * input.x.0).sqrt()) / 2.0).ceil() as i32;
    let max_x = input.x.1;
    let min_y = input.y.0;
    let max_y: i32 = (0..(-input.y.0)).sum();

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut x_velocity = x;
            let mut y_velocity = y;
            let mut x_pos = 0;
            let mut y_pos = 0;
            while x_pos <= input.x.1 && y_pos >= input.y.0 {
                if x_pos >= input.x.0 && y_pos <= input.y.1 {
                    count += 1;
                    break;
                }
                x_pos += x_velocity;
                y_pos += y_velocity;
                if x_velocity > 0 {
                    x_velocity -= 1;
                }
                y_velocity -= 1;
            }
        }
    }
    count
}
