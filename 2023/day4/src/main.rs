#![deny(clippy::all, clippy::pedantic)]

use std::collections::{HashSet, HashMap};

use anyhow::{anyhow, Result, Ok};

fn problem1(input: &[&str]) -> u32 {
    let mut sum = 0;
    for line in input {
        let splits = line.split(&[':', '|']).collect::<Vec<_>>();
        let winning_nums = splits[1]
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .flat_map(str::parse::<u32>)
            .collect::<HashSet<_>>();
        let my_nums = splits[2]
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .flat_map(str::parse::<u32>)
            .collect::<HashSet<_>>();
        let intersection = my_nums.intersection(&winning_nums);
        let mut doubled = 0;
        for _num in intersection {
            if doubled == 0 {
                doubled = 1;
            } else {
                doubled *= 2;
            }
        }
        sum += doubled;
    }
    sum
}

fn problem2(input: &[&str]) -> Result<usize> {
    let mut card_count = HashMap::new();
    for i in 1..=input.len() {
        card_count.insert(i, 1);
    }
    for (i, line) in input.iter().enumerate() {
        if i == input.len() -1 {
            break;
        }
        let splits = line.split(&[':', '|']).collect::<Vec<_>>();
        let winning_nums = splits[1]
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .flat_map(str::parse::<u32>)
            .collect::<HashSet<_>>();
        let my_nums = splits[2]
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .flat_map(str::parse::<u32>)
            .collect::<HashSet<_>>();
        let intersection = my_nums.intersection(&winning_nums);
        for j in 1..=intersection.count() {
            let count = card_count.get(&(i + 1)).ok_or(anyhow!("No card count found"))?;
            let current_count = card_count.get(&(i + j + 1)).ok_or(anyhow!("No current count found"))?;
            card_count.insert(i + j + 1, current_count + count);
        }
    }
    let mut sum = 0;
    for value in card_count.values() {
        sum += value;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
