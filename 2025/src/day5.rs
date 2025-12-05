use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
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
}
