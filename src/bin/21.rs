use std::{
    collections::BTreeSet,
    fmt::{Display, Write},
};

use itertools::Itertools;

advent_of_code::solution!(21);

fn parse(input: &str) -> Garden {
    let mut start_pos = None;
    Garden::new(
        input
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, square)| {
                        if square == 'S' {
                            start_pos = Some((x, y));
                            '.'
                        } else {
                            square
                        }
                    })
                    .collect_vec()
            })
            .collect_vec(),
        start_pos.unwrap(),
    )
}

#[derive(Debug)]
struct Garden {
    grid: Vec<Vec<char>>,
    frontier: BTreeSet<(usize, usize)>,
    steps_taken: usize,
    inf_scrolling: bool,
}

impl Garden {
    fn new(grid: Vec<Vec<char>>, starting_pos: (usize, usize)) -> Self {
        let mut frontier = BTreeSet::new();
        frontier.insert(starting_pos);
        Self {
            grid,
            frontier,
            steps_taken: 0,
            inf_scrolling: false,
        }
    }

    fn take_a_step(&mut self) {
        self.steps_taken += 1;
        self.frontier = self
            .frontier
            .iter()
            .map(|(x, y)| {
                let mut new_frontier = BTreeSet::new();
                // N
                if *y > 0 && self.grid[y - 1][*x] == '.' {
                    new_frontier.insert((*x, y - 1));
                }
                // S
                if *y < self.grid.len() - 1 && self.grid[y + 1][*x] == '.' {
                    new_frontier.insert((*x, y + 1));
                }
                // E
                if *x < self.grid[0].len() - 1 && self.grid[*y][x + 1] == '.' {
                    new_frontier.insert(((x + 1), *y));
                }
                // W
                if *x > 0 && self.grid[*y][x - 1] == '.' {
                    new_frontier.insert((x - 1, *y));
                }
                new_frontier
            })
            .flatten()
            .collect();
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, square)| {
                if self.frontier.contains(&(x, y)) {
                    f.write_char('O').unwrap();
                } else {
                    f.write_char(*square).unwrap();
                }
            });
            f.write_str("\n").unwrap();
        });
        Ok(())
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    walk_it(input, 64)
}

pub fn walk_it(input: &str, steps: usize) -> Option<u32> {
    let mut garden = parse(input);
    (0..steps).for_each(|_| {
        garden.take_a_step();
        // println!(
        //     "steps taken: {} with squares occupied: {}",
        //     &garden.steps_taken,
        //     &garden.frontier.len()
        // );
    });
    Some(garden.frontier.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut garden = parse(input);
    garden.inf_scrolling = true;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = walk_it(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
