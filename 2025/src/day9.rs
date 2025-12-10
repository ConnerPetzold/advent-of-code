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

#[aoc(day9, part2)]
pub fn solve_part2(points: &[I64Vec2]) -> i64 {
    let lines: Vec<(&I64Vec2, &I64Vec2)> = points.iter().circular_tuple_windows().collect_vec();

    points
        .iter()
        .tuple_combinations()
        .filter(|(ba, bb)| {
            !lines
                .iter()
                .any(|(la, lb)| line_intersects_box(ba, bb, la, lb))
        })
        .map(|(a, b)| (a, b, ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .next_back()
        .unwrap()
        .2
}

fn line_intersects_box(ba: &I64Vec2, bb: &I64Vec2, la: &I64Vec2, lb: &I64Vec2) -> bool {
    let bxmin = ba.x.min(bb.x);
    let bxmax = ba.x.max(bb.x);
    let bymin = ba.y.min(bb.y);
    let bymax = ba.y.max(bb.y);

    let lxmin = la.x.min(lb.x);
    let lxmax = la.x.max(lb.x);
    let lymin = la.y.min(lb.y);
    let lymax = la.y.max(lb.y);

    !(lxmax <= bxmin || lxmin >= bxmax || lymin >= bymax || lymax <= bymin)
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
        let input = input_generator(input);
        let result = solve_part1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let input = input_generator(input);
        let result = solve_part2(&input);
        assert_eq!(result, 24);
    }
}
