use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Problem 1: {}", problem1(&input)?);
    // println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}

fn problem1(input: &str) -> Result<i32> {
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let mut first: Vec<u8> = first.bytes().collect();
        first.sort();
        dbg!(first);
    }
    Ok(1)
}
