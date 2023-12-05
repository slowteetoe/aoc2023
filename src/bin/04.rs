use std::collections::{BTreeMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let pile = parse_cards(input);

    Some(
        pile.iter()
            .map(|(_id, winning, card)| {
                let wins = winning
                    .iter()
                    .collect::<HashSet<_>>()
                    .intersection(&card.iter().collect::<HashSet<_>>())
                    .collect::<Vec<_>>()
                    .len();
                if wins == 0 {
                    0
                } else {
                    let wins = wins as u32;
                    2u32.pow(wins - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let pile = parse_cards(input);
    let mut copies: BTreeMap<usize, u32> = BTreeMap::new();
    (1..=pile.len()).for_each(|c| {
        copies.insert(c, 1);
    });
    pile.iter().for_each(|(id, winning, card)| {
        let wins = winning
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&card.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .len();

        if wins > 0 {
            let to_be_added = match &copies.get(id) {
                Some(n) => **n,
                None => 1u32,
            };
            (1..=wins).for_each(|c| {
                copies
                    .entry(id + c)
                    .and_modify(|val| *val += to_be_added)
                    .or_insert(1);
            });
        }
    });

    Some(copies.values().sum())
}

fn parse_cards(input: &str) -> Vec<(usize, Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            let mut hands = line.split(":").nth(1).unwrap().split("|");

            // 41 48 83 86 17
            // 83 86  6 31 17  9 48 53
            let winners = hands
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|digits| digits.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let card = hands
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|digits| digits.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            (id + 1, winners, card)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
