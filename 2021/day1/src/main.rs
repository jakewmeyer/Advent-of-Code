use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}

fn problem1(depths: &[i32]) -> Result<usize> {
    let count = depths
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .filter(|&n| n > 0)
        .count();
    Ok(count)
}

fn problem2(depths: &[i32]) -> Result<usize> {
    let windows = depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| (a + b + c))
        .collect::<Vec<_>>();
    let count = windows
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a))
        .filter(|&n| n > 0)
        .count();
    Ok(count)
}
