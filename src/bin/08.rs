use num::integer::lcm;
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

pub fn part_one(input: &str) -> Option<u64> {
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

// Detect cycles for each of the starting nodes, then figure out least-common-multiple of all of them
pub fn part_two(input: &str) -> Option<u128> {
    let (directions, nodes) = parse(input);
    let starting_nodes: Vec<_> = nodes
        .keys()
        .cloned()
        .filter(|n| n.ends_with("A"))
        // .take(2)
        .collect();
    let mut cycles = vec![];

    let direction_loop_size = directions.len();
    println!("directions repeat every {:?} steps", direction_loop_size);
    starting_nodes.iter().for_each(|node| {
        let mut directions = directions.iter().enumerate().cycle();
        let mut steps = 0;
        let orig = node;
        let mut curr = node;
        let mut first = None;
        // print!("{:?} ", curr);
        let mut cycle_pos = None;
        loop {
            let (dir_pos, dir) = directions.next().unwrap();
            let target = &nodes.get(curr).unwrap();
            curr = match dir {
                Dir::L => &target.0,
                Dir::R => &target.1,
            };
            // print!(" -({:?})-> {:?}", &dir, &curr);

            if first.is_none() {
                first = Some(curr);
            } else {
                steps += 1;
                if curr == first.unwrap() {
                    // FIXME it isn't enough to be back at the original node, we also have to be at the same place in the directions loop?
                    if cycle_pos.is_none() {
                        // println!("found first cycle for {orig}, {steps} steps, but pos in dir vec is {}", dir_pos);
                        cycle_pos = Some(dir_pos);
                    } else {
                        // println!("found cycle for {orig}, {steps} steps, but pos in dir vec is {} (adjusted: {})", dir_pos, (dir_pos % direction_loop_size));
                        if dir_pos % direction_loop_size == cycle_pos.unwrap() {
                            println!(
                                "Success? {:?} {:?} after {:?} steps",
                                cycle_pos.unwrap(),
                                dir_pos,
                                steps
                            );
                            cycles.push(steps);
                            break;
                        }
                    }
                }
            }
        }
    });

    dbg!(&cycles);

    // u128 max is: 340282366920938463463374607431768211455
    // and coming up with: 1858646397880 so well within bounds
    // so maybe something wrong with cycle detection? (correct answer is 14265111103729)
    Some(cycles.into_iter().fold(1, |acc, n| lcm(acc, n)))
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
