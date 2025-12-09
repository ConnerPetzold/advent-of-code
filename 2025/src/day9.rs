use aoc_runner_derive::{aoc, aoc_generator};
use glam::{I64Vec2, i64vec2};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list0,
    sequence::separated_pair,
};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<I64Vec2> {
    separated_list0(newline, parse_vec2).parse(input).unwrap().1
}

fn parse_vec2(input: &str) -> nom::IResult<&str, I64Vec2> {
    separated_pair(i64, tag(","), i64)
        .parse(input)
        .map(|(s, (x, y))| (s, i64vec2(x, y)))
}

#[aoc(day9, part1)]
pub fn solve_part1(points: &[I64Vec2]) -> i64 {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .next_back()
        .unwrap()
        .2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let input = dbg!(input_generator(input));
        let result = solve_part1(&input);
        assert_eq!(result, 50);
    }
}
