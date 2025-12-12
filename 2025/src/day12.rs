use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of, usize},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};

type Shape = [bool; 9];
type Region = ((usize, usize), Vec<usize>);

#[aoc_generator(day12)]
fn input_generator(input: &str) -> (Vec<Shape>, Vec<Region>) {
    (many1(parse_shape), separated_list1(newline, parse_region))
        .parse(input)
        .unwrap()
        .1
}

fn parse_shape(input: &str) -> nom::IResult<&str, Shape> {
    delimited(
        (digit1, tag(":"), newline),
        separated_list1(newline, many1(one_of(".#").map(|c| c == '#')))
            .map(|v| v.into_iter().flatten().collect_vec().try_into().unwrap()),
        (newline, newline),
    )
    .parse(input)
}

fn parse_region(input: &str) -> nom::IResult<&str, Region> {
    separated_pair(
        separated_pair(usize, tag("x"), usize),
        tag(": "),
        separated_list1(tag(" "), usize),
    )
    .parse(input)
}

fn is_region_valid(region: &Region, shapes: &[Shape]) -> bool {
    let available_area = region.0.0 * region.0.1;
    region
        .1
        .iter()
        .enumerate()
        .map(|(i, n)| shapes[i].iter().filter(|b| **b).count() * n)
        .sum::<usize>()
        < available_area
}

#[aoc(day12, part1)]
fn solve_part1((shapes, regions): &(Vec<Shape>, Vec<Region>)) -> usize {
    regions
        .iter()
        .filter(|r| is_region_valid(r, shapes))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        let input = input_generator(input);
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }
}
