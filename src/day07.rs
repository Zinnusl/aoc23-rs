#![allow(dead_code)]
#![allow(unused_imports)]

use contracts::*;

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

use rayon::prelude::*;

use indicatif::{ProgressBar, ProgressStyle};

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Parser)]
#[grammar = "day07.pest"]
struct TableParser;

#[derive(Debug, Eq)]
struct Hand {
    cards: (i64, i64, i64, i64, i64),
    orig_cards: Option<(i64, i64, i64, i64, i64)>,
}

type Bid = i64;

fn parse(card: char) -> Option<i64> {
    match card {
        'A' => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        'J' => Some(1),
        'T' => Some(10),
        '0'..='9' => Some(card.to_digit(10).unwrap() as i64),
        _ => None,
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn ch_eq<'a>(cards: &(i64, i64, i64, i64, i64), mask: impl IntoIterator<Item = i64>) -> bool {
    let cards = [cards.0, cards.1, cards.2, cards.3, cards.4];
    let cards = cards.iter();
    let mask = mask.into_iter();
    let mut ch = [0i64; 2];
    for (c, m) in cards.zip(mask) {
        if m != 0 {
            if ch[m as usize - 1] != 0 && ch[m as usize - 1] != *c {
                return false;
            }
            ch[m as usize - 1] = *c;
        }
    }

    true
}

impl Hand {
    fn is_5_of_a_kind(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 1, 1, 1])
    }
    fn is_4_of_a_kind(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 1, 1, 0]) || ch_eq(&cards, [0, 1, 1, 1, 1])
    }
    fn is_full_house(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 1, 2, 2]) || ch_eq(&cards, [2, 2, 1, 1, 1])
    }
    fn is_3_of_a_kind(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 1, 0, 0])
            || ch_eq(&cards, [0, 1, 1, 1, 0])
            || ch_eq(&cards, [0, 0, 1, 1, 1])
    }
    fn is_2_pair(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 2, 2, 0])
            || ch_eq(&cards, [1, 1, 0, 2, 2])
            || ch_eq(&cards, [0, 1, 1, 2, 2])
            || ch_eq(&cards, [2, 2, 1, 1, 0])
            || ch_eq(&cards, [2, 2, 0, 1, 1])
            || ch_eq(&cards, [0, 2, 2, 1, 1])
    }
    fn is_1_pair(cards: &(i64, i64, i64, i64, i64)) -> bool {
        ch_eq(&cards, [1, 1, 0, 0, 0])
            || ch_eq(&cards, [0, 1, 1, 0, 0])
            || ch_eq(&cards, [0, 0, 1, 1, 0])
            || ch_eq(&cards, [0, 0, 0, 1, 1])
    }
    fn hand_type(&self) -> i64 {
        match self.cards {
            // 5 of a kind
            cards if Hand::is_5_of_a_kind(&cards) => 6,
            // 4 of a kind
            cards if Hand::is_4_of_a_kind(&cards) => 5,
            // // Full house
            cards if Hand::is_full_house(&cards) => 4,
            // // Three of a kind
            cards if Hand::is_3_of_a_kind(&cards) => 3,
            // // Two pair
            cards if Hand::is_2_pair(&cards) => 2,
            // // One pair
            cards if Hand::is_1_pair(&cards) => 1,
            // High card
            _ => 0,
        }
    }

    fn sort(&mut self) {
        if self.orig_cards.is_none() {
            self.orig_cards = Some(self.cards);
        }
        let mut cards = [
            self.cards.0,
            self.cards.1,
            self.cards.2,
            self.cards.3,
            self.cards.4,
        ];
        cards.sort();
        self.cards = (cards[0], cards[1], cards[2], cards[3], cards[4]);
    }
}

