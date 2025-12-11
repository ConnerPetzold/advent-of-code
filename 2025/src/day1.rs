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

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> u32 {
    let mut pointed_at_zero = 0;
    let mut c = 50;
    for mut n in input.iter().copied() {
        if n.is_negative() {
            pointed_at_zero += (n / 100).unsigned_abs();
            n %= 100;
            c = (c + n) % 100;
            if c.is_negative() {
                if c != n {
                    pointed_at_zero += 1;
                }
                c += 100;
            }
            c %= 100;
            if c == 0 {
                pointed_at_zero += 1;
            }
        } else {
            c += n;
            pointed_at_zero += (c / 100) as u32;
            c %= 100;
        }
    }
    pointed_at_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 6);
    }
}
