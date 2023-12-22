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

struct SingleLinkedNode<T> {
    next: *mut SingleLinkedNode<T>,
    value: T,
}
struct SingleLinkedList<T> {
    head: *mut SingleLinkedNode<T>,
}

// impl<T> Drop for SingleLinkedList<T> {
//     fn drop(&mut self) {
//         let mut current = self;
//         while let Some(next) = current.next() {
//             unsafe {
//                 let box = Box::from_raw(current.head);
//                 current = unsafe { &*next };
//             };
//         }
//     }
// }

impl FromIterator<i64> for SingleLinkedList<i64> {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut res = SingleLinkedList {
            head: std::ptr::null_mut(),
        };
        let current = &mut res;

        if let Some(first) = iter.next() {
            current.head = Box::into_raw(Box::new(SingleLinkedNode {
                next: std::ptr::null_mut(),
                value: first,
            }));
            let mut current = unsafe { &mut *current.head };

            while let Some(value) = iter.next() {
                current.next = Box::into_raw(Box::new(SingleLinkedNode {
                    next: std::ptr::null_mut(),
                    value,
                }));
                current = unsafe { &mut *current.next };
            }
        }
        res
    }
}

impl Iterator for SingleLinkedList<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            None
        } else {
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;
            Some(head.value)
        }
    }
}

#[derive(Debug)]
struct Pyramid {
    data: Vec<Vec<i64>>,
}
impl Pyramid {
    fn new(seq: impl IntoIterator<Item = i64>) -> Self {
        let iter = seq.into_iter();
        let first_row = iter.collect::<Vec<_>>();
        let mut data = Vec::new();
        let mut next_len = first_row.len() - 1;
        data.push(first_row);
        while next_len > 0 && !data.last().unwrap().iter().all(|x| *x == 0) {
            data.push(data.last().unwrap().windows(2).map(|x| x[1] - x[0]).collect());
            next_len -= 1;
        }
        Self { data }
    }
    fn first_row(&self) -> &[i64] {
        &self.data[0]
    }
    fn extrapolated_value(&self) -> i64 {
        *self.data[0].last().unwrap()
    }
    fn extrapolated_value_front(&self) -> i64 {
        self.data[0][0]
    }
    fn extrapolate(&mut self) {
        let len = self.data.len() as i64;
        let mut idx: i64 = self.data.len() as i64 - 1;
        while idx >= 0 {
            if idx == len - 1 {
                let row = &mut self.data[idx as usize];
                row.push(0);
                idx -= 1;
                continue;
            }

            let last_val = *self.data[idx as usize + 1].last().unwrap();
            let row = &mut self.data[idx as usize];
            let val = row.last().unwrap();
            row.push(*val + last_val);

            idx -= 1;
        }
    }
    fn extrapolate_front(&mut self) {
        let len = self.data.len() as i64;
        let mut idx: i64 = self.data.len() as i64 - 1;
        while idx >= 0 {
            if idx == len - 1 {
                let row = &mut self.data[idx as usize];
                row.insert(0, 0);
                idx -= 1;
                continue;
            }

            let first_val = *self.data[idx as usize + 1].first().unwrap();
            let row = &mut self.data[idx as usize];
            let val = row.first().unwrap();
            row.insert(0, *val - first_val);

            idx -= 1;
        }
    }
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let lines = file_contents.lines();
    let entries = lines
        .map(|line| {
            line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<SingleLinkedList<_>>()
        });

    let mut pyramids = entries.map(|entry| Pyramid::new(entry)).collect::<Vec<_>>();

    pyramids.iter().for_each(|pyramid| {
        for row in pyramid.data.iter() {
            println!("{:?}", row);
        }
    });

    pyramids.iter_mut().for_each(|pyramid| pyramid.extrapolate());

    pyramids.iter().for_each(|pyramid| {
        for row in pyramid.data.iter() {
            println!("{:?}", row);
        }
    });

    Ok(pyramids.iter().map(|pyramid| pyramid.extrapolated_value()).sum::<i64>() as i64)
}

fn part2(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let lines = file_contents.lines();
    let entries = lines
        .map(|line| {
            line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<SingleLinkedList<_>>()
        });

    let mut pyramids = entries.map(|entry| Pyramid::new(entry)).collect::<Vec<_>>();

    pyramids.iter().for_each(|pyramid| {
        for row in pyramid.data.iter() {
            println!("{:?}", row);
        }
    });

    pyramids.iter_mut().for_each(|pyramid| pyramid.extrapolate_front());

    pyramids.iter().for_each(|pyramid| {
        for row in pyramid.data.iter() {
            println!("{:?}", row);
        }
    });

    Ok(pyramids.iter().map(|pyramid| pyramid.extrapolated_value_front()).sum::<i64>() as i64)
}

fn main() -> Result<()> {
    // println!("Part 1: {}", part1("day08_p1_in")?);
    // println!("Part 2: {}", part2("day08_p1_in")?);
    println!("Part 2: {}", part2("day08_p1_in")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day09_p1_ex")?, 114);
        Ok(())
    }

    #[test]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day09_p1_in")?, 2174807968);
        Ok(())
    }
    //
    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day09_p1_ex")?, 2);
        Ok(())
    }
    //
    #[test]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day09_p1_in")?, 1208);
        Ok(())
    }
}
