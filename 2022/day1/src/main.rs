use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Problem 1: {}", problem1(input)?);
    println!("Problem 2: {}", problem2(input)?);
    Ok(())
}

fn problem1(input: &str) -> Result<i32> {
    let mut elves: Vec<i32> = input
        .split_terminator("\n\n")
        .map(|line| {
            line.lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum()
        })
        .collect();
    elves.sort();
    elves.pop().ok_or_else(|| anyhow!("empty vector"))
}

fn problem2(input: &str) -> Result<i32> {
    let mut elves: Vec<i32> = input
        .split_terminator("\n\n")
        .map(|line| {
            line.lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum()
        })
        .collect();
    elves.sort();
    Ok(elves.iter().rev().take(3).sum())
}
