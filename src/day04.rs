#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

use indicatif::{ProgressBar, ProgressStyle};

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Parser)]
#[grammar = "day04.pest"]
struct CardParser;

fn part1(input: &'static str) -> Result<i64> {
    let pile_points = std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("line: {:?}", line))
        .map(|line| {
            let mut card = CardParser::parse(Rule::card, line)
                .unwrap_or_else(|e| panic!("{}", e))
                .next()
                .unwrap()
                .into_inner();

            let _num_card = card.next().unwrap().as_str().parse::<i64>().unwrap();
            let winning_numbers = card
                .next()
                .unwrap()
                .into_inner()
                .map(|inner_pair| inner_pair.as_str().parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            let have_numbers = card
                .next()
                .unwrap()
                .into_inner()
                .map(|inner_pair| inner_pair.as_str().parse::<i64>().unwrap())
                .collect::<HashSet<_>>();

            let num_winning_numbers =
                winning_numbers.intersection(&have_numbers).count() as i64 - 1;
            if num_winning_numbers >= 0 {
                2i64.pow(num_winning_numbers as u32)
            } else {
                0
            }
        })
        .sum::<i64>();

    Ok(pile_points)
}

#[derive(Debug, Clone, Eq)]
struct Card {
    num_card: i64,
    winning_numbers: HashSet<i64>,
    have_numbers: HashSet<i64>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num_card == other.num_card
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num_card.partial_cmp(&other.num_card)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num_card.cmp(&other.num_card)
    }
}

impl Card {
    fn num_winning_numbers(&self) -> usize {
        self.winning_numbers
            .intersection(&self.have_numbers)
            .count()
    }
    fn eval(&self, cards: &mut VecDeque<CardPile>) -> () {
        let num_winning_numbers = self.num_winning_numbers();

        let idx_init = self.num_card as usize;
        let mut idx = idx_init;
        while idx < cards.len() && idx < idx_init + num_winning_numbers {
            let card_clone = cards[idx].cards.last().unwrap().clone();
            // println!("self: {}; card_clone: {}, idx: {}", self, card_clone, idx);
            cards[idx].cards.push(card_clone);
            idx += 1;
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {} {}", self.num_card, self.num_winning_numbers())
    }
}

#[derive(Debug, Clone)]
struct CardPile {
    cards: Vec<Card>,
}

fn part2(input: &'static str) -> Result<i64> {
    let mut cardpiles = std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("line: {:?}", line))
        .map(|line| {
            let mut card = CardParser::parse(Rule::card, line)
                .unwrap_or_else(|e| panic!("{}", e))
                .next()
                .unwrap()
                .into_inner();

            let num_card = card.next().unwrap().as_str().parse::<i64>().unwrap();
            let winning_numbers = card
                .next()
                .unwrap()
                .into_inner()
                .map(|inner_pair| inner_pair.as_str().parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            let have_numbers = card
                .next()
                .unwrap()
                .into_inner()
                .map(|inner_pair| inner_pair.as_str().parse::<i64>().unwrap())
                .collect::<HashSet<_>>();

            Card {
                num_card,
                winning_numbers,
                have_numbers,
            }
        })
        .map(|card| CardPile { cards: vec![card] })
        .collect::<VecDeque<_>>();

    let mut overall_num_cards = 0;
    let mut cardpile_idx = 0;
    while cardpile_idx < cardpiles.len() {
        let cardpile = cardpiles[cardpile_idx].clone();
        overall_num_cards += cardpile.cards.len();
        println!("cardpile len: {}", cardpile.cards.len());
        cardpile
            .cards
            .iter()
            .for_each(|card| card.eval(&mut cardpiles));

        cardpile_idx += 1;
    }

    Ok(overall_num_cards as i64)
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1("day04_p1_in")?);
    println!("Part 2: {}", part2("day04_p1_in")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_log() {
        assert_eq!(1, 2i64.pow(0));
        assert_eq!(2, 2i64.pow(1));
        assert_eq!(4, 2i64.pow(2));
    }

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day04_p1_ex")?, 13);
        Ok(())
    }

    #[test]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day04_p1_in")?, 25174);
        Ok(())
    }

    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day04_p1_ex")?, 30);
        Ok(())
    }

    #[test]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day04_p1_in")?, 6420979);
        Ok(())
    }
}
