use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}

fn problem1(input: &[&str]) -> Result<i32> {
    let mut score = 0;
    for line in input {
        if let Some((opp, player)) = line.split_once(' ') {
            for (opp_play, player_play) in opp.chars().zip(player.chars()) {
                match (opp_play, player_play) {
                    ('A', 'X') => score += 3 + 1,
                    ('A', 'Y') => score += 6 + 2,
                    ('A', 'Z') => score += 3,
                    ('B', 'X') => score += 1,
                    ('B', 'Y') => score += 3 + 2,
                    ('B', 'Z') => score += 6 + 3,
                    ('C', 'X') => score += 6 + 1,
                    ('C', 'Y') => score += 2,
                    ('C', 'Z') => score += 3 + 3,
                    _ => (),
                }
            }
        }
    }
    Ok(score)
}

fn problem2(input: &[&str]) -> Result<i32> {
    let mut built_plays = Vec::new();
    for line in input {
        if let Some((opp, outcome)) = line.split_once(' ') {
            for (opp_play, outcome) in opp.chars().zip(outcome.chars()) {
                match (opp_play, outcome) {
                    ('A', 'X') => built_plays.push("A Z"),
                    ('A', 'Y') => built_plays.push("A X"),
                    ('A', 'Z') => built_plays.push("A Y"),
                    ('B', 'X') => built_plays.push("B X"),
                    ('B', 'Z') => built_plays.push("B Z"),
                    ('B', 'Y') => built_plays.push("B Y"),
                    ('C', 'X') => built_plays.push("C Y"),
                    ('C', 'Y') => built_plays.push("C Z"),
                    ('C', 'Z') => built_plays.push("C X"),
                    _ => (),
                }
            }
        }
    }
    let score = problem1(built_plays.as_slice())?;
    Ok(score)
}
