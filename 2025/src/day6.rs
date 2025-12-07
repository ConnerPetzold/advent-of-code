use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Parser,
    character::complete::{self, one_of, u64},
    multi::separated_list1,
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

#[aoc_generator(day6, part1)]
fn input_generator_part_1(input: &str) -> Vec<Problem> {
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

#[aoc_generator(day6, part2)]
fn input_generator_part_2(input: &str) -> Vec<Problem> {
    let lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let (op_line, num_lines) = lines.split_last().unwrap();

    let mut problems = Vec::new();
    let mut p = Problem {
        op: Operator::Add,
        nums: Vec::new(),
    };

    for (i, c) in op_line.chars().enumerate() {
        if let Some(op) = match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => None,
        } {
            if !p.nums.is_empty() {
                problems.push(p.clone());
            }
            p = Problem {
                op,
                nums: Vec::new(),
            };
        }

        let mut num_str = String::new();
        for num_line in num_lines {
            num_str.push(num_line.chars().nth(i).unwrap());
        }
        if let Ok(num) = num_str.trim().parse() {
            p.nums.push(num);
        }
    }
    if !p.nums.is_empty() {
        problems.push(p.clone());
    }

    problems
}

#[aoc(day6, part1)]
fn solve_part1(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(|problem| problem.solve()).sum()
}

#[aoc(day6, part2)]
fn solve_part2(problems: &Vec<Problem>) -> u64 {
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
        let input = input_generator_part_1(input);
        assert_eq!(solve_part1(&input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let input = input_generator_part_2(input);
        assert_eq!(solve_part2(&input), 3263827);
    }
}
