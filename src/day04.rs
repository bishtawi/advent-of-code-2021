use crate::shared;
use anyhow::Result;

const DAY: i32 = 4;
const SIZE: usize = 5;

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Unmarked,
    Marked,
}

#[derive(Clone)]
struct Board {
    data: [[(i32, Status); SIZE]; SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            data: [[(0, Status::Unmarked); SIZE]; SIZE],
        }
    }

    fn play(self: &mut Board, num: i32) -> bool {
        let (mut i, mut j) = (usize::MAX, usize::MAX);
        'outer: for (r, row) in self.data.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if col.0 == num {
                    i = r;
                    j = c;
                    break 'outer;
                }
            }
        }
        if i == usize::MAX {
            return false;
        }
        self.data[i][j].1 = Status::Marked;

        (0..SIZE).all(|n| self.data[i][n].1 == Status::Marked)
            || (0..SIZE).all(|n| self.data[n][j].1 == Status::Marked)
    }

    fn score(self: &Board, value: i32) -> i32 {
        let sum = self.data.iter().fold(0, |acc, row| {
            let row_sum: i32 = row
                .iter()
                .filter_map(|(v, status)| {
                    if status == &Status::Unmarked {
                        Some(v)
                    } else {
                        None
                    }
                })
                .sum();
            acc + row_sum
        });
        sum * value
    }
}

pub fn solve() -> Result<()> {
    let (nums, boards) = parse()?;

    println!(
        "Day {} = {} :: {}",
        DAY,
        part1(&nums, &boards),
        part2(&nums, &boards)
    );

    Ok(())
}

fn parse() -> Result<(Vec<i32>, Vec<Board>)> {
    let mut lines = shared::read_input(DAY)?;
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut i = 0;
    let mut curr = Board::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            assert_eq!(i, 0);
            continue;
        }

        for (j, v) in line
            .trim()
            .split(' ')
            .filter(|c| !c.is_empty())
            .map(|c| c.trim().parse::<i32>().unwrap())
            .enumerate()
        {
            curr.data[i][j] = (v, Status::Unmarked);
        }

        if i < SIZE - 1 {
            i += 1;
        } else {
            i = 0;
            boards.push(curr);
            curr = Board::new();
        }
    }

    Ok((nums, boards))
}

fn part1(nums: &[i32], boards: &[Board]) -> i32 {
    let mut boards = boards.to_vec();

    for &num in nums {
        for board in &mut boards {
            if board.play(num) {
                return board.score(num);
            }
        }
    }

    -1
}

fn part2(nums: &[i32], boards: &[Board]) -> i32 {
    let mut boards = boards.to_vec();

    for &num in nums {
        let mut i = 0;
        while i < boards.len() {
            if boards[i].play(num) {
                if boards.len() == 1 {
                    return boards[i].score(num);
                }
                boards.remove(i);
            } else {
                i += 1;
            }
        }
    }

    -1
}
