use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{UVec2, uvec2};
use pathfinding::prelude::{bfs_reach, count_paths};

#[derive(Debug, Clone)]
struct Manifold {
    size: UVec2,
    start: UVec2,
    splitters: HashSet<UVec2>,
}

impl Manifold {
    fn times_split(&self) -> usize {
        bfs_reach(self.start, |&pos| {
            let beams = if self.splitters.contains(&pos) {
                vec![pos + UVec2::X, pos - UVec2::X]
            } else {
                vec![pos]
            };

            beams.into_iter().filter_map(|beam| {
                let mut next = beam + UVec2::Y;

                while next.y < self.size.y {
                    if self.splitters.contains(&next) {
                        return Some(next);
                    }
                    next += UVec2::Y;
                }

                None
            })
        })
        .count()
            - 1
    }

    fn timelines(&self) -> usize {
        count_paths(
            self.start,
            |&pos| {
                let next = pos + UVec2::Y;
                if self.splitters.contains(&next) {
                    vec![pos + UVec2::X, pos - UVec2::X].into_iter()
                } else {
                    vec![next].into_iter()
                }
            },
            |&pos| pos.y == self.size.y,
        )
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Manifold {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().count() as u32;

    let mut start = None;
    let mut splitters = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some(uvec2(x as u32, y as u32));
                }
                '^' => {
                    splitters.insert(uvec2(x as u32, y as u32));
                }
                _ => (),
            }
        }
    }

    Manifold {
        size: UVec2::new(width, height),
        start: start.expect("No start position found"),
        splitters,
    }
}

#[aoc(day7, part1)]
fn solve_part1(manifold: &Manifold) -> usize {
    manifold.times_split()
}

#[aoc(day7, part2)]
fn solve_part2(manifold: &Manifold) -> usize {
    manifold.timelines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 40);
    }
}
