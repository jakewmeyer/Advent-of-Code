#![deny(clippy::all, clippy::pedantic)]

use anyhow::{anyhow, Result};

const SYMBOLS: [char; 12] = ['!', '@', '#', '$', '%', '^', '&', '*', '/', '=', '+', '-'];

fn problem1(input: &[&[u8]]) -> Result<u32> {
    let mut sum = 0;
    for (r, row) in input.iter().enumerate() {
        let mut number = String::new();
        let mut number_indices = Vec::new();
        for (c, &col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(col as char);
                number_indices.push(c);
            } else if number.is_empty() {
                continue;
            }
            if (col.is_ascii_digit() && !number.is_empty() && c == row.len() - 1)|| !col.is_ascii_digit() {
                dbg!(&number, &number_indices);
                let mut valid = false;
                let first = number_indices
                    .first()
                    .ok_or(anyhow!("Should have a first"))?;
                let last = number_indices.last().ok_or(anyhow!("Should have a last"))?;

                // Check one before if not at the beginning
                if *first != 0 && SYMBOLS.contains(&(input[r][first - 1] as char)) {
                    valid = true;
                }

                // Check one after if not at the end
                if *last != input[r].len() - 1 && SYMBOLS.contains(&(input[r][last + 1] as char)) {
                    valid = true;
                }

                if r != 0 {
                    let above = input[r - 1];

                    // Check one before if not at the beginning
                    if first.saturating_sub(1) != 0
                        && SYMBOLS.contains(&(above[first.saturating_sub(1)] as char))
                    {
                        valid = true;
                    }

                    // Check one after if not at the end
                    if last.saturating_add(1) != above.len()
                        && SYMBOLS.contains(&(above[last.saturating_add(1)] as char))
                    {
                        valid = true;
                    }

                    for &i in &number_indices {
                        if SYMBOLS.contains(&(above[i] as char)) {
                            valid = true;
                        }
                    }
                }

                if r != input.len() - 1 {
                    let below = input[r + 1];
                    // Check one before if not at the beginning
                    if first.saturating_sub(1) != 0
                        && SYMBOLS.contains(&(below[first.saturating_sub(1)] as char))
                    {
                        valid = true;
                    }
                    // Check one after if not at the end
                    if last.saturating_add(1) != below.len()
                        && SYMBOLS.contains(&(below[last.saturating_add(1)] as char))
                    {
                        valid = true;
                    }
                    for &i in &number_indices {
                        if SYMBOLS.contains(&(below[i] as char)) {
                            valid = true;
                        }
                    }
                }

                if valid {
                    let parsed = number.parse::<u32>()?;
                    dbg!(parsed);
                    sum += parsed;
                }
                number.clear();
                number_indices.clear();
            }
        }
    }
    Ok(sum)
}

fn problem2(input: &[&[u8]]) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt")
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
