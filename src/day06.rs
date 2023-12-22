#![allow(dead_code)]
#![allow(unused_imports)]

use contracts::*;

use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

use rayon::prelude::*;

use indicatif::{ProgressBar, ProgressStyle};

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Parser)]
#[grammar = "day06.pest"]
struct TableParser;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let mut table = TableParser::parse(Rule::table, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let times = table.next().unwrap().into_inner().collect::<Vec<_>>();
    let distances = table.next().unwrap().into_inner().collect::<Vec<_>>();
    let races = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time: time.as_str().parse::<i64>().unwrap(),
            distance: distance.as_str().parse::<i64>().unwrap(),
        })
        .collect::<Vec<_>>();

    races.iter().for_each(|r| println!("{:?}", r));

    let res = races
        .iter()
        .map(|race| {
            (1..race.time)
                .map(|t| t * (race.time - t) > race.distance)
                .map(|v| if v { 1 } else { 0 })
                .sum::<i64>()
        })
        .product();

    Ok(res)
}
fn part2(input: &'static str) -> Result<i64> {
    part1(input)
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
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day06_p1_ex")?, 288);
        Ok(())
    }

    #[test]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day06_p1_in")?, 1624896);
        Ok(())
    }

    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day06_p2_ex")?, 71503);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day06_p2_in")?, 32583852);
        Ok(())
    }
}
