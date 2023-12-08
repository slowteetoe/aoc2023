use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Ranks {
    FiveOfKind(Card),
    FourOfKind(Card, Card),
    FullHouse(Card, Card),
    ThreeOfKind(Card, Card, Card),
    TwoPair(Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
    Nothing,
}

pub fn part_one(input: &str) -> Option<u32> {
    use crate::Card::*;
    let rounds: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts
                    .nth(0)
                    .unwrap()
                    .chars()
                    .map(|c| match c {
                        'A' => &Ace,
                        'K' => &King,
                        'Q' => &Queen,
                        'J' => &Jack,
                        'T' => &Ten,
                        '9' => &Nine,
                        '8' => &Eight,
                        '7' => &Seven,
                        '6' => &Six,
                        '5' => &Five,
                        '4' => &Four,
                        '3' => &Three,
                        '2' => &Two,
                        '1' => &One,
                        _ => unreachable!(),
                    })
                    .sorted()
                    .tuples::<(_, _, _, _, _)>()
                    .collect_vec()
                    .first()
                    .unwrap()
                    .to_owned(),
                parts.nth(0).unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    dbg!(&rounds);
    // now have to figure out how to score the hand
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
