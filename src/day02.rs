#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{Display, Formatter};

use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day02.pest"]
struct TokenParser;

#[derive(Debug)]
struct Draw {
    red: i64,
    green: i64,
    blue: i64,
}

impl Display for Draw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} red, {} green, {} blue",
            self.red, self.green, self.blue
        )
    }
}
struct Game {
    num: i64,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self) -> bool {
        !self
            .draws
            .iter()
            .any(|draw| draw.red > 12 || draw.green > 13 || draw.blue > 14)
    }

    fn power(&self) -> i64 {
        let vals = self.draws.iter().fold((0, 0, 0), |acc, draw| {
            (
                acc.0.max(draw.red),
                acc.1.max(draw.green),
                acc.2.max(draw.blue),
            )
        });
        vals.0 * vals.1 * vals.2
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let draws = self
            .draws
            .iter()
            .map(|draw| format!("{}; ", draw))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "Game {}: {}", self.num, draws)
    }
}

fn part1(input: &'static str) -> i64 {
    let games = std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("line: {:?}", line))
        .map(|line| {
            let mut pairs = TokenParser::parse(Rule::game, line)
                .unwrap_or_else(|e| panic!("{}", e))
                .inspect(|game| {
                    if line.len()
                        != (game.as_span().end_pos().line_col().1
                            - game.as_span().start_pos().line_col().1)
                    {
                        panic!("Line not fully parsed: {:?}\n\n {:?}", game, line);
                    }
                })
                .next()
                .unwrap()
                .into_inner();

            let num = pairs.next().unwrap().as_str().parse::<i64>().unwrap();
            let draws = pairs
                .inspect(|pair| println!("pair: {:?}", pair))
                .map(|pair| {
                    let mut color_pair_iter = pair.into_inner();
                    let mut map = std::collections::HashMap::new();

                    while let Some(color_pair) = color_pair_iter.next() {
                        let mut color_pair_inner = color_pair.into_inner();
                        let num = color_pair_inner
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap();
                        let color = color_pair_inner.next().unwrap().as_str();
                        match color {
                            "red" => map.insert("red", num),
                            "green" => map.insert("green", num),
                            "blue" => map.insert("blue", num),
                            _ => panic!("Expected color"),
                        };
                    }
                    Draw {
                        red: *map.get("red").unwrap_or(&0),
                        green: *map.get("green").unwrap_or(&0),
                        blue: *map.get("blue").unwrap_or(&0),
                    }
                })
                .collect::<Vec<Draw>>();

            Game { num, draws }
        })
        .collect::<Vec<Game>>();

    games
        .iter()
        .filter(|game| game.is_possible())
        .fold(0, |acc, game| acc + game.num)
}

fn part2(input: &'static str) -> i64 {
    let games = std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("line: {:?}", line))
        .map(|line| {
            let mut pairs = TokenParser::parse(Rule::game, line)
                .unwrap_or_else(|e| panic!("{}", e))
                .inspect(|game| {
                    if line.len()
                        != (game.as_span().end_pos().line_col().1
                            - game.as_span().start_pos().line_col().1)
                    {
                        panic!("Line not fully parsed: {:?}\n\n {:?}", game, line);
                    }
                })
                .next()
                .unwrap()
                .into_inner();

            let num = pairs.next().unwrap().as_str().parse::<i64>().unwrap();
            let draws = pairs
                .inspect(|pair| println!("pair: {:?}", pair))
                .map(|pair| {
                    let mut color_pair_iter = pair.into_inner();
                    let mut map = std::collections::HashMap::new();

                    while let Some(color_pair) = color_pair_iter.next() {
                        let mut color_pair_inner = color_pair.into_inner();
                        let num = color_pair_inner
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<i64>()
                            .unwrap();
                        let color = color_pair_inner.next().unwrap().as_str();
                        match color {
                            "red" => map.insert("red", num),
                            "green" => map.insert("green", num),
                            "blue" => map.insert("blue", num),
                            _ => panic!("Expected color"),
                        };
                    }
                    Draw {
                        red: *map.get("red").unwrap_or(&0),
                        green: *map.get("green").unwrap_or(&0),
                        blue: *map.get("blue").unwrap_or(&0),
                    }
                })
                .collect::<Vec<Draw>>();

            Game { num, draws }
        })
        .collect::<Vec<Game>>();

    games
        .iter()
        // .filter(|game| game.is_possible())
        .fold(0, |acc, game| acc + game.power())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1("day02_p1_ex"), 8);
    }

    #[test]
    fn test_part1_in() {
        assert_eq!(part1("day02_p1_in"), 2162);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2("day02_p2_ex"), 2286);
    }

    #[test]
    fn test_part2_in() {
        assert_eq!(part2("day02_p1_in"), 72513);
    }
}
