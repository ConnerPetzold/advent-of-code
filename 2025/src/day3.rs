use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    Parser,
    character::complete::{self, satisfy},
    combinator::map,
    multi::{many1, separated_list0},
};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    separated_list0(complete::newline, many1(single_digit))
        .parse(input)
        .unwrap()
        .1
}

fn single_digit(input: &str) -> nom::IResult<&str, u32> {
    map(satisfy(|c| c.is_ascii_digit()), |c| {
        c.to_digit(10).unwrap() as u32
    })
    .parse(input)
}

fn concat(digits: &Vec<u32>) -> u64 {
    digits.iter().fold(0, |acc, elem| acc * 10 + *elem as u64)
}

fn get_joltage(bank: &Vec<u32>, num_batteries: usize) -> u64 {
    let mut batteries = vec![0u32; num_batteries];
    for i in 0..bank.len() - num_batteries + 1 {
        for j in 0..num_batteries {
            let num = bank[i + j];
            if num > batteries[j] {
                batteries[j] = num;
                for k in j + 1..num_batteries {
                    batteries[k] = 0;
                }
            }
        }
    }

    concat(&batteries)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> u64 {
    input.iter().map(|bank| get_joltage(bank, 2)).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<u32>>) -> u64 {
    input.iter().map(|bank| get_joltage(bank, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 357);
    }
}
