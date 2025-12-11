use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
};
use pathfinding::prelude::{bfs_reach, count_paths};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> HashMap<String, Vec<String>> {
    separated_list1(
        newline::<&str, nom::error::Error<&str>>,
        separated_pair(
            alpha1::<&str, nom::error::Error<&str>>.map(|s| s.to_string()),
            tag(": "),
            separated_list1(
                tag(" "),
                alpha1::<&str, nom::error::Error<&str>>.map(|s| s.to_string()),
            ),
        ),
    )
    .parse(input)
    .unwrap()
    .1
    .into_iter()
    .collect()
}

#[aoc(day11, part1)]
fn solve_part1(devices: &HashMap<String, Vec<String>>) -> usize {
    count_paths(
        "you".to_string(),
        |id| devices.get(id).cloned().unwrap_or_default(),
        |id| id == "out",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let input = input_generator(input);
        let result = solve_part1(&input);
        assert_eq!(result, 5);
    }
}
