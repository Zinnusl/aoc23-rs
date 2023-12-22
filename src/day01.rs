use onig::Regex;

fn part1(input: &'static str) -> i64 {
    std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let re = Regex::new(r"\d").unwrap();
            let vals = re
                .captures_iter(&line)
                .map(|m| m.at(0).unwrap())
                .map(|s| match s {
                    _ => s,
                })
                .map(|s| {
                    s.parse::<i64>()
                        .expect(format!("Could not parse int from string {}", s).as_str())
                });
            let mut pa = vals.peekable();

            let f = *pa.peek().unwrap_or(&0);
            let l = pa.last().unwrap_or(0);
            let lg = 10i64.pow(l.ilog10() + 1);
            f * lg + l
        })
        .inspect(|x| println!("x: {:?}", x))
        .sum()
}

fn part2(input: &'static str) -> i64 {
    std::fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
            let vals = re
                .captures_iter(&line)
                .map(|m| m.at(1).unwrap())
                // .inspect(|x| println!("x: {:?} line {:?}", x, line))
                .map(|s| match s {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => s,
                })
                .map(|s| {
                    s.parse::<i64>()
                        .expect(format!("Could not parse int from string \"{}\"", s).as_str())
                });
            let mut pa = vals.peekable();

            let f = *pa.peek().unwrap_or(&0);
            let l = pa.last().unwrap_or(0);
            let lg = 10i64.pow(l.ilog10() + 1);
            f * lg + l
        })
        .inspect(|x| println!("x: {:?}", x))
        .sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        let a = 3i64;
        let b = 17i64;
        let c = 10i64.pow(b.ilog10() + 1);
        assert_eq!(c, 100);
        assert_eq!(c * a + b, 317);
    }

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1("day01_p1_ex"), 142);
    }

    #[test]
    fn test_part1_in() {
        assert_eq!(part1("day01_p1_in"), 55172);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2("day01_p2_ex"), 281);
    }

    #[test]
    fn test_part2_in() {
        assert_eq!(part2("day01_p2_in"), 54925);
    }
}
