use std::collections::BTreeMap;

use itertools::Itertools;

advent_of_code::solution!(14);

/// (num_cols, num_rows, dish)
fn parse(input: &str) -> (usize, usize, Vec<char>) {
    (
        input.find("\n").unwrap(),
        input.lines().count(),
        input.chars().filter(|c| !c.is_whitespace()).collect_vec(),
    )
}

fn score(dish: &Vec<char>, num_rows: usize) -> u32 {
    let score = dish.iter().enumerate().fold(0, |mut acc, (idx, ch)| {
        if *ch == 'O' {
            acc += num_rows - (idx / num_rows)
        }
        acc
    });
    score as u32
}

// only going to work for a square matrix, which we have, yay.
pub fn rot_90(dish: &Vec<char>, width: usize) -> Vec<char> {
    let mut rot = vec![' '; dish.len()];
    dish.iter().enumerate().for_each(|(n, c)| {
        let level = width - 1 - (n / width);
        let col = n % width;
        let target = (col * width) + level;
        rot[target] = *c;
    });
    // println!("{}", &dish.iter().join(""));
    rot.to_owned()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (num_cols, num_rows, mut dish) = parse(input);
    let dish = tilt_north(&mut dish, num_cols);
    Some(score(&dish, num_rows))
}

pub fn tilt_north(dish: &mut Vec<char>, num_cols: usize) -> Vec<char> {
    (num_cols..dish.len()).for_each(|n| {
        // try move each dish[n] left
        let mut this_n = n;
        if dish[n] == 'O' {
            loop {
                if this_n < num_cols {
                    break;
                }
                let next_n = this_n - num_cols;
                if dish[next_n] == '.' {
                    dish[next_n] = 'O';
                    dish[this_n] = '.';
                } else {
                    break;
                }
                if this_n < num_cols {
                    break;
                }
                this_n = this_n - num_cols;
            }
        }
    });
    dish.to_owned()
}

// we'll have to detect a cycle, there's no way we need to run this 1_000_000_000 times
pub fn part_two(input: &str) -> Option<u32> {
    let (num_cols, num_rows, dish) = parse(input);

    let mut dish = dish.to_owned();
    let mut times = 1;
    let mut cycle = 0;
    let mut m = BTreeMap::new();
    let mut answer = None;
    loop {
        let mut tilted = &tilt_north(&mut dish, num_cols);
        let rot = rot_90(&mut tilted, num_cols);
        if times % 4 == 0 {
            let s = rot.iter().join("");
            cycle += 1;
            // println!("cycle {cycle}: {s}");
            if m.contains_key(&s) {
                let last_seen = m.get(&s).unwrap();
                let repeating_cycle = cycle - last_seen;
                // println!("Cycle of cycles detected in {cycle}, prev was {last_seen}! So, every {repeating_cycle} cycles");
                // then we need to grab the nth cycle since the billionth didn't fall neatly
                let offset = (1_000_000_000 - *last_seen as usize) % repeating_cycle as usize;
                // println!("We need the output from {offset}");
                // should be quick iteration since we stopped after the first cycle was detected, but must be a better way..?
                answer = m
                    .iter()
                    .filter_map(|(k, v)| {
                        if *v == offset + last_seen {
                            Some(k)
                        } else {
                            None
                        }
                    })
                    .at_most_one()
                    .unwrap();
                // dbg!(&answer);
                break;
            } else {
                m.insert(s, cycle);
            }
        }
        times = times + 1;
        dish = rot;
    }
    let dish = answer.unwrap().chars().collect_vec();
    Some(score(&dish, num_rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_rotation() {
        let result = rot_90(
            &vec![
                '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            ],
            4,
        );
        assert_eq!(
            result,
            vec!['d', '9', '5', '1', 'e', 'a', '6', '2', 'f', 'b', '7', '3', 'g', 'c', '8', '4']
        )
    }
}
