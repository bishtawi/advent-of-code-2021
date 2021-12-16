use crate::shared;
use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const DAY: i32 = 15;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<Vec<u32>>> {
    let lines = shared::read_input(DAY)?;
    let grid: Vec<Vec<u32>> = lines
        .map(|line| {
            let line = line.unwrap();
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();
    Ok(grid)
}

fn part1(grid: &[Vec<u32>]) -> usize {
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist.insert((0, 0), 0);
    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(state) = heap.pop() {
        if state.position == (grid.len() - 1, grid[grid.len() - 1].len() - 1) {
            return state.cost;
        }

        if state.cost > *dist.get(&state.position).unwrap_or(&usize::MAX) {
            continue;
        }

        let (x, y) = state.position;
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < grid.len() {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < grid[x].len() {
            neighbors.push((x, y + 1));
        }
        for (x, y) in neighbors {
            let next = State {
                cost: state.cost + (grid[x][y] as usize),
                position: (x, y),
            };
            if next.cost < *dist.get(&(x, y)).unwrap_or(&usize::MAX) {
                dist.insert((x, y), next.cost);
                heap.push(next);
            }
        }
    }

    0
}

fn part2(grid: &[Vec<u32>]) -> usize {
    let scale = 5;
    let end_position = (
        grid.len() * scale - 1,
        grid[grid.len() - 1].len() * scale - 1,
    );
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist.insert((0, 0), 0);
    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(state) = heap.pop() {
        if state.position == end_position {
            return state.cost;
        }

        if state.cost > *dist.get(&state.position).unwrap_or(&usize::MAX) {
            continue;
        }

        let (x, y) = state.position;
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < grid.len() * scale {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < grid[x % grid.len()].len() * scale {
            neighbors.push((x, y + 1));
        }
        for (x, y) in neighbors {
            let val = (grid[x % grid.len()][y % grid[x % grid.len()].len()] as usize)
                + x / grid.len()
                + y / grid[x % grid.len()].len();
            let next_cost = if val > 9 { val - 9 } else { val };

            let next = State {
                cost: state.cost + next_cost,
                position: (x, y),
            };
            if next.cost < *dist.get(&(x, y)).unwrap_or(&usize::MAX) {
                dist.insert((x, y), next.cost);
                heap.push(next);
            }
        }
    }

    0
}
