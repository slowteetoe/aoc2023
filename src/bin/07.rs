use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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
                    .collect_vec(),
                // .tuples::<(_, _, _, _, _)>()
                // .collect_vec()
                // .first()
                // .unwrap()
                // .to_owned(),
                parts.nth(0).unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    // dbg!(&rounds);
    // now have to figure out how to score the hand
    let counts: Vec<_> = rounds
        .iter()
        .map(|(round, bet)| (round.into_iter().counts_by(|c| **c), bet))
        .map(|(hand, bet)| match hand.len() {
            1 => {
                if let [c1] = *hand.keys().collect_vec() {
                    Ranks::FiveOfKind(*c1)
                } else {
                    Ranks::Nothing
                }
            }
            2 => {
                if let [c1, c2] = *hand
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(b.1, a.1))
                    .collect_vec()
                {
                    if c1.1 == &3 {
                        // 3, 2
                        Ranks::FullHouse(*c1.0, *c2.0)
                    } else {
                        // 4, 1
                        Ranks::FourOfKind(*c1.0, *c2.0)
                    }
                } else {
                    Ranks::Nothing
                }
            }
            3 => {
                if let [c1, c2, c3] = *hand
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(b.1, a.1))
                    .collect_vec()
                {
                    if c1.1 == &2 {
                        // 2, 2, 1 - two pair
                        Ranks::TwoPair(*c1.0, *c2.0, *c3.0)
                    } else {
                        // 3, 1, 1 - three of a kind
                        Ranks::ThreeOfKind(*c1.0, *c2.0, *c3.0)
                    }
                } else {
                    Ranks::Nothing
                }
            }
            4 => {
                if let [c1, c2, c3, c4] = *hand
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(b.1, a.1))
                    .collect_vec()
                {
                    Ranks::OnePair(*c1.0, *c2.0, *c3.0, *c4.0)
                } else {
                    Ranks::Nothing
                }
            }
            5 => {
                if let [c1, c2, c3, c4, c5] = *hand
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(b.1, a.1))
                    .collect_vec()
                {
                    Ranks::HighCard(*c1.0, *c2.0, *c3.0, *c4.0, *c5.0)
                } else {
                    Ranks::Nothing
                }
            }
            _ => Ranks::Nothing,
        })
        .collect();
    dbg!(counts);
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
