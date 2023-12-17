use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Puzzle {
    rows: Vec<String>,
    cols: Vec<String>,
}

fn parse(input: &str) -> Vec<Puzzle> {
    let input = input.split("\n\n");
    input
        .map(|p| {
            let raw = p
                .lines()
                .map(|line| line.chars().map(|c| c).collect_vec())
                .collect_vec();

            let mut rows = vec!["".to_owned(); raw.len()];
            let mut cols = vec!["".to_owned(); raw[0].len()];
            (0..raw.len()).for_each(|y| {
                (0..raw[0].len()).for_each(|x| {
                    let c = raw[y][x];
                    rows[y].push(c);
                    cols[x].push(c);
                });
            });
            Puzzle { rows, cols }
        })
        .collect()
}

// direction only needed to be able to score the pattern
fn check_for_reflection(map: &Vec<String>, left: &usize, right: &usize) -> Option<usize> {
    let lines_to_check = std::cmp::min(map.len() - 1 - right, *left);
    let mut valid = 0;
    (1..=lines_to_check).for_each(|delta| {
        let right = &map[right + delta];
        let left = &map[left - delta];
        if left == right {
            valid += 1;
        }
    });
    if valid == lines_to_check {
        Some(*left)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzles = parse(input);
    let mut score = 0u32;
    puzzles.iter().for_each(|puzzle| {
        // dbg!(&score);
        // rows
        let possible_mids = puzzle
            .rows
            .windows(2)
            .enumerate()
            .filter_map(|(n, strings)| {
                if strings[0] == strings[1] {
                    Some((n, n + 1))
                } else {
                    None
                }
            })
            .collect_vec();

        let actual = possible_mids
            .iter()
            .filter_map(|(left, right)| check_for_reflection(&puzzle.rows, left, right))
            .collect_vec();

        if !actual.is_empty() {
            let this_score = (actual[0] as u32 + 1) * 100;
            // println!(
            //     "VALID ROW REFLECTION AT {:?} for {:?} points ",
            //     actual, this_score
            // );
            score += this_score;
            return;
        }
        // cols
        let possible_mids = puzzle
            .cols
            .windows(2)
            .enumerate()
            .filter_map(|(n, strings)| {
                if strings[0] == strings[1] {
                    Some((n, n + 1))
                } else {
                    None
                }
            })
            .collect_vec();

        let actual = possible_mids
            .iter()
            .filter_map(|(left, right)| check_for_reflection(&puzzle.cols, left, right))
            .collect_vec();

        if !actual.is_empty() {
            let this_score = actual[0] as u32 + 1;
            // println!(
            //     "VALID COL REFLECTION AT {:?} for {:?} points",
            //     actual, this_score
            // );
            score += this_score;
            return;
        } else {
            panic!("didn't find ANY reflections");
        }
    });
    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
