use std::collections::BTreeMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = parse(input);
    let mut paths = BTreeMap::new();
    almanac.seeds.iter().for_each(|seed| {
        paths.insert(seed, vec![]);
    });
    almanac.seeds.iter().for_each(|seed| {
        let mut found = false;
        almanac.seeds_to_soil.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&seed)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = seed - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(*seed));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.soil_to_fertilizer.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.fertilizer_to_water.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.water_to_light.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.light_to_temperature.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.temperature_to_humidity.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    almanac.seeds.iter().for_each(|seed| {
        let next_val = *paths.get(seed).unwrap().iter().last().unwrap();
        let mut found = false;
        almanac.humidity_to_location.iter().for_each(|rule| {
            if (rule.source_range_start..rule.source_range_start + rule.range_length)
                .contains(&next_val)
            {
                // figure out index in source_range, then translate over to dest_range_start
                let dx = next_val - rule.source_range_start;
                let dest_value = rule.dest_range_start + dx;
                paths.entry(seed).and_modify(|e| e.push(dest_value));
                found = true;
            }
        });
        if !found {
            paths.entry(seed).and_modify(|e| e.push(next_val));
        }
    });
    Some(
        *paths
            .iter()
            .map(|(_seed, path)| path.iter().last().unwrap())
            .min()
            .unwrap() as u32,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MappingRule {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug, Default)]
pub struct Almanac {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<MappingRule>,
    soil_to_fertilizer: Vec<MappingRule>,
    fertilizer_to_water: Vec<MappingRule>,
    water_to_light: Vec<MappingRule>,
    light_to_temperature: Vec<MappingRule>,
    temperature_to_humidity: Vec<MappingRule>,
    humidity_to_location: Vec<MappingRule>,
}

pub fn parse(input: &str) -> Almanac {
    let mut alm = Almanac::default();
    let mut dest = "";
    input.lines().for_each(|line| {
        if line.starts_with("seeds: ") {
            alm.seeds = line
                // seeds: 79 14 55 13
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.trim().parse::<u64>().expect("should have been a seed id"))
                .collect();
        } else if line.ends_with(" map:") {
            dest = line
                .split(" map:")
                .nth(0)
                .expect("should have found the mapping name");
        } else if line == "" {
        } else {
            let p: Vec<u64> = line
                .split_whitespace()
                .map(|c| c.trim().parse::<u64>().unwrap())
                .collect();
            let m = MappingRule {
                dest_range_start: p[0],
                source_range_start: p[1],
                range_length: p[2],
            };
            match dest {
                "seed-to-soil" => alm.seeds_to_soil.push(m),
                "soil-to-fertilizer" => alm.soil_to_fertilizer.push(m),
                "fertilizer-to-water" => alm.fertilizer_to_water.push(m),
                "water-to-light" => alm.water_to_light.push(m),
                "light-to-temperature" => alm.light_to_temperature.push(m),
                "temperature-to-humidity" => alm.temperature_to_humidity.push(m),
                "humidity-to-location" => alm.humidity_to_location.push(m),
                _ => {}
            }
        }
    });
    alm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
