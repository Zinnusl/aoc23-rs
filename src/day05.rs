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
#[grammar = "day05.pest"]
struct AlmanacParser;

#[derive(Debug)]
struct Map {
    mappings: Vec<(i64, i64, i64)>,
}

struct FromTo {
    from: Interval,
    to: Interval,
}

impl FromTo {
    fn offset(&self) -> i64 {
        self.to.start() - self.from.start()
    }
}

impl Map {
    fn transform(&self, val: i64) -> i64 {
        for mapping in &self.mappings {
            if (mapping.1..=mapping.1 + mapping.2).contains(&val) {
                return val + mapping.0 - mapping.1;
            }
        }

        val
    }

    fn mappings_as_fromtos(&self) -> Vec<FromTo> {
        self.mappings
            .iter()
            .map(|mapping| FromTo {
                from: Interval::new(mapping.1..mapping.1 + mapping.2),
                to: Interval::new(mapping.0..mapping.0 + mapping.2),
            })
            .collect::<Vec<_>>()
    }

    fn transform_interval(&self, location: &Interval) -> Vec<Interval> {
        let mappings = self.mappings_as_fromtos();
        let split = location.split_on_ranges(
            &mappings
                .into_iter()
                .map(|mapping| {
                    let offset = mapping.to.start() - mapping.from.start();
                    (mapping.from, offset)
                })
                .collect::<Vec<(Interval, i64)>>(),
        );
        split
    }
}

