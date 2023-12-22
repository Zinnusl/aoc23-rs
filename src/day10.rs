// #![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::{anyhow, Result};
use contracts::*;
use indicatif::{ProgressBar, ProgressStyle};
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{char, digit1, line_ending, space0, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use pest::Parser;
use pest_derive::Parser;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
struct Pipe {
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
    up: Option<usize>,
    down: Option<usize>,
}

impl Pipe {
    fn new(id: usize) -> Self {
        Self {
            id,
            left: None,
            right: None,
            up: None,
            down: None,
        }
    }
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let line_len = file_contents.lines().next().unwrap().len();

    // Possible pipes
    // -
    // |
    // L
    // J
    // F
    // 7
    // Ground
    // . (dot) [Same as pipe with no connections]
    let mut pipes = file_contents
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '-' => Some(Pipe::new(x + y * line_len)),
                    '|' => Some(Pipe::new(x + y * line_len)),
                    'L' => Some(Pipe::new(x + y * line_len)),
                    'J' => Some(Pipe::new(x + y * line_len)),
                    'F' => Some(Pipe::new(x + y * line_len)),
                    '7' => Some(Pipe::new(x + y * line_len)),
                    '.' => Some(Pipe::new(x + y * line_len)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter_map(|p| p)
        .collect::<Vec<_>>();

    // Connect pipes
    for y in 0..line_len {
        for x in 0..line_len {
            let i = x + y * line_len;
            let mut pipe = pipes.get_mut(i).unwrap();
            if let Some(p) = pipes.get(i + 1) {
                pipe.right = Some(p.id);
            }
            if let Some(p) = pipes.get(i - 1) {
                pipe.left = Some(p.id);
            }
            if let Some(p) = pipes.get(i + line_len) {
                pipe.down = Some(p.id);
            }
            if let Some(p) = pipes.get(i - line_len) {
                pipe.up = Some(p.id);
            }
        }
    }

    Ok(0)
}

fn part2(input: &'static str) -> Result<i64> {
    let _file_contents = std::fs::read_to_string(input).expect("Could not read file");

    Ok(0)
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1("day10_p1_in")?);
    // println!("Part 2: {}", part2("day08_p1_in")?);
    // println!("Part 2: {}", part2("day08_p1_in")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day10_p1_ex")?, 4);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day10_p1_in")?, 2174807968);
        Ok(())
    }
    //
    #[test]
    #[ignore]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day10_p1_ex")?, 2);
        Ok(())
    }
    //
    #[test]
    #[ignore]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day10_p1_in")?, 1208);
        Ok(())
    }
}
