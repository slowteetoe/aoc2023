advent_of_code::solution!(2);

use regex::Regex;

#[derive(Debug)]
pub struct Cubes {
    r: u8,
    g: u8,
    b: u8,
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Game (\d+):.*").unwrap();
    let games: u32 = input
        .lines()
        .map(|line| {
            let id = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .unwrap();
            let sets_line = line.split(":").skip(1).collect::<String>();
            let sets: Vec<_> = sets_line
                .trim()
                .split(";")
                .map(|part| {
                    let mut c = Cubes { r: 0, g: 0, b: 0 };
                    // 8 green, 6 blue, 20 red
                    part.split(", ").for_each(|rgb| {
                        if rgb.contains(" green") {
                            c.g = rgb.replace(" green", "").trim().parse().unwrap();
                        } else if rgb.contains(" blue") {
                            c.b = rgb.replace(" blue", "").trim().parse().unwrap();
                        } else if rgb.contains(" red") {
                            c.r = rgb.replace(" red", "").trim().parse().unwrap();
                        } else {
                            panic!("could not parse values from set, line is: {rgb}");
                        }
                    });
                    c
                })
                .collect();
            (id, sets)
        })
        // .inspect(|game| {
        //     dbg!(game);
        // })
        .filter_map(|(id, sets)| {
            if sets.iter().all(|cube| is_valid_set(cube)) {
                Some(id as u32)
            } else {
                None
            }
        })
        .sum();
    Some(games)
}

fn is_valid_set(cubes: &Cubes) -> bool {
    cubes.r <= 12 && cubes.g <= 13 && cubes.b <= 14
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
