use itertools::Itertools;

advent_of_code::solution!(14);

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

pub fn part_one(input: &str) -> Option<u32> {
    let (num_cols, num_rows, mut dish) = parse(input);
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
    // println!("OOOO.#.O..OO..#....#OO..O##..OO..#.OO...........#...#....#.#..O..#.O.O..O.......#....###..#....#....");
    // println!("{}", dish.iter().join(""));

    Some(score(&dish, num_rows))
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
