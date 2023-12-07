use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
pub struct TimeDistance {
    time: usize,
    distance: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let t = parse(input);
    // dbg!(&t);
    let winning_ways = t
        .iter()
        .map(|race| {
            // some sort of binary search to find lower bound and upper bound for where we win
            find_latest_win(&race.time, &race.distance).unwrap() + 1
                - find_earliest_win(&race.time, &race.distance).unwrap()
        })
        .fold(1, |acc, n| acc * n);
    Some(winning_ways)
}

pub fn part_two(input: &str) -> Option<u64> {
    let temp = parse(input);

    let merged = temp.iter().fold((0, 0), |mut acc, td| {
        acc.0 = format!("{}{}", acc.0, td.time).parse().unwrap();
        acc.1 = format!("{}{}", acc.1, td.distance).parse().unwrap();
        acc
    });

    // dbg!(&merged);

    let winning_ways = vec![TimeDistance {
        time: merged.0,
        distance: merged.1,
    }]
    .iter()
    .map(|race| {
        find_latest_win(&race.time, &race.distance).unwrap() + 1
            - find_earliest_win(&race.time, &race.distance).unwrap()
    })
    .fold(1, |acc, n| acc * n);
    Some(winning_ways)
}

fn find_earliest_win(time: &usize, distance: &usize) -> Option<u64> {
    let mut last_win = None;
    let mut low = 1;
    let mut high = *time;

    while low <= high {
        let mid = (high - low) / 2 + low;
        let actual = play(&mid, time);
        if actual > *distance as u64 {
            last_win = Some(mid as u64);
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    last_win
}

fn find_latest_win(time: &usize, distance: &usize) -> Option<u64> {
    let mut last_win = None;
    let mut low = 1;
    let mut high = *time;

    while low <= high {
        let mid = (high - low) / 2 + low;
        let actual = play(&mid, time);
        if actual > *distance as u64 {
            last_win = Some(mid as u64);
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    last_win
}

pub fn play(duration: &usize, total_time: &usize) -> u64 {
    let velocity = total_time - duration;
    let distance = velocity * (total_time - velocity);
    distance as u64
}

pub fn parse(input: &str) -> Vec<TimeDistance> {
    let mut result = vec![];
    input.lines().tuples().for_each(|(line1, line2)| {
        line1
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .for_each(|n| {
                result.push(TimeDistance {
                    time: n.parse::<usize>().unwrap(),
                    distance: 0,
                });
            });
        line2
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .enumerate()
            .for_each(|(idx, n)| {
                result[idx].distance = n.parse::<usize>().unwrap();
            });
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
