advent_of_code::solution!(3);
use std::{collections::BTreeMap, ops::Deref};

#[derive(Debug)]
struct Grid(Vec<Vec<char>>);
impl Grid {
    fn is_symbol(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;

        if x > self[0].len() - 1 || y > self.len() - 1 {
            false
        } else {
            let c = self[y][x];
            c != '.' && !c.is_numeric()
        }
    }

    fn is_gear_symbol(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;

        if x > self[0].len() - 1 || y > self.len() - 1 {
            None
        } else {
            // println!("grid[{}][{}] = '{}'", x, y, &c);
            if self[y][x] == '*' {
                Some((x, y))
            } else {
                None
            }
        }
    }
}
impl Deref for Grid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut parts = vec![];
    grid.iter().enumerate().for_each(|(y, grid_line)| {
        let mut adj = false;
        let mut part_num = vec![];
        grid_line.iter().enumerate().for_each(|(x, c)| {
            if !c.is_numeric() || x == grid_line.len() - 1 {
                if x == grid_line.len() - 1 && c.is_numeric() {
                    // AAAAAAARGH edge conditions, literally.
                    part_num.push(*c);
                }

                if !part_num.is_empty() && adj {
                    parts.push(part_num.iter().collect::<String>().parse::<u32>().unwrap());
                }
                part_num.clear();
                adj = false;
            } else if c.is_numeric() {
                part_num.push(*c);
                let x = x as isize;
                let y = y as isize;
                if !adj {
                    adj = grid.is_symbol(x - 1, y)
                        || grid.is_symbol(x + 1, y)
                        || grid.is_symbol(x, y - 1)
                        || grid.is_symbol(x, y + 1)
                        || grid.is_symbol(x + 1, y + 1)
                        || grid.is_symbol(x + 1, y - 1)
                        || grid.is_symbol(x - 1, y - 1)
                        || grid.is_symbol(x - 1, y + 1)
                }
            }
        })
    });
    Some(parts.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut parts: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new(); // keep track of the gear coord, and which parts are next to it
    grid.iter().enumerate().for_each(|(y, grid_line)| {
        let mut attached_gear: Option<(usize, usize)> = None;
        let mut part_num = vec![];
        grid_line.iter().enumerate().for_each(|(x, c)| {
            if !c.is_numeric() || x == grid_line.len() - 1 {
                if x == grid_line.len() - 1 && c.is_numeric() {
                    part_num.push(*c);
                }

                if !part_num.is_empty() && attached_gear.is_some() {
                    let (x, y) = attached_gear.unwrap();
                    let part = part_num.iter().collect::<String>().parse::<u32>().unwrap();
                    parts.entry((x, y)).or_insert_with(Vec::new).push(part);
                }
                part_num.clear();
                attached_gear = None;
            } else if c.is_numeric() {
                part_num.push(*c);
                let x = x as isize;
                let y = y as isize;
                if attached_gear.is_none() {
                    let checks: Vec<_> = vec![
                        grid.is_gear_symbol(x - 1, y),
                        grid.is_gear_symbol(x + 1, y),
                        grid.is_gear_symbol(x, y - 1),
                        grid.is_gear_symbol(x, y + 1),
                        grid.is_gear_symbol(x + 1, y + 1),
                        grid.is_gear_symbol(x + 1, y - 1),
                        grid.is_gear_symbol(x - 1, y - 1),
                        grid.is_gear_symbol(x - 1, y + 1),
                    ]
                    .iter()
                    .filter_map(|v| *v)
                    .collect();
                    if !checks.is_empty() {
                        attached_gear = checks.first().copied();
                        // dbg!(&attached_gear, &checks);
                    }
                }
            }
        })
    });
    // dbg!(&parts);
    Some(
        parts
            .iter()
            .filter_map(|(_gear_location, part_numbers)| {
                if part_numbers.len() == 2 {
                    // println!("Found two parts around {:?}, {} and {}", _gear_location, part_numbers[0], part_numbers[1]);
                    Some(part_numbers[0] * part_numbers[1])
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn parse_grid(input: &str) -> Grid {
    Grid(input.lines().map(|line| line.chars().collect()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
