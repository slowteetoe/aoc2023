use std::{collections::BTreeMap, str::FromStr};

use anyhow::anyhow;
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
    Joker,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Card as C;
        match s {
            "A" => Ok(C::Ace),
            "K" => Ok(C::King),
            "Q" => Ok(C::Queen),
            "J" => Ok(C::Jack),
            "T" => Ok(C::Ten),
            "9" => Ok(C::Nine),
            "8" => Ok(C::Eight),
            "7" => Ok(C::Seven),
            "6" => Ok(C::Six),
            "5" => Ok(C::Five),
            "4" => Ok(C::Four),
            "3" => Ok(C::Three),
            "2" => Ok(C::Two),
            _ => Err(anyhow!("could not map {} to a Card", s)),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    rank: Rank,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            let mut n = 0;
            loop {
                if self.cards[n] != other.cards[n] {
                    return self.cards[n].partial_cmp(&other.cards[n]);
                }
                n += 1;
            }
        } else {
            self.rank.partial_cmp(&other.rank)
        }
    }
}

#[derive(PartialEq, Clone)]
enum Modifier {
    WithJokers,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32, modifier: &Option<Modifier>) -> Self {
        let card_map = cards.iter().fold(BTreeMap::new(), |mut acc, c| {
            acc.entry(*c).and_modify(|e| *e += 1usize).or_insert(1);
            acc
        });
        let mut tups = card_map.iter().map(|(k, v)| (*k, *v)).collect_vec();
        if modifier
            .as_ref()
            .is_some_and(|it| *it == Modifier::WithJokers)
        {
            // as long as there aren't 5 jokers (really, FIVE jokers?), find the highest card(s) and add the joker(s) to that count
            if let Some((_, jokers_count)) = tups.iter().find(|(card, _)| *card == Card::Joker) {
                if *jokers_count != 5 {
                    let mut tmp = tups
                        .iter()
                        .filter(|(card, _)| *card != Card::Joker)
                        .map(|c| *c)
                        .collect_vec();
                    tmp.sort_by(|a, b| {
                        // want the highest card(s) first so we can just access [0]
                        if b.1 == a.1 {
                            b.0.cmp(&a.0)
                        } else {
                            b.1.cmp(&a.1)
                        }
                    });
                    tmp[0].1 += jokers_count;
                    tups = tmp;
                }
            }
        }
        let rank = match tups.len() {
            5 => Rank::HighCard,
            4 => Rank::OnePair,
            3 => {
                let three = tups.iter().find(|(_card, count)| *count == 3);
                if three.is_some() {
                    Rank::ThreeOfKind
                } else {
                    Rank::TwoPair
                }
            }
            2 => {
                let four = tups.iter().find(|(_card, count)| *count == 4);
                if four.is_some() {
                    Rank::FourOfKind
                } else {
                    Rank::FullHouse
                }
            }
            1 => Rank::FiveOfKind,
            _ => unreachable!("hand did not map to a known rank"),
        };
        Self { cards, bid, rank }
    }
}

fn parse_hands(input: &str, modifier: Option<Modifier>) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let cards = hand
                .chars()
                .map(|c| Card::from_str(c.to_string().as_str()).unwrap())
                .map(|c| {
                    if modifier
                        .as_ref()
                        .is_some_and(|it| *it == Modifier::WithJokers)
                        && c == Card::Jack
                    {
                        Card::Joker // 'J' are now Jokers
                    } else {
                        c
                    }
                })
                .collect_vec();
            Hand::new(cards, bid, &modifier.clone())
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands = parse_hands(input, None);
    hands.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    hands.reverse();
    let answer = hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        acc + ((rank + 1) as u64 * hand.bid as u64)
    });
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut hands = parse_hands(input, Some(Modifier::WithJokers));
    hands.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    hands.reverse();
    let answer = hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        acc + ((rank + 1) as u64 * hand.bid as u64)
    });
    Some(answer)
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
        assert_eq!(result, Some(5905));
    }
}
