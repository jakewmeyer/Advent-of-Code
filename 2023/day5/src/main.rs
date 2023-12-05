#![deny(clippy::all, clippy::pedantic)]

use anyhow::{anyhow, Result};

fn problem1(input: &[&str]) -> Result<u32> {
    todo!()
}

fn problem2(input: &[&str]) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
