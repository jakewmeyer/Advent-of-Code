#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
struct Mapping {
    dest: u64,
    source: u64,
    length: u64,
}

fn ranges_overlap(start1: u64, end1: u64, start2: u64, end2: u64) -> Option<Vec<u64>> {
    if start1 <= end2 && end1 >= start2 {
        let overlapping_start = std::cmp::max(start1, start2);
        let overlapping_end = std::cmp::min(end1, end2);
        Some((overlapping_start..=overlapping_end).collect())
    } else {
        None
    }
}

fn problem1(input: &str) -> Result<u64> {
    let seeds: Vec<u64> = input.lines().collect::<Vec<_>>()[0]
        .split(':')
        .collect::<Vec<_>>()[1]
        .split(' ')
        .flat_map(str::parse::<u64>)
        .collect();

    // Map seed to soil
    let seed_to_soil_re = Regex::new(r"seed-to-soil map:\n([\d\s]+)")?;
    let seed_to_soil = seed_to_soil_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let seed_to_soil = seed_to_soil.chunks(3).collect::<Vec<_>>();
    let mut seed_to_soil_mappings: Vec<Mapping> = Vec::new();
    for line in seed_to_soil {
        seed_to_soil_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut soils = Vec::new();
    for seed in seeds {
        let mut is_match = false;
        for mapping in &seed_to_soil_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&seed) {
                is_match = true;
                soils.push(mapping.dest + (seed - mapping.source));
            }
        }
        if !is_match {
            soils.push(seed);
        }
    }

    // Map soil to fertilizer
    let soil_to_fert_re = Regex::new(r"soil-to-fertilizer map:\n([\d\s]+)")?;
    let soil_to_fert = soil_to_fert_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let soil_to_fert = soil_to_fert.chunks(3).collect::<Vec<_>>();
    let mut soil_to_fert_mappings: Vec<Mapping> = Vec::new();
    for line in soil_to_fert {
        soil_to_fert_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut ferts = Vec::new();
    for soil in soils {
        let mut is_match = false;
        for mapping in &soil_to_fert_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&soil) {
                is_match = true;
                ferts.push(mapping.dest + (soil - mapping.source));
            }
        }
        if !is_match {
            ferts.push(soil);
        }
    }

    // Map fertilizer to water
    let fert_to_water_re = Regex::new(r"fertilizer-to-water map:\n([\d\s]+)")?;
    let fert_to_water = fert_to_water_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let fert_to_water = fert_to_water.chunks(3).collect::<Vec<_>>();
    let mut fert_to_water_mappings: Vec<Mapping> = Vec::new();
    for line in fert_to_water {
        fert_to_water_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut waters = Vec::new();
    for fert in ferts {
        let mut is_match = false;
        for mapping in &fert_to_water_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&fert) {
                is_match = true;
                waters.push(mapping.dest + (fert - mapping.source));
            }
        }
        if !is_match {
            waters.push(fert);
        }
    }

    // Map water to light
    let water_to_light_re = Regex::new(r"water-to-light map:\n([\d\s]+)")?;
    let water_to_light = water_to_light_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let water_to_light = water_to_light.chunks(3).collect::<Vec<_>>();
    let mut water_to_light_mappings: Vec<Mapping> = Vec::new();
    for line in water_to_light {
        water_to_light_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut lights = Vec::new();
    for water in waters {
        let mut is_match = false;
        for mapping in &water_to_light_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&water) {
                is_match = true;
                lights.push(mapping.dest + (water - mapping.source));
            }
        }
        if !is_match {
            lights.push(water);
        }
    }

    // Map light to temperature
    let light_to_temp_re = Regex::new(r"light-to-temperature map:\n([\d\s]+)")?;
    let light_to_temp = light_to_temp_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let light_to_temp = light_to_temp.chunks(3).collect::<Vec<_>>();
    let mut light_to_temp_mappings: Vec<Mapping> = Vec::new();
    for line in light_to_temp {
        light_to_temp_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut temps = Vec::new();
    for light in lights {
        let mut is_match = false;
        for mapping in &light_to_temp_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&light) {
                is_match = true;
                temps.push(mapping.dest + (light - mapping.source));
            }
        }
        if !is_match {
            temps.push(light);
        }
    }

    // Map temperature to humidity
    let temp_to_humidity_re = Regex::new(r"temperature-to-humidity map:\n([\d\s]+)")?;
    let temp_to_humidity = temp_to_humidity_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let temp_to_humidity = temp_to_humidity.chunks(3).collect::<Vec<_>>();
    let mut temp_to_humidity_mappings: Vec<Mapping> = Vec::new();
    for line in temp_to_humidity {
        temp_to_humidity_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut humidities = Vec::new();
    for temp in temps {
        let mut is_match = false;
        for mapping in &temp_to_humidity_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&temp) {
                is_match = true;
                humidities.push(mapping.dest + (temp - mapping.source));
            }
        }
        if !is_match {
            humidities.push(temp);
        }
    }

    // Map humidity to location
    let humidity_to_location_re = Regex::new(r"humidity-to-location map:\n([\d\s]+)")?;
    let humidity_to_location = humidity_to_location_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let humidity_to_location = humidity_to_location.chunks(3).collect::<Vec<_>>();
    let mut humidity_to_location_mappings: Vec<Mapping> = Vec::new();
    for line in humidity_to_location {
        humidity_to_location_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut locations = Vec::new();
    for humidity in humidities {
        let mut is_match = false;
        for mapping in &humidity_to_location_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&humidity) {
                is_match = true;
                locations.push(mapping.dest + (humidity - mapping.source));
            }
        }
        if !is_match {
            locations.push(humidity);
        }
    }
    // Return lowest location number corresponding to an initial seed
    Ok(*locations
        .iter()
        .min()
        .expect("There had better be a value here"))
}

