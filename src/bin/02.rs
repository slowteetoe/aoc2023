advent_of_code::solution!(2);

use regex::Regex;
use std::cmp;

#[derive(Default, Debug, Clone)]
pub struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn max(&mut self, other: &Cubes) {
        self.red = cmp::max(self.red, other.red);
        self.green = cmp::max(self.green, other.green);
        self.blue = cmp::max(self.blue, other.blue);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let summed_ids: u32 = extract_games(input)
        .iter()
        .filter_map(|(id, sets)| {
            if sets.iter().all(|cube| is_valid_set(cube)) {
                Some(*id as u32)
            } else {
                None
            }
        })
        .sum();
    Some(summed_ids)
}

fn is_valid_set(cubes: &Cubes) -> bool {
    cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14
}

pub fn part_two(input: &str) -> Option<u32> {
    let power: u32 = extract_games(input)
        .iter()
        .map(|(_id, sets)| {
            let mut acc = Cubes::default();
            let cube = sets.iter().fold(&mut acc, |acc, this_set| {
                acc.max(this_set);
                acc
            });
            cube.clone()
        })
        .map(|c| c.red * c.green * c.blue)
        .sum();
    Some(power)
}

fn extract_games(input: &str) -> Vec<(u8, Vec<Cubes>)> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let re = Regex::new(r"Game (\d+):.*").unwrap();
    input
        .lines()
        .map(|line| {
            let game_id = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap();
            let sets_line = line.split(":").skip(1).collect::<String>();
            //  3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let sets: Vec<_> = sets_line
                .trim()
                .split(";")
                .map(|part| {
                    let mut cubes = Cubes::default();
                    // 3 blue, 4 red
                    part.split(", ").for_each(|set| {
                        if set.contains(" green") {
                            cubes.green = set.replace(" green", "").trim().parse().unwrap();
                        } else if set.contains(" blue") {
                            cubes.blue = set.replace(" blue", "").trim().parse().unwrap();
                        } else if set.contains(" red") {
                            cubes.red = set.replace(" red", "").trim().parse().unwrap();
                        } else {
                            panic!("could not parse values from set, line is: {set}");
                        }
                    });
                    cubes
                })
                .collect();
            (game_id, sets)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
