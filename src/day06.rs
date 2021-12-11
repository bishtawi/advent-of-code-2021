use crate::shared;
use anyhow::Result;

const DAY: i32 = 6;

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn parse() -> Result<Vec<u8>> {
    let contents = shared::read_input_as_string(DAY)?;
    Ok(contents
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect())
}

fn lifecycle(fishes: &mut Vec<u8>, cycles: i32) {
    for _ in 0..cycles {
        for i in (0..fishes.len()).rev() {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let mut fishes = input.to_vec();
    lifecycle(&mut fishes, 80);
    fishes.len()
}

fn part2(input: &[u8]) -> usize {
    let fish_decendents_128: Vec<Vec<u8>> = (0..=8)
        .map(|cycle| {
            let mut fishes = vec![cycle];
            lifecycle(&mut fishes, 128);
            fishes
        })
        .collect();

    let fish_decendent_count_256: Vec<usize> = (0_u8..=6)
        .map(|cycle| {
            fish_decendents_128[cycle as usize]
                .iter()
                .map(|&c| fish_decendents_128[c as usize].len())
                .sum()
        })
        .collect();

    input
        .iter()
        .map(|&f| fish_decendent_count_256[f as usize])
        .sum()
}
