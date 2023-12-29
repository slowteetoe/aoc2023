use std::{collections::BTreeSet, str::FromStr};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, hex_digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(18);

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

fn _display(boundary: &BTreeSet<(i32, i32)>, insides: &BTreeSet<(i32, i32)>) {
    let min_x = boundary.iter().map(|pt| pt.0).min().unwrap();
    let min_y = boundary.iter().map(|pt| pt.1).min().unwrap();
    // assert!(min_x == min_y && min_x == 0, "origin is not (0,0)");
    let max_x = boundary.iter().map(|pt| pt.0).max().unwrap();
    let max_y = boundary.iter().map(|pt| pt.1).max().unwrap();
    (min_x..=max_y).for_each(|y| {
        (min_y..=max_x).for_each(|x| {
            if boundary.contains(&(x, y)) {
                print!("#");
            } else if insides.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        });
        println!();
    });
}

fn fill(boundary: &BTreeSet<(i32, i32)>, start: (i32, i32)) -> BTreeSet<(i32, i32)> {
    // just try to flood fill UDLR from a point we're hoping is inside the shape
    let mut frontier = vec![start];
    let mut insides = BTreeSet::new();
    loop {
        if frontier.is_empty() {
            break;
        }
        frontier = frontier
            .iter()
            .map(|pt| {
                let mut add_to_insides = BTreeSet::new();
                let up = (pt.0, pt.1 - 1);
                let down = (pt.0, pt.1 + 1);
                let left = (pt.0 - 1, pt.1);
                let right = (pt.0 + 1, pt.1);
                let next = vec![up, down, left, right];
                let result = next
                    .iter()
                    .filter(|p| {
                        !boundary.contains(p)
                            && !frontier.contains(p)
                            && !insides.contains(&(p.0, p.1))
                    })
                    .map(|p1| {
                        add_to_insides.insert((p1.0, p1.1));
                        (p1.0, p1.1)
                    })
                    .collect_vec();
                insides.append(&mut add_to_insides);
                result
            })
            .flatten()
            .map(|v| v)
            .collect();
    }
    insides
}

fn to_instr(hexcolor: &str) -> Instruction {
    // skip the hashmark
    let distance = &hexcolor[1..6];
    let steps = usize::from_str_radix(distance, 16).unwrap();

    let direction = match &hexcolor.chars().nth(6).unwrap() {
        '3' => Dir::U,
        '1' => Dir::D,
        '2' => Dir::L,
        '0' => Dir::R,
        _ => unreachable!(),
    };
    Instruction {
        direction,
        steps,
        color: String::from(""),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    use crate::Dir as D;
    let instructions = parse(input);
    // dbg!(&instructions);
    let origin = (0, 0);
    // maybe have to center on origin? lets try keeping track of just points right now...
    let mut boundary = BTreeSet::new();
    boundary.insert(origin);
    let mut cur = origin;
    instructions.iter().for_each(|i| {
        let d: (i32, i32) = match i.direction {
            D::R => (1, 0),
            D::U => (0, -1),
            D::D => (0, 1),
            D::L => (-1, 0),
        };
        (1..=i.steps as i32).for_each(|step| {
            let next_hole = (cur.0 + step * d.0, cur.1 + step * d.1);
            // dbg!(&next_hole);
            boundary.insert(next_hole);
        });
        cur = (cur.0 + i.steps as i32 * d.0, cur.1 + i.steps as i32 * d.1);
    });

    let insides = fill(&boundary, (1, 1));
    // display(&boundary, &insides);
    Some(boundary.len() as u32 + insides.len() as u32)
}

// yup, part 1 took 14s in release mode, there's no way this will run without a better algorithm
// let's at least parse out the new instructions though
pub fn part_two(input: &str) -> Option<u32> {
    let _instructions = parse(input)
        .iter()
        .map(|i| to_instr(&format!("#{}", &i.color)))
        .collect_vec();
    // dbg!(&instructions);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_conversion() {
        assert_eq!(
            to_instr("#70c710"),
            Instruction {
                direction: Dir::R,
                steps: 461937,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#0dc571"),
            Instruction {
                direction: Dir::D,
                steps: 56407,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#5713f0"),
            Instruction {
                direction: Dir::R,
                steps: 356671,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#d2c081"),
            Instruction {
                direction: Dir::D,
                steps: 863240,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#59c680"),
            Instruction {
                direction: Dir::R,
                steps: 367720,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#411b91"),
            Instruction {
                direction: Dir::D,
                steps: 266681,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#8ceee2"),
            Instruction {
                direction: Dir::L,
                steps: 577262,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#caa173"),
            Instruction {
                direction: Dir::U,
                steps: 829975,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#1b58a2"),
            Instruction {
                direction: Dir::L,
                steps: 112010,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#caa171"),
            Instruction {
                direction: Dir::D,
                steps: 829975,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#7807d2"),
            Instruction {
                direction: Dir::L,
                steps: 491645,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#a77fa3"),
            Instruction {
                direction: Dir::U,
                steps: 686074,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#015232"),
            Instruction {
                direction: Dir::L,
                steps: 5411,
                color: String::from("")
            }
        );
        assert_eq!(
            to_instr("#7a21e3"),
            Instruction {
                direction: Dir::U,
                steps: 500254,
                color: String::from("")
            }
        );
    }
}
