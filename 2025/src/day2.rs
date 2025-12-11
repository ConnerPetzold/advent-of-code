use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Parser, bytes::complete::tag, character::complete::u64, multi::separated_list0,
    sequence::separated_pair,
};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Range<u64>> {
    separated_list0(
        tag(","),
        separated_pair(
            u64::<&str, nom::error::Error<&str>>,
            tag("-"),
            u64::<&str, nom::error::Error<&str>>,
        )
        .map(|(start, end)| Range {
            start,
            end: end + 1,
        }),
    )
    .parse(input)
    .unwrap()
    .1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Range<u64>]) -> u64 {
    let mut invalid_ids = Vec::new();
    for range in input {
        for id in range.clone() {
            if !is_valid_id(id) {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids.iter().sum()
}

fn is_valid_id(id: u64) -> bool {
    let s = id.to_string();
    let (left, right) = s.split_at(s.len() / 2);
    left != right
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Range<u64>]) -> u64 {
    let mut invalid_ids = Vec::new();
    for range in input {
        for id in range.clone() {
            if !is_valid_id_part2(id) {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids.iter().sum()
}

fn is_valid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    for n in 1..=len / 2 {
        if !len.is_multiple_of(n) {
            continue;
        }
        let sub = s.chars().take(n).collect::<String>();
        if sub.repeat(len / n) == s {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 4174379265);
    }
}
