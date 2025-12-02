use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Parser,
    character::complete::{newline, one_of, u32},
    multi::separated_list0,
    sequence::pair,
};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    separated_list0(
        newline::<&str, nom::error::Error<&str>>,
        pair(one_of("LR"), u32),
    )
    .parse(input)
    .unwrap()
    .1
    .into_iter()
    .map(|(l_or_r, n)| if l_or_r == 'L' { -(n as i32) } else { n as i32 })
    .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> u32 {
    let mut landed_on_zero = 0;
    let mut c = 50;
    for n in input {
        c = (c + n) % 100;
        if c == 0 {
            landed_on_zero += 1;
        }
    }
    landed_on_zero
}
