use aoc_runner_derive::{aoc, aoc_generator};
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

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {
    input
        .to_owned()
        .into_iter()
        .map(|row| {
            let mut first = 0;
            // let mut first_index = 0;
            let mut second = 0;
            let len = row.len();
            for (i, num) in row.into_iter().enumerate() {
                if i < len - 1 && num > first {
                    first = num;
                    second = 0;
                } else if i > 0 {
                    second = second.max(num);
                }
            }
            first * 10 + second
        })
        .sum()
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
