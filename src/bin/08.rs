use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::map,
    error,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct InvalidDirection(String);

impl TryFrom<&str> for Dir {
    type Error = InvalidDirection;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L" => Ok(Dir::L),
            "R" => Ok(Dir::R),
            _ => Err(InvalidDirection(value.to_owned())),
        }
    }
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    preceded(
        tag("\n\n"), // get rid of extra line
        separated_list1(
            // AAA = (BBB, CCC)\nBBB = (DDD, EEE)\n...
            tag("\n"),
            map(
                tuple((
                    alphanumeric1::<&str, error::Error<_>>, // node
                    tag(" = ("),
                    alphanumeric1, // left
                    tag(", "),
                    alphanumeric1, // right
                    tag(")"),
                )),
                |t| (t.0, t.2, t.4), // (AAA, BBB, CCC)
            ),
        ),
    )(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Dir>> {
    many1(map(alt((tag("L"), tag("R"))), |c| {
        Dir::try_from(c).unwrap()
    }))(input)
}

fn parse(input: &str) -> (Vec<Dir>, BTreeMap<&str, (&str, &str)>) {
    let (input, directions) = parse_directions(input).unwrap();
    let (_, nodes) = parse_nodes(input).unwrap();
    let nodes = nodes
        .iter()
        .fold(BTreeMap::<&str, _>::new(), |mut acc, this_node| {
            acc.insert(this_node.0, (this_node.1, this_node.2));
            acc
        });
    (directions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);
    let mut curr = "AAA";
    let mut steps = 0;
    let mut directions = directions.iter().cycle();
    loop {
        steps += 1;
        let dir = directions.next().unwrap();
        curr = match dir {
            Dir::L => nodes.get(curr).unwrap().0,
            Dir::R => nodes.get(curr).unwrap().1,
        };
        // println!("went {:?} to arrive at {curr}", dir);
        if curr == "ZZZ" {
            break;
        }
    }
    Some(steps)
}

// FML - of course AoC tailors the input so that part two is too slow and we have to come up with a better algorithm...
// Guessing this involves detecting a cycle for each of the starting points, and then figuring out where they intersect
// e.g. from example sp1 hits an end node every 2 steps, sp2 hits an end node every 3 steps, so 2*3=6
pub fn part_two(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);
    let mut curr: Vec<_> = nodes.keys().cloned().filter(|n| n.ends_with("A")).collect();
    let total_starting_points = curr.len();
    let mut directions = directions.iter().cycle();
    let mut steps = 0;
    loop {
        steps += 1;
        let dir = directions.next().unwrap();
        let mut matches = 0;
        curr = curr
            .iter()
            .map(|node| match dir {
                Dir::L => nodes.get(node).unwrap().0,
                Dir::R => nodes.get(node).unwrap().1,
            })
            .inspect(|n| {
                if n.ends_with("Z") {
                    matches += 1;
                }
            })
            .collect();
        // println!("went {:?} to arrive at {curr}", dir);
        if matches == total_starting_points {
            break;
        }
    }
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