fn part1(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let mut almanac = AlmanacParser::parse(Rule::almanac, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let seeds = almanac
        .next()
        .unwrap()
        .into_inner()
        .map(|seed| seed.as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let maps = almanac.map(|pair| Map {
        mappings: pair
            .into_inner()
            .map(|p| p.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|slice| (slice[0], slice[1], slice[2]))
            .collect::<Vec<_>>(),
    });
    let mut locations: Vec<i64> = seeds;
    println!("{:?}", locations);
    if maps.len() != 7 {
        return Err(anyhow!("Wrong number of maps: {}", maps.len()));
    }
    maps.for_each(|map| {
        println!("{:?}", map);
        println!(
            "{:?}",
            locations
                .iter_mut()
                .map(|seed| {
                    *seed = map.transform(*seed);
                    seed
                })
                .collect::<Vec<_>>()
        );
    });
    Ok(*locations.iter().min().unwrap())
}

fn part2_old(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let mut almanac = AlmanacParser::parse(Rule::almanac, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let seeds = almanac
        .next()
        .unwrap()
        .into_inner()
        .map(|seed| seed.as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|slice| (slice[0]..slice[0] + slice[1]).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    // println!("{:?}", seeds);
    let maps = almanac.map(|pair| Map {
        mappings: pair
            .into_inner()
            .map(|p| p.as_str().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|slice| (slice[0], slice[1], slice[2]))
            .collect::<Vec<_>>(),
    });
    let mut locations: Vec<i64> = seeds;
    // println!("{:?}", locations);
    if maps.len() != 7 {
        return Err(anyhow!("Wrong number of maps: {}", maps.len()));
    }
    maps.for_each(|map| {
        // println!("{:?}", map);
        locations.par_iter_mut().for_each(|seed| {
            *seed = map.transform(*seed);
        });
    });
    Ok(*locations.iter().min().unwrap())
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Interval {
    range: std::ops::Range<i64>,
}
impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start(), self.end() - 1)
    }
}
impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}
impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.start.cmp(&other.range.start)
    }
}
impl Interval {
    fn contains(&self, val: i64) -> bool {
        self.range.contains(&val)
    }
    #[requires(range.start < range.end)]
    fn new(range: std::ops::Range<i64>) -> Self {
        Self { range }
    }
    fn overlaps_both_sides_of(&self, other: &Self) -> bool {
        self.range.start < other.range.start && self.range.end > other.range.end
    }
    fn overlaps_left_of(&self, other: &Self) -> bool {
        self.range.start < other.range.start
            && self.range.end <= other.range.end
            && self.range.end > other.range.start
    }
    fn overlaps_right_of(&self, other: &Self) -> bool {
        self.range.start >= other.range.start
            && self.range.end > other.range.end
            && self.range.start < other.range.end
    }
    fn is_contained(&self, other: &Self) -> bool {
        self.range.start >= other.range.start && self.range.end <= other.range.end
    }
    fn offset_by(&self, offset: &i64) -> Self {
        Self {
            range: (self.range.start + offset)..(self.range.end + offset),
        }
    }
    fn split_on_ranges(&self, ranges_and_offset: &[(Interval, i64)]) -> Vec<Interval> {
        let mut ret_split_intervals = vec![self.clone()];
        let mut idx = 0i64;

        println!(
            "vals pre split = {}",
            ret_split_intervals
                .iter()
                .map(|old| old.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        );

        while idx < ret_split_intervals.len() as i64 {
            for (range, _) in ranges_and_offset {
                let this = &mut ret_split_intervals[idx as usize];
                let start = this.range.start.clone();
                let end = this.range.end.clone();
                if this.range.start == 1972667147 {
                    println!("checking this: {}, range: {}", this, range);
                }
                match (this.clone(), range) {
                    (_, _) if this.overlaps_both_sides_of(range) => {
                        println!("range {} overlaps both sides of {}", this, range);
                        // Split into 3
                        let removed = ret_split_intervals.remove(idx as usize);
                        println!("removed interval: {}", removed);
                        ret_split_intervals.push(Interval::new(start..range.start()));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());
                        ret_split_intervals.push(range.clone());
                        println!("new interval: {}", ret_split_intervals.last().unwrap());
                        ret_split_intervals.push(Interval::new(range.end()..end));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());

                        idx = -1;
                        break;
                    }
                    (_, _) if this.overlaps_left_of(range) => {
                        println!("range {} overlaps left of {}", this, range);
                        // Split into 2
                        let removed = ret_split_intervals.remove(idx as usize);
                        println!("removed interval: {}", removed);
                        ret_split_intervals.push(Interval::new(start..range.start()));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());
                        ret_split_intervals.push(Interval::new(range.start()..end));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());

                        idx = -1;
                        break;
                    }
                    (_, _) if this.overlaps_right_of(range) => {
                        println!("range {} overlaps right of {}", this, range);
                        // Split into 2
                        let removed = ret_split_intervals.remove(idx as usize);
                        println!("removed interval: {}", removed);
                        ret_split_intervals.push(Interval::new(start..range.end()));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());
                        ret_split_intervals.push(Interval::new(range.end()..end));
                        println!("new interval: {}", ret_split_intervals.last().unwrap());

                        idx = -1;
                        break;
                    }
                    _ => {}
                }
            }

            idx += 1;
        }

        let old = ret_split_intervals
            .iter()
            .cloned()
            .collect::<Vec<Interval>>();

        println!(
            "vals post split = {}",
            old.iter()
                .map(|old| old.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        );

        ret_split_intervals.iter_mut().for_each(|interval| {
            for (range, offset) in ranges_and_offset {
                match (interval.clone(), range) {
                    (t, r) if t.overlaps_both_sides_of(r) => {
                        panic!("range {} overlaps both sides of {}", t, r);
                    }
                    (t, r) if t.overlaps_left_of(r) => {
                        panic!("range {} overlaps left of {}", t, r);
                    }
                    (t, r) if t.overlaps_right_of(r) => {
                        panic!("range {} overlaps right of {}", t, r);
                    }
                    _ => {}
                }
            }
        });
        ret_split_intervals
            .iter_mut()
            .for_each(|interval: &mut Interval| {
                for (range, offset) in ranges_and_offset {
                    if interval.is_contained(range) {
                        println!(
                            "range {} is contained in {} offset: {}",
                            interval, range, offset
                        );
                        *interval = interval.offset_by(offset);
                        break;
                    }
                }
            });

        if old.len() != ret_split_intervals.len() {
            panic!("old.len() != intervals.len()");
        }
        old.iter()
            .zip(ret_split_intervals.iter())
            .for_each(|(old, new)| {
                println!("{} => {}", old, new);
            });

        ret_split_intervals
    }

    fn start(&self) -> i64 {
        self.range.start
    }
    fn end(&self) -> i64 {
        // Make it inclusive
        self.range.end
    }
}
fn part2(input: &'static str) -> Result<i64> {
    let file_contents = std::fs::read_to_string(input).expect("Could not read file");
    let mut almanac = AlmanacParser::parse(Rule::almanac, file_contents.as_str())
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap()
        .into_inner();

    let seeds = almanac
        .next()
        .unwrap()
        .into_inner()
        .map(|seed| seed.as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|slice| Interval::new(slice[0]..slice[0] + slice[1]))
        .collect::<Vec<Interval>>();
    // println!("{:?}", seeds);
    let maps = almanac
        .map(|pair| Map {
            mappings: pair
                .into_inner()
                .map(|p| p.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|slice| (slice[0], slice[1], slice[2]))
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    let mut locations: Vec<Interval> = seeds;
    // println!("{:?}", locations);
    if maps.len() != 7 {
        return Err(anyhow!("Wrong number of maps: {}", maps.len()));
    }
    // println!("maps[0].fromto[0]: {}, {}", fromtos[0].from, fromtos[0].to);
    // println!("maps[0].fromto[1]: {}, {}", fromtos[1].from, fromtos[1].to);
    for location in &locations {
        println!("{}", location);
    }
    println!("+++");
    maps.iter().for_each(|map| {
        let fromtos = map.mappings_as_fromtos();
        for fromto in &fromtos {
            println!(
                "fromto: {}, {}, {}",
                fromto.from,
                fromto.to,
                fromto.offset()
            );
        }

        let mut new_locations = vec![];
        let mut locations_iter = locations.iter();
        while let Some(location) = locations_iter.next() {
            let new_locals = map.transform_interval(location);
            new_locations.extend(new_locals);
        }

        locations = new_locations;

        // for location in &locations {
        //     println!("{}", location);
        // }
        println!("+++");
    });
    Ok(locations.iter().min().unwrap().start())
}

fn main() -> Result<()> {
    // println!("Part 1: {}", part1("day05_p1_in")?);
    println!("Part 2: {}", part2_old("day05_p1_in")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_test() -> Result<()> {
        let interval = Interval::new(1972667147..2203381571);
        assert_eq!(
            interval.overlaps_left_of(&Interval::new(2032673361..2203381571)),
            true
        );
        Ok(())
    }

    #[test]
    fn numbers_test() -> Result<()> {
        assert_eq!(2378259165u32, 1972667147 + 405592018);
        assert_eq!(1477976316u32, 1450194064 + 27782252);
        assert_eq!(410212617u32, 348350443 + 61862174);
        assert_eq!(4092364215u32, 3911195009 + 181169206);
        assert_eq!(765648080u32, 626861593 + 138786487);
        assert_eq!(3162265119u32, 2886966111 + 275299008);
        assert_eq!(1303406955u32, 825403564 + 478003391);
        assert_eq!(520687690u32, 514585599 + 6102091);
        assert_eq!(2541511753u32, 2526020300 + 15491453);
        assert_eq!(3757205391u32, 3211013652 + 546191739);
        Ok(())
    }

    #[test]
    fn test_part1_ex() -> Result<()> {
        assert_eq!(part1("day05_p1_ex")?, 35);
        Ok(())
    }

    #[test]
    fn test_part1_in() -> Result<()> {
        assert_eq!(part1("day05_p1_in")?, 662197086);
        Ok(())
    }

    #[test]
    fn test_part2_ex() -> Result<()> {
        assert_eq!(part2("day05_p1_ex")?, 46);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_part2_in() -> Result<()> {
        assert_eq!(part2("day05_p1_in")?, 52510809);
        Ok(())
    }
}
