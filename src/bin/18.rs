use std::str::FromStr;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, hex_digit1, newline, space1},
    combinator::{map, map_res, rest},
    multi::separated_list1,
    number,
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(18);

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            "L" => Ok(Dir::L),
            "R" => Ok(Dir::R),
            _ => unreachable!("invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Dir,
    steps: usize,
    color: String,
}

fn dig_instruction(input: &str) -> IResult<&str, (Dir, usize)> {
    separated_pair(
        map_res(alpha1, str::parse),
        nom::character::complete::space1,
        map_res(nom::character::complete::digit1, str::parse),
    )(input)
}

fn hex_string(input: &str) -> IResult<&str, &str> {
    delimited(tag("(#"), hex_digit1, tag(")"))(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        newline,
        map(
            separated_pair(dig_instruction, space1, hex_string),
            |((direction, steps), color)| Instruction {
                direction,
                steps,
                color: String::from(color),
            },
        ),
    )(input)
}

fn parse(input: &str) -> Vec<Instruction> {
    parse_lines(input).unwrap().1
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse(input);
    dbg!(&instructions);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
