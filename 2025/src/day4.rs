use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Parser,
    character::complete::{self, one_of},
    multi::{many1, separated_list0},
};

type Grid = gridit::Grid<bool>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    let rows = separated_list0(
        complete::newline::<&str, nom::error::Error<&str>>,
        many1(one_of(".@").map(|c| c == '@')),
    )
    .parse(input)
    .unwrap()
    .1;

    let width = rows[0].len();
    let height = rows.len();
    let items = rows.iter().flatten().cloned().collect::<Vec<_>>();
    Grid::from(items, width, height)
}

fn removable_positions(grid: &Grid) -> Vec<gridit::Position> {
    grid.positions()
        .filter(|&p| {
            matches!(grid.get(p), Some(true)) && grid.neighbors(p).filter(|&&n| n).count() < 4
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(grid: &Grid) -> usize {
    removable_positions(grid).len()
}

#[aoc(day4, part2)]
pub fn solve_part2(grid: &Grid) -> usize {
    let mut total = 0;
    let (width, height) = grid.size();
    let items = grid.iter().cloned().collect::<Vec<_>>();
    let mut grid = Grid::from(items, width, height);

    loop {
        let positions = removable_positions(&grid);
        if positions.is_empty() {
            break;
        }

        for position in positions.iter() {
            grid.set(*position, false);
        }
        total += positions.len();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = input_generator(input);
        assert_eq!(solve_part1(&grid), 13);
    }
}
