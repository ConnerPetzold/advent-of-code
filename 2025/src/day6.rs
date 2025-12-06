use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Parser,
    character::complete::{self, one_of, u64},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    pub nums: Vec<u64>,
    pub op: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Operator::Add => self.nums.iter().sum(),
            Operator::Multiply => self.nums.iter().product(),
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Problem> {
    let (nums, ops) = separated_pair(
        separated_list1(
            complete::newline::<&str, nom::error::Error<&str>>,
            terminated(
                preceded(complete::space0, separated_list1(complete::space1, u64)),
                complete::space0,
            ),
        ),
        complete::newline,
        terminated(
            preceded(
                complete::space0,
                separated_list1(
                    complete::space1,
                    one_of("+*").map(|op| match op {
                        '+' => Operator::Add,
                        '*' => Operator::Multiply,
                        _ => unreachable!(),
                    }),
                ),
            ),
            complete::space0,
        ),
    )
    .parse(input)
    .unwrap()
    .1;

    let mut problems = Vec::new();
    for i in 0..ops.len() {
        problems.push(Problem {
            nums: nums.iter().map(|nums| nums[i]).collect(),
            op: ops[i],
        });
    }

    problems
}

#[aoc(day6, part1)]
pub fn solve_part1(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(|problem| problem.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 4277556);
    }
}
