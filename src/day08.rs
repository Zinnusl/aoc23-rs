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

// struct SingleLinkedList<T> {
//     next: *const SingleLinkedList<T>,
//     value: Option<T>,
// }
//
// impl<T> Drop for SingleLinkedList<T> {
//     fn drop(&mut self) {
//         let mut current = self;
//         while let Some(next) = current.next() {
//             current = unsafe { &*next };
//         }
//
//         drop(current);
//     }
// }
//
// impl<T> SingleLinkedList<T> {
//     fn next(&self) -> *const SingleLinkedList<T> {
//         self.next
//     }
// }
//
// impl FromIterator<i32> for SingleLinkedList<i32> {
//     fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
//         let mut iter = iter.into_iter();
//         let mut head = SingleLinkedList {
//             value: iter.next(),
//             next: std::ptr::null(),
//         };
//         let mut current = &mut head;
//
//         while let Some(value) = iter.next() {
//
//         }
//
//         head
//     }
// }
//
// impl Iterator for SingleLinkedList<i32> {
//     type Item = i32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let next = unsafe { &*self.next };
//         if next.next == std::ptr::null() {
//             None
//         } else {
//             Some(next.value)
//         }
//     }
// }

#[derive(Debug, Clone)]
struct Instructions(String);

// RRRLRLRLRLRLRLRLRLRLRLLRLRRLLRLRLRLRRLRLRLRLLLLLLRLRRRLRLLRRLRLRLRRRRLRLLRLRLRLLRLR
fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, instructions) =
        map(take_until("\n"), |s: &str| Instructions(s.to_string()))(input)?;

    Ok((input, instructions))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

// AAA = (BBB, CCC)
fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, (id, _, left, _, right, _)) = tuple((
        map(take(3usize), |s: &str| s.to_string()),
        tag(" = ("),
        map(take(3usize), |s: &str| s.to_string()),
        tag(", "),
        map(take(3usize), |s: &str| s.to_string()),
        tag(")"),
    ))(input)?;

    // println!("id: {:?}, left: {:?}, right: {:?}", id, left, right);

    Ok((
        input,
        Node {
            id: id.chars().collect::<Vec<char>>().try_into().unwrap(),
            left: left.chars().collect::<Vec<char>>().try_into().unwrap(),
            right: right.chars().collect::<Vec<char>>().try_into().unwrap(),
        },
    ))
}

fn empty_line(input: &str) -> IResult<&str, &str> {
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, ""))
}

// <Instructions>
//
// <Node>
// <Node>
// <Node>
// ...
fn parse_file_contents(input: &str) -> IResult<&str, (Instructions, Vec<Node>)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = empty_line(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;

    Ok((input, (instructions, nodes)))
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let (_, (instructions, nodes)) = parse_file_contents(&file_contents).expect("Could not parse");

    let mut jumps = 0;
    let mut current_node = nodes
        .iter()
        .find(|n| n.id == ['A', 'A', 'A'])
        .ok_or(anyhow!("Could not find starting node"))?;
    assert_eq!(current_node.id, ['A', 'A', 'A']);
    while current_node.id != ['Z', 'Z', 'Z'] {
        for instruction in instructions.0.chars() {
            jumps += 1;
            match instruction {
                'R' => {
                    current_node = nodes
                        .iter()
                        .find(|n| n.id == current_node.right)
                        .ok_or(anyhow!("Could not find right: {:?}", current_node.right))?;
                }
                'L' => {
                    current_node = nodes
                        .iter()
                        .find(|n| n.id == current_node.left)
                        .ok_or(anyhow!("Could not find left: {:?}", current_node.left))?;
                }
                _ => panic!("Unknown instruction: {}", instruction),
            }
        }
    }

    Ok(jumps)
}

fn part2(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let (_, (instructions, nodes)) = parse_file_contents(&file_contents).expect("Could not parse");

    let edges = nodes
        .iter()
        .map(|n| -> (&Node, (&Node, &Node)) {
            (
                nodes.iter().find(|fiter| fiter.id == n.id).unwrap(),
                (
                    nodes.iter().find(|fiter| fiter.id == n.left).unwrap(),
                    nodes.iter().find(|fiter| fiter.id == n.right).unwrap(),
                ),
            )
        })
        .collect::<HashMap<&Node, (&Node, &Node)>>();

    // For every node, count the number of steps to a node that ends in 'Z'.
    let step_map: HashMap<&Node, i64> = nodes
        .iter()
        .filter(|n| match n.id {
            [_, _, 'A'] => true,
            _ => false,
        })
        .map(|n| {
            let mut current_node = n;
            let mut steps = 0;
            while current_node.id.last().unwrap() != &'Z' {
                steps += 1;
                for instruction in instructions.0.chars() {
                    match instruction {
                        'L' => {
                            current_node = &edges.get(current_node).unwrap().0;
                        }
                        'R' => {
                            current_node = &edges.get(current_node).unwrap().1;
                        }
                        _ => panic!("Unknown instruction: {}", instruction),
                    }
                }
            }

            (n, steps)
        })
        .collect();

    // kA wie man darauf kommt, aber es ist die Lösung.
    let jumps = step_map.iter().map(|(_, n)| n).product::<i64>() * instructions.0.len() as i64;

    Ok(jumps)
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
    fn test_part1_ex2() -> Result<()> {
        assert_eq!(part1("day08_p1_ex2")?, 6);
        Ok(())
    }

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day08_p1_ex")?, 2);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day08_p1_in")?, 16409);
        Ok(())
    }
    //
    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day08_p2_ex")?, 6);
        Ok(())
    }
    //
    #[test]
    #[ignore]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day08_p1_in")?, 0);
        Ok(())
    }
}
