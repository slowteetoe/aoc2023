use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Rule {
    LessThan(String, u32, Target),
    GreaterThan(String, u32, Target),
    Otherwise(Target),
}

#[derive(Debug)]
enum Target {
    Accepted,
    Rejected,
    Workflow(String),
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Target::Rejected),
            "A" => Ok(Target::Accepted),
            _ => Ok(Target::Workflow(s.to_owned())),
        }
    }
}

// a<2006:qkq m>2090:A rfg
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = alpha1(input)?;
    // just the "otherwise" rule
    if input.starts_with("}") {
        let target = Target::from_str(name).unwrap();
        return Ok((input, Rule::Otherwise(target)));
    }
    // parse out a full conditional rule
    let (input, condition) = alt((
        char('<').map(|_| Condition::LessThan),
        char('>').map(|_| Condition::GreaterThan),
    ))(input)?;
    let (input, amount) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, target) =
        preceded(char(':'), map(alpha1, |s| Target::from_str(s).unwrap()))(input)?;
    let rule = match condition {
        Condition::GreaterThan => Rule::GreaterThan(name.to_owned(), amount, target),
        Condition::LessThan => Rule::LessThan(name.to_owned(), amount, target),
    };
    Ok((input, rule))
}

//{a<2006:qkq,m>2090:A,rfg}
fn parse_rules(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
    let (input, node_name) = alpha1(input)?;
    dbg!(&node_name);
    Ok((
        input,
        (
            node_name.to_owned(),
            delimited(char('{'), separated_list1(char(','), parse_rule), char('}'))(input)
                .unwrap()
                .1,
        ),
    ))
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_part(input: &str) -> Result<Part> {
    let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let Some(caps) = re.captures(input) else {
        return Err(Error::msg("didn't work"));
    };
    Ok(Part {
        x: caps[1].parse().unwrap(),
        m: caps[2].parse().unwrap(),
        a: caps[3].parse().unwrap(),
        s: caps[4].parse().unwrap(),
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, parts_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| parse_rules(line).unwrap().1)
        .collect_vec();

    let parts = parts_input
        .lines()
        .map(|line| parse_part(line).unwrap())
        .collect_vec();
    dbg!(&rules, &parts);
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
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
