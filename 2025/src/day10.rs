use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{newline, one_of, usize},
    multi::{many1, separated_list1},
    sequence::delimited,
};

#[derive(Debug)]
struct Machine {
    light_diagram: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn min_sequence(&self) -> Vec<&Vec<usize>> {
        self.wiring_schematics
            .iter()
            .powerset()
            .filter_map(|sequence| {
                let lights = sequence.iter().fold(
                    vec![false; self.light_diagram.len()],
                    |mut lights, &schematic| {
                        for &light in schematic {
                            lights[light] = !lights[light];
                        }
                        lights
                    },
                );

                if lights == self.light_diagram {
                    Some(sequence)
                } else {
                    None
                }
            })
            .min_by_key(|sequence| sequence.len())
            .unwrap()
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Machine> {
    separated_list1(newline, parse_machine)
        .parse(input)
        .unwrap()
        .1
}

fn parse_machine(input: &str) -> nom::IResult<&str, Machine> {
    let (input, light_diagram) = parse_light_diagram(input)?;
    let (input, wiring_schematics) = parse_wiring_schematics(input)?;
    let (input, joltage_requirements) = parse_joltage_requirements(input)?;

    Ok((
        input,
        Machine {
            light_diagram,
            wiring_schematics,
            joltage_requirements,
        },
    ))
}

fn parse_light_diagram(input: &str) -> nom::IResult<&str, Vec<bool>> {
    delimited(tag("["), many1(one_of(".#")), tag("]"))
        .parse(input)
        .map(|(s, light_diagram)| (s, light_diagram.iter().map(|c| *c == '#').collect()))
}

fn parse_wiring_schematics(input: &str) -> nom::IResult<&str, Vec<Vec<usize>>> {
    delimited(
        tag(" "),
        separated_list1(tag(" "), parse_wiring_schematic),
        tag(" "),
    )
    .parse(input)
}

fn parse_wiring_schematic(input: &str) -> nom::IResult<&str, Vec<usize>> {
    delimited(tag("("), separated_list1(tag(","), usize), tag(")"))
        .parse(input)
        .map(|(s, wiring_schematic)| (s, wiring_schematic.iter().map(|w| *w).collect()))
}

fn parse_joltage_requirements(input: &str) -> nom::IResult<&str, Vec<usize>> {
    delimited(tag("{"), separated_list1(tag(","), usize), tag("}"))
        .parse(input)
        .map(|(s, joltage_requirements)| (s, joltage_requirements.iter().map(|w| *w).collect()))
}

#[aoc(day10, part1)]
pub fn solve_part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| machine.min_sequence().len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let input = input_generator(input);
        let result = solve_part1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part1_longer() {
        let input = r"[#.#..#.##] (0,1,2,5,6,7,8) (1,4,6,7,8) (0,5,7) (0,1,2,6,7) (0,1,2,3,5,7,8) (0,1,5,7) (0,1,3,7,8) {138,150,10,13,17,127,25,155,38}";
        let input = input_generator(input);
        let result = solve_part1(&input);
        assert_eq!(result, 5);
    }
}
