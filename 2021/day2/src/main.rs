use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

enum Directions {
    Forward,
    Up,
    Down,
}

struct Command {
    direction: Directions,
    distance: i32,
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let direction = match split.next() {
            Some("forward") => Directions::Forward,
            Some("up") => Directions::Up,
            Some("down") => Directions::Down,
            _ => return Err(anyhow!("Unable to parse direction")),
        };
        let distance = split
            .next()
            .and_then(|i| i.parse::<i32>().ok())
            .ok_or_else(|| anyhow!("Unable to parse distance"))?;
        Ok(Command {
            direction,
            distance,
        })
    }
}

fn problem1(commands: &[Command]) -> Result<i32> {
    let mut horizontal = 0;
    let mut depth = 0;
    for c in commands {
        match c.direction {
            Directions::Forward => horizontal += c.distance,
            Directions::Up => depth -= c.distance,
            Directions::Down => depth += c.distance,
        };
    }
    Ok(horizontal * depth)
}

fn problem2(commands: &[Command]) -> Result<i32> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for c in commands {
        match c.direction {
            Directions::Forward => {
                horizontal += c.distance;
                depth += aim * c.distance;
            }
            Directions::Up => aim -= c.distance,
            Directions::Down => aim += c.distance,
        };
    }
    Ok(horizontal * depth)
}

fn main() -> Result<()> {
    let input: Vec<Command> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
