#![deny(clippy::all, clippy::pedantic)]

use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use regex::Regex;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_re = Regex::new("Game (?P<id>[0-9]+):")?
            .captures(s)
            .ok_or(anyhow!("Failed to capture game id"))?;
        let id = game_re
            .name("id")
            .ok_or(anyhow!("Failed to find game id match"))?
            .as_str()
            .parse::<u32>()?;
        let re = Regex::new("(?P<red>[0-9]{1,3}) red")?;
        let mut reds = Vec::new();
        for (_, [red]) in re.captures_iter(s).map(|c| c.extract()) {
            reds.push(red.parse::<u32>()?);
        }
        let re = Regex::new("(?P<green>[0-9]{1,3}) green")?;
        let mut greens = Vec::new();
        for (_, [green]) in re.captures_iter(s).map(|c| c.extract()) {
            greens.push(green.parse::<u32>()?);
        }
        let re = Regex::new("(?P<blue>[0-9]{1,3}) blue")?;
        let mut blues = Vec::new();
        for (_, [blue]) in re.captures_iter(s).map(|c| c.extract()) {
            blues.push(blue.parse::<u32>()?);
        }
        Ok(Game {
            id,
            red: *reds.iter().max().ok_or(anyhow!("Invalid red max"))?,
            green: *greens.iter().max().ok_or(anyhow!("Invalid green max"))?,
            blue: *blues.iter().max().ok_or(anyhow!("Invalid blue max"))?,
        })
    }
}

fn problem1(input: &[&str]) -> Result<u32> {
    let (red_max, green_max, blue_max) = (12, 13, 14);
    let mut ids = Vec::new();
    for line in input {
        let game = line.parse::<Game>()?;
        if red_max >= game.red && green_max >= game.green && blue_max >= game.blue {
            ids.push(game.id);
        }
    }
    Ok(ids.iter().sum())
}

fn problem2(input: &[&str]) -> Result<u32> {
    let mut powers = Vec::new();
    for line in input {
        let game = line.parse::<Game>()?;
        powers.push(game.red * game.blue * game.green);
    }
    Ok(powers.iter().sum())
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
