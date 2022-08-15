use anyhow::{anyhow, Ok, Result};

/// For a bitstring index, determine the most common bit in a slice of
/// bitstrings.
pub fn most_common_bit(index: usize, input: &[&str]) -> char {
    let mut zero_count = 0;
    let mut one_count = 0;
    for bitstring in input {
        if bitstring.chars().nth(index) == Some('0') {
            zero_count += 1;
        } else {
            one_count += 1;
        }
    }
    if one_count >= zero_count {
        '1'
    } else {
        '0'
    }
}

/// This accepts a 2D vector of char "bits" from the report. Gamma is defined as the
/// most common bit occurrence for each column, while epsilon is the least common
/// for each column. The resulting bits from each column form a binary number, and
/// power consumption is the product of the two.
pub fn problem1(input: &[&str]) -> Result<u32> {
    let columns = input.first().ok_or_else(|| anyhow!("Invalid index"))?.len();
    let mut gamma_bits = String::new();
    let mut epsilon_bits = String::new();

    for column in 0..columns {
        match most_common_bit(column, input) {
            '1' => {
                gamma_bits += "1";
                epsilon_bits += "0";
            }
            '0' => {
                gamma_bits += "0";
                epsilon_bits += "1";
            }
            _ => return Err(anyhow!("Invalid common bit")),
        };
    }
    let gamma = u32::from_str_radix(&gamma_bits, 2)?;
    let epsilon = u32::from_str_radix(&epsilon_bits, 2)?;
    Ok(gamma * epsilon)
}

/// This accepts the same 2D vector as problem 1, but requires significantly
/// more bit filtering. If the current bit index does not have the most common bit, the
/// number is filtered from the list
fn problem2(input: &[&str]) -> Result<u32> {
    let mut o2_candidates = input.to_vec();
    let mut o2_index = 0;
    while o2_candidates.len() > 1 {
        let most_common = most_common_bit(o2_index, &o2_candidates);
        o2_candidates = o2_candidates
            .into_iter()
            .filter(|c| c.chars().nth(o2_index) == Some(most_common))
            .collect();
        o2_index += 1;
    }
    let o2_rating = u32::from_str_radix(o2_candidates.first().unwrap(), 2)?;

    let mut co2_candidates = input.to_vec();
    let mut co2_index = 0;
    while co2_candidates.len() > 1 {
        let least_common = match most_common_bit(co2_index, &co2_candidates) {
            '0' => '1',
            '1' => '0',
            _ => return Err(anyhow!("Invalid least common bit")),
        };
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|c| c.chars().nth(co2_index) == Some(least_common))
            .collect();
        co2_index += 1;
    }
    let co2_rating = u32::from_str_radix(co2_candidates.first().unwrap(), 2)?;

    Ok(o2_rating * co2_rating)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();
    println!("Problem 1: {}", problem1(&input)?);
    println!("Problem 2: {}", problem2(&input)?);
    Ok(())
}
