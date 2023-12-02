#![deny(clippy::all, clippy::pedantic)]

use anyhow::{anyhow, Result};

fn problem1(input: &[&str]) -> Result<usize> {
    let mut sum = 0;
    for line in input {
        let mut pair = Vec::with_capacity(2);
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                if let Some(digit) = letter.to_digit(10) {
                    pair.push(digit);
                } else {
                    return Err(anyhow!("Failed to parse char as digit"));
                }
            }
        }
        let first = pair.first().ok_or(anyhow!("No first number"))?;
        let last = pair.last().ok_or(anyhow!("No last number"))?;
        sum += format!("{first}{last}").parse::<usize>()?;
    }
    Ok(sum)
}

fn problem2(input: &[&str]) -> Result<usize> {
    let words: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    for line in input {
        let mut seq = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if let Some(digit) = c.to_digit(10) {
                    seq.push(digit as usize);
                } else {
                    return Err(anyhow!("Invalid digit"));
                }
            } else {
                let (_, back) = line.split_at(i);
                for (i, word) in words.iter().enumerate() {
                    if back.starts_with(word) {
                        seq.push(i + 1);
                    }
                }
            }
        }
        let first = seq.first().ok_or(anyhow!("No first number"))?;
        let last = seq.last().ok_or(anyhow!("No last number"))?;
        sum += format!("{first}{last}").parse::<usize>()?;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
