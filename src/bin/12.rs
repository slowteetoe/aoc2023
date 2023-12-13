use std::vec;

use itertools::Itertools;

advent_of_code::solution!(12);

// Might be able to just brute force this, at least for part 1
// since there are only two possiblities for each position with a question mark...
pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);
    let mut answer = vec![];
    data.iter()
        // .take(1)
        .for_each(|(positions, validation)| answer.push(resolve_one(&positions)));
    dbg!(answer);
    None
}

// fixme this is bad
pub fn is_valid(line: &str, rules: Vec<u32>) -> bool {
    todo!();
    let c = line.chars().collect_vec();
    let mut valid = true;
    rules.iter().for_each(|n| {
        let mut obs = 0;
        let mut pos = 0;
        loop {
            if pos >= c.len() {
                valid = false;
                break;
            } else if c[pos] == '.' {
                // eat whitespace (.)
                pos += 1;
            } else {
            }
        }
    });
}

// each time, replace one placeholder with its possiblities
pub fn resolve_one(input: &str) -> Vec<String> {
    // println!("called with {:?}", input);
    if input.chars().filter(|c| *c == '?').count() == 1 {
        let period = input.replace("?", ".");
        let hash = input.replace("?", "#");
        // dbg!(&period, &hash);
        return vec![period.to_owned(), hash.to_owned()];
    }
    let mut answer: Vec<String> = vec![];
    resolve_one(input.replacen("?", ".", 1).as_str())
        .iter()
        .for_each(|path| answer.push(path.clone()));

    resolve_one(input.replacen("?", "#", 1).as_str())
        .iter()
        .for_each(|path| answer.push(path.clone()));
    answer
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<(&str, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            (
                s.next().unwrap(),
                s.next()
                    .unwrap()
                    .chars()
                    .filter_map(|c| if c.is_numeric() { c.to_digit(10) } else { None })
                    .collect(),
            )
        })
        // .map(|t| {
        //     let v = t.0.collect();
        //     (vec![], vec![])
        // })
        .collect_vec()
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
