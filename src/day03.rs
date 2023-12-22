#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

struct PartNum<'a> {
    hay: &'a str,
    idx: usize,
    width: usize,
}

impl<'a> PartNum<'a> {
    fn new(hay: &'a str, idx: usize, width: usize) -> PartNum<'a> {
        Self { idx, hay, width }
    }

    fn num(&self) -> Result<i64> {
        self.hay[self.idx..]
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .map_err(|e| anyhow!("Could not parse partnum: {}", e))
    }

    fn is_symbol(sym: char) -> bool {
        match sym {
            '0'..='9' => false,
            '.' => false,
            _ => true,
        }
    }

    fn is_adjacent(&self, idx: usize) -> Result<bool> {
        let len = self.num()?.ilog10() as usize + 1;
        Ok(vec![
            vec![self.idx - 1usize],
            vec![self.idx + len],
            ((self.idx - 1usize - self.width)..(self.idx + len + 1usize - self.width)).collect(),
            ((self.idx - 1usize + self.width)..(self.idx + len + 1usize + self.width)).collect(),
        ]
        .iter()
        .flatten()
        .any(|adjacent_usize| idx == *adjacent_usize))
    }

    fn is_valid(&self) -> Result<bool> {
        Ok(self
            .hay
            .chars()
            .enumerate()
            .any(|(i, c)| Self::is_symbol(c) && self.is_adjacent(i).unwrap_or(false)))
    }
}

fn part1(input: &'static str) -> Result<i64> {
    let file: String = std::fs::read_to_string(input).expect("Could not read file");
    let schematic_width = file.find('\n').unwrap() + 2;

    let file = file.replace("\n", "..");
    let file = ".".repeat(schematic_width + 1) + &file + &".".repeat(schematic_width - 1);

    // std::fs::write("day03_p1_out", &file).expect("Could not write out file");

    let mut it = file.chars().enumerate();
    let mut res = 0;
    while let Some(partnum) = (&mut it)
        .skip_while(|(_, c)| c.is_digit(10))
        .skip_while(|(_, c)| !c.is_digit(10))
        .take(1)
        .map(|(i, _)| PartNum::new(&file, i, schematic_width))
        .nth(0)
    {
        println!("{}:{}", partnum.num().unwrap_or(0), partnum.is_valid()?);
        if partnum.is_valid()? {
            res += partnum.num()?;
        }
    }

    Ok(res)
}

fn part2(input: &'static str) -> Result<i64> {
    let file: String = std::fs::read_to_string(input).expect("Could not read file");
    let schematic_width = file.find('\n').unwrap() + 2;

    let file = file.replace("\n", "..");
    let file = ".".repeat(schematic_width + 1) + &file + &".".repeat(schematic_width - 1);

    // std::fs::write("day03_p1_out", &file).expect("Could not write out file");

    let mut it = file.chars().enumerate();
    let mut parts = vec![];
    while let Some(partnum) = (&mut it)
        .skip_while(|(_, c)| c.is_digit(10))
        .skip_while(|(_, c)| !c.is_digit(10))
        .take(1)
        .map(|(i, _)| PartNum::new(&file, i, schematic_width))
        .nth(0)
    {
        println!("{}:{}", partnum.num().unwrap_or(0), partnum.is_valid()?);
        parts.push(partnum);
    }

    Ok(file
        .chars()
        .enumerate()
        .filter(|(_, c)| c == &'*')
        .map(|(i, _)| {
            let p = parts
                .iter()
                .filter(|part| part.is_adjacent(i).unwrap_or(false))
                .collect::<Vec<_>>();
            if p.len() == 2 {
                p.iter()
                    .map(|part| part.num().unwrap_or(0))
                    .product::<i64>()
            } else {
                0
            }
        })
        .sum::<i64>())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day03_p1_ex")?, 4361);
        Ok(())
    }

    #[test]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day03_p1_in")?, 512794);
        Ok(())
    }

    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day03_p1_ex")?, 467835);
        Ok(())
    }

    #[test]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day03_p1_in")?, 67779080);
        Ok(())
    }
}
