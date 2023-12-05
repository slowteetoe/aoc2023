advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = parse(input);
    dbg!(&almanac);
    None
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
    let mut dest = &mut alm.seeds_to_soil;
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
        } else if line.starts_with("seed-to-soil") {
            // dest = &mut alm.seeds_to_soil;
        } else if line.starts_with("soil-to-fertilizer") {
            dest = &mut alm.soil_to_fertilizer;
        } else if line.starts_with("fertilizer-to-water") {
            dest = &mut &alm.fertilizer_to_water;
        } else if line.starts_with("water-to-light") {
            dest = &mut alm.water_to_light;
        } else if line.starts_with("light-to-temperature") {
            dest = &mut alm.light_to_temperature;
        } else if line.starts_with("temperature-to-humidity") {
            dest = &mut alm.temperature_to_humidity;
        } else if line.starts_with("humidity-to-location") {
            dest = &mut alm.humidity_to_location;
        } else {
            // do nothing for now
            let p: Vec<u64> = line
                .split_whitespace()
                .map(|c| c.trim().parse::<u64>().unwrap())
                .collect();
            dest.push(MappingRule {
                dest_range_start: p[0],
                source_range_start: p[1],
                range_length: p[2],
            })
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
