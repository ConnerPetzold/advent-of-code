use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{self, u64},
    multi::separated_list0,
    sequence::separated_pair,
};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    separated_pair(
        separated_list0(
            complete::newline::<&str, nom::error::Error<&str>>,
            inclusive_range,
        ),
        (complete::newline, complete::newline),
        separated_list0(complete::newline::<&str, nom::error::Error<&str>>, u64),
    )
    .parse(input)
    .unwrap()
    .1
}

fn inclusive_range(input: &str) -> nom::IResult<&str, RangeInclusive<u64>> {
    separated_pair(u64, tag("-"), u64)
        .parse(input)
        .map(|(input, (start, end))| (input, start..=end))
}

#[aoc(day5, part1)]
pub fn solve_part1((ranges, numbers): &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
    numbers
        .iter()
        .filter(|&number| ranges.iter().any(|range| range.contains(number)))
        .count()
}

#[aoc(day5, part2)]
pub fn solve_part2((ranges, _): &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
    ranges
        .iter()
        .sorted_by(|a, b| a.start().cmp(b.start()))
        .fold(Vec::<RangeInclusive<u64>>::new(), |mut acc, range| {
            let mut combined = false;
            for r in acc.iter_mut() {
                if r.overlaps(range) {
                    *r = r.combine(range);
                    combined = true;
                    break;
                }
            }

            if !combined {
                acc.push(range.clone());
            }

            acc
        })
        .iter()
        .map(|range| range.clone().count())
        .sum()
}

trait RangeExt {
    fn overlaps(&self, other: &RangeInclusive<u64>) -> bool;
    fn combine(&self, other: &RangeInclusive<u64>) -> RangeInclusive<u64>;
}

impl RangeExt for RangeInclusive<u64> {
    fn overlaps(&self, other: &RangeInclusive<u64>) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }

    fn combine(&self, other: &RangeInclusive<u64>) -> RangeInclusive<u64> {
        *self.start().min(other.start())..=*self.end().max(other.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 14);
    }
}
