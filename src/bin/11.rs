use std::collections::BTreeSet;

use itertools::Itertools;

advent_of_code::solution!(11);

fn manhattan(a: &(u32, u32), b: &(u32, u32)) -> i64 {
    i64::abs(b.0 as i64 - a.0 as i64) + i64::abs(b.1 as i64 - a.1 as i64)
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, ch)| {
                    if ch == '#' {
                        Some((x as u32, y as u32))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .flat_map(|v| {
            v.iter()
                .filter(|p| p.is_some())
                .map(|pt| pt.unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn expand(points: &mut Vec<(u32, u32)>, max_x: u32, max_y: u32, delta: u32) {
    let rows = points.iter().map(|pt| pt.1).collect::<BTreeSet<u32>>();
    let cols = points.iter().map(|pt| pt.0).collect::<BTreeSet<u32>>();

    let expand_rows = (0..=max_y).filter(|row| !rows.contains(row)).collect_vec();
    let expand_cols = (0..=max_x).filter(|col| !cols.contains(col)).collect_vec();

    expand_rows.iter().enumerate().for_each(|(n, row_num)| {
        points.iter_mut().for_each(|pt| {
            if pt.1 >= (n as u32 * delta) + row_num {
                pt.1 += delta;
            }
        });
    });

    expand_cols.iter().enumerate().for_each(|(n, col_num)| {
        points.iter_mut().for_each(|pt| {
            if pt.0 >= (n as u32 * delta) + col_num {
                pt.0 += delta;
            }
        });
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_y = input.lines().count() as u32;
    let max_x = input.lines().nth(0).unwrap().chars().count() as u32;

    let mut points = parse(input);
    expand(&mut points, max_x, max_y, 1);

    let unique_points = points.iter().combinations(2).collect_vec();

    let result = unique_points
        .iter()
        .map(|pt| manhattan(pt[0], pt[1]) as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_y = input.lines().count() as u32;
    let max_x = input.lines().nth(0).unwrap().chars().count() as u32;
    let mut points = parse(input);
    expand(&mut points, max_x, max_y, 1000000 - 1); // sigh, off by one
    let unique_points = points.iter().combinations(2).collect_vec();

    let result: u64 = unique_points
        .iter()
        .map(|pt| manhattan(pt[0], pt[1]) as u64)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
    }
}
