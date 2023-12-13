use std::vec;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(12);

// Might be able to just brute force this, at least for part 1
// since there are only two possiblities for each position with a question mark...
pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);
    let a: u32 = data
        .iter()
        .take(2)
        .map(|(positions, validation)| (resolve_one(&positions), build_regex(validation)))
        .map(|(inputs, validation_regex)| {
            let re = Regex::new(&validation_regex).unwrap();
            inputs
                .iter()
                .unique()
                .filter(|i| {
                    if re.is_match(*i) {
                        dbg!(&re, &i);
                        true
                    } else {
                        false
                    }
                })
                .count() as u32
        })
        .sum();
    Some(a)
}

fn build_regex(rule: &Vec<u32>) -> String {
    let mut re = r"^\.*".to_owned();
    rule.iter().enumerate().for_each(|(pos, val)| {
        re.push_str(r"#{");
        re.push_str(&val.to_string());
        re.push_str(r"}");
        if pos == rule.len() - 1 {
            re.push_str(r"\.*$");
        } else {
            re.push_str(r"\.+");
        }
    });
    re
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
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_regex_creation() {
        let result = build_regex(&vec![1, 1, 3]);
        assert_eq!(result, r"^\.*#{1}\.+#{1}\.+#{3}\.*$");
    }
}
