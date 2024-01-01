use std::{
    collections::{BTreeMap, BTreeSet},
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
                            start_pos = Some((x as isize, y as isize));
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
    frontier: BTreeSet<(isize, isize)>,
    steps_taken: usize,
    inf_grid: bool,
}

impl Garden {
    fn new(grid: Vec<Vec<char>>, starting_pos: (isize, isize)) -> Self {
        let mut frontier = BTreeSet::new();
        frontier.insert(starting_pos);
        Self {
            grid,
            frontier,
            steps_taken: 0,
            inf_grid: false,
        }
    }

    fn take_a_step(&mut self) {
        self.steps_taken += 1;
        self.frontier = self
            .frontier
            .iter()
            .map(|(x, y)| {
                let mut new_frontier = BTreeSet::new();

                // for part 2 we have an infinitely repeating grid
                // so we'll need to translate the points and check the original grid
                if self.inf_grid {
                    let max_y = self.grid.len() as isize;
                    let max_x = self.grid[0].len() as isize;
                    // N
                    let tx = x.rem_euclid(max_x) as usize;
                    let ty = (y - 1).rem_euclid(max_y) as usize;
                    if self.grid[ty][tx] == '.' {
                        new_frontier.insert((*x, y - 1));
                    }
                    // S
                    let ty = (y + 1).rem_euclid(max_y) as usize;
                    if self.grid[ty][tx] == '.' {
                        new_frontier.insert((*x, y + 1));
                    }
                    // E
                    let tx = (x + 1).rem_euclid(max_x) as usize;
                    let ty = y.rem_euclid(max_y) as usize;
                    if self.grid[ty][tx] == '.' {
                        new_frontier.insert(((x + 1), *y));
                    }
                    // W
                    let tx = (x - 1).rem_euclid(max_x) as usize;
                    if self.grid[ty][tx] == '.' {
                        new_frontier.insert((x - 1, *y));
                    }
                } else {
                    // N
                    if *y > 0 && self.grid[*y as usize - 1][*x as usize] == '.' {
                        new_frontier.insert((*x, y - 1));
                    }
                    // S
                    if *y < (self.grid.len() - 1) as isize
                        && self.grid[*y as usize + 1][*x as usize] == '.'
                    {
                        new_frontier.insert((*x, y + 1));
                    }
                    // E
                    if *x < (self.grid[0].len() - 1) as isize
                        && self.grid[*y as usize][*x as usize + 1] == '.'
                    {
                        new_frontier.insert(((x + 1), *y));
                    }
                    // W
                    if *x > 0 && self.grid[*y as usize][*x as usize - 1] == '.' {
                        new_frontier.insert((x - 1, *y));
                    }
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
                if self.frontier.contains(&(x as isize, y as isize)) {
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
pub fn part_one(input: &str) -> Option<u64> {
    Some(walk_it(input, 64, false).unwrap().0)
}

pub fn walk_it(input: &str, steps: usize, inf_grid: bool) -> Option<(u64, Vec<(usize, usize)>)> {
    let mut garden = parse(input);
    garden.inf_grid = inf_grid;
    let step_results = (0..steps)
        .map(|_| {
            garden.take_a_step();
            (garden.steps_taken, garden.frontier.len())
        })
        .collect_vec();
    // dbg!(&step_results);
    // let mut deltas: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    // step_results
    //     .iter()
    //     .tuple_windows::<(_, _)>()
    //     .for_each(|(a, b)| {
    //         let d = b.1 - a.1;
    //         deltas
    //             .entry(d)
    //             .and_modify(|it| it.push(b.0))
    //             .or_insert(vec![b.0]);
    //     });

    // dbg!(&deltas);
    Some((garden.frontier.len() as u64, step_results))
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = walk_it(input, 50, true).unwrap();
    // Does NOT work for large number of steps,
    //there's clearly a cycle/pattern we're supposed to be able to detect...
    Some(result.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = walk_it(
            &advent_of_code::template::read_file("examples", DAY),
            6,
            false,
        )
        .unwrap()
        .0;
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let result = walk_it(
            &advent_of_code::template::read_file("examples", DAY),
            500,
            true,
        )
        .unwrap()
        .0;
        assert_eq!(result, 6536);
    }
}
