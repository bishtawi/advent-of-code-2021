use crate::shared;
use anyhow::Result;

const DAY: i32 = 11;

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<Vec<u32>>> {
    let lines = shared::read_input(DAY)?;
    Ok(lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect())
}

fn step(grid: &mut [Vec<u32>]) -> usize {
    let mut flashes: Vec<(usize, usize)> = Vec::new();
    for (i, r) in grid.iter_mut().enumerate() {
        for (j, v) in r.iter_mut().enumerate() {
            *v += 1;
            if *v > 9 {
                assert_eq!(*v, 10);
                flashes.push((i, j));
            }
        }
    }
    let mut index = 0;
    while index < flashes.len() {
        let (i, j) = flashes[index];
        if i > 0 {
            handle_neighbor(grid, &mut flashes, i - 1, j);
        }
        if j > 0 {
            handle_neighbor(grid, &mut flashes, i, j - 1);
        }
        if i < grid.len() - 1 {
            handle_neighbor(grid, &mut flashes, i + 1, j);
        }
        if j < grid[i].len() - 1 {
            handle_neighbor(grid, &mut flashes, i, j + 1);
        }
        if i > 0 && j > 0 {
            handle_neighbor(grid, &mut flashes, i - 1, j - 1);
        }
        if i < grid.len() - 1 && j > 0 {
            handle_neighbor(grid, &mut flashes, i + 1, j - 1);
        }
        if i < grid.len() - 1 && j < grid[i].len() - 1 {
            handle_neighbor(grid, &mut flashes, i + 1, j + 1);
        }
        if i > 0 && j < grid[i].len() - 1 {
            handle_neighbor(grid, &mut flashes, i - 1, j + 1);
        }
        index += 1;
    }
    for (i, j) in &flashes {
        grid[*i][*j] = 0;
    }
    flashes.len()
}

fn handle_neighbor(grid: &mut [Vec<u32>], flashes: &mut Vec<(usize, usize)>, i: usize, j: usize) {
    grid[i][j] += 1;
    if grid[i][j] == 10 {
        flashes.push((i, j));
    }
}

fn part1(input: &[Vec<u32>]) -> usize {
    let mut grid = input.to_owned();
    (0..100).map(|_| step(&mut grid)).sum()
}

fn part2(input: &[Vec<u32>]) -> i32 {
    let mut grid = input.to_owned();
    let mut index = 0;
    loop {
        index += 1;
        if step(&mut grid) == 100 {
            return index;
        }
    }
}
