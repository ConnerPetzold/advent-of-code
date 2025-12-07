use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{UVec2, uvec2};

#[derive(Debug, Clone)]
struct Manifold {
    size: UVec2,
    start: UVec2,
    splitters: HashSet<UVec2>,
}

impl Manifold {
    fn num_times_split(&self) -> usize {
        let mut count = 0;
        let mut beams = HashSet::new();
        beams.insert(self.start);

        while !beams.is_empty() {
            let mut new_beams = HashSet::new();
            for beam in beams.clone().iter() {
                let next = beam + UVec2::Y;

                if self.splitters.contains(&next) {
                    count += 1;
                    beams.remove(beam);
                    new_beams.insert(next + UVec2::X);
                    new_beams.insert(next - UVec2::X);
                } else if next.y < self.size.y {
                    new_beams.insert(next);
                }
            }
            beams = new_beams;
        }

        count
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
    manifold.num_times_split()
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
}
