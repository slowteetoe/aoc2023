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
    FiveOfKind(Card),
    FourOfKind(Card, Card),
    FullHouse(Card, Card),
    ThreeOfKind(Card, Card, Card),
    TwoPair(Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
    Unmapped,
}

#[derive(Debug)]
struct Hand {
    cards: BTreeMap<Card, usize>,
    bid: u32,
    rank: Rank,
}

impl Hand {
    fn new(cards: BTreeMap<Card, usize>, bid: u32) -> Self {
        let tups = cards.iter().map(|(k, v)| (*k, *v)).collect_vec();
        let rank = match tups.len() {
            5 => {
                let mut cards = tups.iter().map(|(c, _)| *c).collect_vec();
                cards.sort();
                Rank::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4])
            }
            4 => {
                let pair = tups.iter().find(|(_card, count)| *count == 2).unwrap().0;
                let mut rest = tups
                    .iter()
                    .filter(|(_card, count)| *count != 2)
                    .map(|(card, _)| *card)
                    .collect_vec();
                rest.sort();
                // one pair
                Rank::OnePair(pair, rest[0], rest[1], rest[2])
            }
            3 => {
                // Three of a kind
                let three = tups.iter().find(|(_card, count)| *count == 3);
                if three.is_some() {
                    let mut rest = tups
                        .iter()
                        .filter(|(_card, count)| *count != 3)
                        .map(|(card, _)| *card)
                        .collect_vec();
                    rest.sort();
                    Rank::ThreeOfKind(three.unwrap().0, rest[0], rest[1])
                } else {
                    // Two pair
                    let mut rest = tups
                        .iter()
                        .filter(|(_card, count)| *count == 2)
                        .map(|(card, _)| *card)
                        .collect_vec();
                    rest.sort();
                    Rank::TwoPair(
                        rest[0],
                        rest[1],
                        tups.iter().find(|(_card, count)| *count == 1).unwrap().0,
                    )
                }
            }
            2 => {
                // Four of a kind
                let four = tups.iter().find(|(_card, count)| *count == 4);
                if four.is_some() {
                    Rank::FourOfKind(
                        four.unwrap().0,
                        tups.iter().find(|(_card, count)| *count == 1).unwrap().0,
                    )
                } else {
                    // Full House
                    Rank::FullHouse(
                        tups.iter().find(|(_card, count)| *count == 3).unwrap().0,
                        tups.iter().find(|(_card, count)| *count == 2).unwrap().0,
                    )
                }
            }
            1 => Rank::FiveOfKind(tups.iter().nth(0).unwrap().0),
            _ => Rank::Unmapped,
        };
        Self { cards, bid, rank }
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let cards = hand
                .chars()
                .map(|c| Card::from_str(c.to_string().as_str()).unwrap())
                .fold(BTreeMap::new(), |mut acc, c| {
                    acc.entry(c).and_modify(|e| *e += 1usize).or_insert(1);
                    acc
                });
            Hand::new(cards, bid)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands = parse_hands(input);
    hands.sort_by(|a, b| a.rank.cmp(&b.rank));
    hands.reverse();
    // dbg!(&hands);
    let answer = hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        println!(
            "{:?} wins {:?}x{} = {}",
            hand.rank,
            rank + 1,
            hand.bid,
            ((rank + 1) as u64 * hand.bid as u64)
        );
        acc + ((rank + 1) as u64 * hand.bid as u64)
    });
    // 255940725
    Some(answer)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
