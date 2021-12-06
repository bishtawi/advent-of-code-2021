use crate::shared;
use anyhow::{bail, Result};

const DAY: i32 = 2;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[allow(dead_code)]
pub fn solve() -> Result<()> {
    let cmds = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&cmds), part2(&cmds));

    Ok(())
}

fn parse() -> Result<Vec<Command>> {
    let lines = shared::read_input(DAY)?;
    lines.fold(Ok(Vec::new()), |acc, line| {
        let mut commands = acc?;
        let line = line?;
        let segments: Vec<_> = line.split(' ').collect();
        assert_eq!(segments.len(), 2);
        let value = segments[1].parse::<i32>()?;
        let command = match segments.get(0) {
            Some(&"forward") => Command::Forward(value),
            Some(&"down") => Command::Down(value),
            Some(&"up") => Command::Up(value),
            x => bail!("Unexpected command: {:?}", x),
        };
        commands.push(command);
        Ok(commands)
    })
}

fn part1(cmds: &[Command]) -> i32 {
    let (horz, depth) = cmds.iter().fold((0, 0), |(horz, depth), cmd| match cmd {
        Command::Forward(value) => (horz + value, depth),
        Command::Down(value) => (horz, depth + value),
        Command::Up(value) => (horz, depth - value),
    });

    horz * depth
}

fn part2(cmds: &[Command]) -> i32 {
    let (horz, depth, _) = cmds
        .iter()
        .fold((0, 0, 0), |(horz, depth, aim), cmd| match cmd {
            Command::Forward(value) => (horz + value, depth + aim * value, aim),
            Command::Down(value) => (horz, depth, aim + value),
            Command::Up(value) => (horz, depth, aim - value),
        });

    horz * depth
}