fn partial_cmp(
    a: impl IntoIterator<Item = i64>,
    b: impl IntoIterator<Item = i64>,
) -> Option<std::cmp::Ordering> {
    let a = a.into_iter();
    let b = b.into_iter();

    a.zip(b)
        .map(|(a, b): (i64, i64)| a.partial_cmp(&b))
        .filter(|x| {
            println!("{:?}", x);
            x.is_some()
        })
        .find(|x| x.unwrap() != std::cmp::Ordering::Equal)
        .unwrap_or(Some(std::cmp::Ordering::Equal))
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cards = [
            self.cards.0,
            self.cards.1,
            self.cards.2,
            self.cards.3,
            self.cards.4,
        ];
        let cards = cards.iter().map(|c| match c {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 | 1 => 'J',
            10 => 'T',
            _ => std::char::from_digit(*c as u32, 10).unwrap(),
        });
        write!(f, "{}", cards.collect::<String>())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        self_type.partial_cmp(&other_type).and_then(|x| {
            println!("{}:{} {}:{} => {:?}", self_type, self, other_type, other, x);
            if x != std::cmp::Ordering::Equal {
                return Some(x);
            }
            partial_cmp(
                [
                    self.orig_cards?.0,
                    self.orig_cards?.1,
                    self.orig_cards?.2,
                    self.orig_cards?.3,
                    self.orig_cards?.4,
                ],
                [
                    other.orig_cards?.0,
                    other.orig_cards?.1,
                    other.orig_cards?.2,
                    other.orig_cards?.3,
                    other.orig_cards?.4,
                ],
            )
        })
    }
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let hands_bids = TableParser::parse(Rule::hands, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let mut hands_bids = hands_bids
        .map(|hand_bid| {
            let mut hand_bid_iter = hand_bid.into_inner();
            (
                Hand {
                    orig_cards: None,
                    cards: hand_bid_iter
                        .next()
                        .unwrap()
                        .as_str()
                        .chars()
                        .map(|card| parse(card).unwrap_or_else(|| panic!("Invalid card: {}", card)))
                        .collect::<Vec<_>>()
                        .chunks(5)
                        .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]))
                        .last()
                        .unwrap(),
                },
                hand_bid_iter
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<Bid>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(_, _)>>();
    hands_bids.iter_mut().for_each(|(hand, _)| hand.sort());

    hands_bids.sort_by(|(hand_a, _), (hand_b, _)| hand_a.partial_cmp(hand_b).unwrap());

    let total_winnings = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (1 + i as i64) * bid)
        .sum::<i64>();

    for (i, (hand, bid)) in hands_bids.iter().enumerate() {
        println!("{}: {:?} {}", i + 1, hand, bid);
    }

    Ok(total_winnings)
}

fn part2(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let hands_bids = TableParser::parse(Rule::hands, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let mut hands_bids = hands_bids
        .map(|hand_bid| {
            let mut hand_bid_iter = hand_bid.into_inner();
            (
                Hand {
                    orig_cards: None,
                    cards: hand_bid_iter
                        .next()
                        .unwrap()
                        .as_str()
                        .chars()
                        .map(|card| parse(card).unwrap_or_else(|| panic!("Invalid card: {}", card)))
                        .collect::<Vec<_>>()
                        .chunks(5)
                        .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]))
                        .last()
                        .unwrap(),
                },
                hand_bid_iter
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<Bid>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(_, _)>>();
    hands_bids.iter_mut().for_each(|(hand, _)| hand.sort());

    hands_bids.sort_by(|(hand_a, _), (hand_b, _)| hand_a.partial_cmp(hand_b).unwrap());

    let total_winnings = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (1 + i as i64) * bid)
        .sum::<i64>();

    for (i, (hand, bid)) in hands_bids.iter().enumerate() {
        println!("{}: {:?} {}", i + 1, hand, bid);
    }

    Ok(total_winnings)
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1("day06_p1_in")?);
    println!("Part 2: {}", part2("day06_p1_in")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering_of_hands() {
        assert_eq!(
            Some(std::cmp::Ordering::Equal),
            partial_cmp([13, 13, 6, 7, 7], [13, 13, 6, 7, 7])
        );
        assert_eq!(
            Some(std::cmp::Ordering::Greater),
            partial_cmp([13, 13, 6, 7, 7], [13, 10, 11, 11, 10])
        );
        assert_eq!(
            Some(std::cmp::Ordering::Greater),
            partial_cmp([13, 1, 11, 11, 10], [13, 13, 6, 7, 7])
        );
    }

    #[test]
    #[ignore]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day07_p1_ex")?, 6440);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day07_p1_in")?, 249726565);
        Ok(())
    }
    //
    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day07_p1_ex")?, 5905);
        Ok(())
    }
    //
    #[test]
    #[ignore]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day07_p1_in")?, 252149023);
        Ok(())
    }
}