fn problem2(input: &str) -> Result<u64> {
    let binding = input.lines().collect::<Vec<_>>()[0]
        .split(':')
        .collect::<Vec<_>>()[1]
        .split(' ')
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let seed_ranges = binding
        .chunks(2)
        .collect::<Vec<_>>();

    // Map seed to soil
    println!("Map seed to soil....");
    let seed_to_soil_re = Regex::new(r"seed-to-soil map:\n([\d\s]+)")?;
    let seed_to_soil = seed_to_soil_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let seed_to_soil = seed_to_soil.chunks(3).collect::<Vec<_>>();
    let mut seed_to_soil_mappings: Vec<Mapping> = Vec::new();
    for line in seed_to_soil {
        seed_to_soil_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut soils = Vec::new();
    println!("Checking seed ranges....");
    for seed_pair in seed_ranges {
        let mut is_match = false;
        for mapping in &seed_to_soil_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&seed_pair[0]) && range.contains(&(seed_pair[0] + seed_pair[1])) {
                is_match = true;
                let overlap = ranges_overlap(
                    range.start,
                    range.end,
                    seed_pair[0],
                    seed_pair[0] + seed_pair[1],
                )
                .ok_or(anyhow!("No overlapping"))?;
                for num in overlap {
                  soils.push(mapping.dest + (num - mapping.source));
                }
            }
        }
        if !is_match {
            soils.push(seed_pair[0]);
        }
    }

    // Map soil to fertilizer
    println!("Map soil to fertilizer....");
    let soil_to_fert_re = Regex::new(r"soil-to-fertilizer map:\n([\d\s]+)")?;
    let soil_to_fert = soil_to_fert_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let soil_to_fert = soil_to_fert.chunks(3).collect::<Vec<_>>();
    let mut soil_to_fert_mappings: Vec<Mapping> = Vec::new();
    for line in soil_to_fert {
        soil_to_fert_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut ferts = Vec::new();
    for soil in soils {
        let mut is_match = false;
        for mapping in &soil_to_fert_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&soil) {
                is_match = true;
                ferts.push(mapping.dest + (soil - mapping.source));
            }
        }
        if !is_match {
            ferts.push(soil);
        }
    }

    // Map fertilizer to water
    println!("Map fertilizer to water....");
    let fert_to_water_re = Regex::new(r"fertilizer-to-water map:\n([\d\s]+)")?;
    let fert_to_water = fert_to_water_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let fert_to_water = fert_to_water.chunks(3).collect::<Vec<_>>();
    let mut fert_to_water_mappings: Vec<Mapping> = Vec::new();
    for line in fert_to_water {
        fert_to_water_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut waters = Vec::new();
    for fert in ferts {
        let mut is_match = false;
        for mapping in &fert_to_water_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&fert) {
                is_match = true;
                waters.push(mapping.dest + (fert - mapping.source));
            }
        }
        if !is_match {
            waters.push(fert);
        }
    }

    // Map water to light
    println!("Map water to light....");
    let water_to_light_re = Regex::new(r"water-to-light map:\n([\d\s]+)")?;
    let water_to_light = water_to_light_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let water_to_light = water_to_light.chunks(3).collect::<Vec<_>>();
    let mut water_to_light_mappings: Vec<Mapping> = Vec::new();
    for line in water_to_light {
        water_to_light_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut lights = Vec::new();
    for water in waters {
        let mut is_match = false;
        for mapping in &water_to_light_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&water) {
                is_match = true;
                lights.push(mapping.dest + (water - mapping.source));
            }
        }
        if !is_match {
            lights.push(water);
        }
    }

    // Map light to temperature
    println!("Map light to temperature....");
    let light_to_temp_re = Regex::new(r"light-to-temperature map:\n([\d\s]+)")?;
    let light_to_temp = light_to_temp_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let light_to_temp = light_to_temp.chunks(3).collect::<Vec<_>>();
    let mut light_to_temp_mappings: Vec<Mapping> = Vec::new();
    for line in light_to_temp {
        light_to_temp_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut temps = Vec::new();
    for light in lights {
        let mut is_match = false;
        for mapping in &light_to_temp_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&light) {
                is_match = true;
                temps.push(mapping.dest + (light - mapping.source));
            }
        }
        if !is_match {
            temps.push(light);
        }
    }

    // Map temperature to humidity
    println!("Map temperature to humidity....");
    let temp_to_humidity_re = Regex::new(r"temperature-to-humidity map:\n([\d\s]+)")?;
    let temp_to_humidity = temp_to_humidity_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let temp_to_humidity = temp_to_humidity.chunks(3).collect::<Vec<_>>();
    let mut temp_to_humidity_mappings: Vec<Mapping> = Vec::new();
    for line in temp_to_humidity {
        temp_to_humidity_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut humidities = Vec::new();
    for temp in temps {
        let mut is_match = false;
        for mapping in &temp_to_humidity_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&temp) {
                is_match = true;
                humidities.push(mapping.dest + (temp - mapping.source));
            }
        }
        if !is_match {
            humidities.push(temp);
        }
    }

    // Map humidity to location
    println!("Map humidity to location....");
    let humidity_to_location_re = Regex::new(r"humidity-to-location map:\n([\d\s]+)")?;
    let humidity_to_location = humidity_to_location_re
        .captures(input)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .get(1)
        .ok_or(anyhow!("No seed to soil mapping found"))?
        .as_str()
        .trim()
        .split(&[' ', '\n'])
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();
    let humidity_to_location = humidity_to_location.chunks(3).collect::<Vec<_>>();
    let mut humidity_to_location_mappings: Vec<Mapping> = Vec::new();
    for line in humidity_to_location {
        humidity_to_location_mappings.push(Mapping {
            dest: line[0],
            source: line[1],
            length: line[2],
        });
    }
    let mut locations = Vec::new();
    for humidity in humidities {
        let mut is_match = false;
        for mapping in &humidity_to_location_mappings {
            let range = mapping.source..(mapping.source + mapping.length);
            if range.contains(&humidity) {
                is_match = true;
                locations.push(mapping.dest + (humidity - mapping.source));
            }
        }
        if !is_match {
            locations.push(humidity);
        }
    }
    // Return lowest location number corresponding to an initial seed
    println!("Getting min location....");
    Ok(*locations
        .iter()
        .min()
        .expect("There had better be a value here"))
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Problem 1: {}", problem1(input)?);
    println!("Problem 2: {}", problem2(input)?);
    Ok(())
}
