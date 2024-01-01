use std::{collections::BTreeMap, str::FromStr};

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

#[derive(Debug, PartialEq, Clone)]
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
fn parse_ruleset(input: &str) -> IResult<&str, Rule> {
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
    Ok((
        input,
        (
            node_name.to_owned(),
            delimited(
                char('{'),
                separated_list1(char(','), parse_ruleset),
                char('}'),
            )(input)
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

fn determine_applicable_rule(
    part: &Part,
    workflow_name: &str,
    rules: &BTreeMap<String, Vec<Rule>>,
) -> Target {
    let applicable_rules = rules.get(workflow_name).unwrap();
    applicable_rules
        .iter()
        .filter_map(|r| match r {
            Rule::LessThan(field, amount, workflow) => match field.as_str() {
                "x" => {
                    if part.x < *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "m" => {
                    if part.m < *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "a" => {
                    if part.a < *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "s" => {
                    if part.s < *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            },
            Rule::GreaterThan(field, amount, workflow) => match field.as_str() {
                "x" => {
                    if part.x > *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "m" => {
                    if part.m > *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "a" => {
                    if part.a > *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                "s" => {
                    if part.s > *amount {
                        Some(workflow.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Rule::Otherwise(workflow) => Some(workflow.clone()),
        })
        .nth(0)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, parts_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| parse_rules(line).unwrap().1)
        .collect::<BTreeMap<String, Vec<Rule>>>();

    let parts = parts_input
        .lines()
        .map(|line| parse_part(line).unwrap())
        .collect_vec();
    // dbg!(&rules, &parts);

    let mut sum = 0;
    parts.iter().for_each(|part| {
        let mut ultimate_dest = determine_applicable_rule(part, "in", &rules);
        // apply rules for 'in'
        loop {
            if ultimate_dest == Target::Rejected {
                break;
            } else if ultimate_dest == Target::Accepted {
                sum += part.x + part.m + part.a + part.s;
                break;
            } else {
                // run the next rule
                let next_dest = if let Target::Workflow(workflow_name) = ultimate_dest {
                    workflow_name
                } else {
                    unreachable!()
                };
                ultimate_dest = determine_applicable_rule(part, &next_dest, &rules);
            }
        }
    });
    Some(sum)
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
