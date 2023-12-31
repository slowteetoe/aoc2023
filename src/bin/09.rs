use std::collections::BTreeSet;

use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn compute(sofar: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer = vec![sofar.clone()];
    let mut next = next_line(&sofar);
    loop {
        let unique = next.iter().collect::<BTreeSet<_>>();
        answer.push(next.clone());
        if unique.len() == 1 && unique.get(&0).is_some() {
            break;
        } else {
            next = next_line(&next);
        }
    }
    let depth = answer.len();
    answer.reverse();
    (0..depth).for_each(|n| {
        let val = if n == 0 {
            0
        } else {
            answer[n - 1][answer[n - 1].len() - 1] + answer[n][answer[n].len() - 1]
        };
        answer[n].push(val.clone());
    });
    answer
}

fn compute_front(sofar: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer = vec![sofar.clone()];
    let mut next = next_line(&sofar);
    loop {
        let unique = next.iter().collect::<BTreeSet<_>>();
        answer.push(next.clone());
        if unique.len() == 1 && unique.get(&0).is_some() {
            break;
        } else {
            next = next_line(&next);
        }
    }

    let depth = answer.len();
    answer.reverse();
    (0..depth).for_each(|n| {
        let val = if n == 0 {
            0
        } else {
            answer[n][0] - answer[n - 1][0]
        };
        answer[n].insert(0, val.clone());
    });
    // dbg!(&answer);
    answer
}

fn next_line(line: &Vec<i32>) -> Vec<i32> {
    line.iter()
        .tuple_windows::<(_, _)>()
        .map(|t| t.1 - *t.0 as i32)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i32> {
    let answers = parse(input)
        .iter()
        .map(|puzzle| {
            let mut answer = compute(&puzzle);
            answer.reverse();
            answer[0][answer[0].len() - 1]
        })
        .collect_vec();
    Some(answers.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let answers = parse(input)
        .iter()
        .map(|puzzle| {
            let mut answer = compute_front(&puzzle);
            answer.reverse();
            answer[0][0]
        })
        .collect_vec();
    Some(answers.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
