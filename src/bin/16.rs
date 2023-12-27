use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

/// type and whether it's energized or not
#[derive(Debug)]
enum Component {
    RightLeaningMirror,
    LeftLeaningMirror,
    HorizontalSplitter,
    VerticalSplitter,
    EmptySpace,
}

#[derive(Debug, Clone)]
enum Heading {
    R,
    L,
    U,
    D,
}

#[derive(Debug)]
struct Beam {
    /// zero-based position in the layout
    pos: (isize, isize),
    heading: Heading,
}

impl Beam {
    fn next_pos(&self) -> (isize, isize) {
        let (dx, dy) = match self.heading {
            Heading::R => (1, 0),
            Heading::D => (0, 1),
            Heading::L => (-1, 0),
            Heading::U => (0, -1),
        };
        (self.pos.0 + dx, self.pos.1 + dy)
    }
}

fn parse(input: &str) -> Vec<Vec<Component>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Component::EmptySpace,
                    '|' => Component::VerticalSplitter,
                    '-' => Component::HorizontalSplitter,
                    '/' => Component::LeftLeaningMirror,
                    '\\' => Component::RightLeaningMirror,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec()
}

#[allow(dead_code)]
fn display(layout: &Vec<Vec<Component>>, energized: &HashSet<(isize, isize)>) {
    layout.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, component)| {
            if energized.contains(&(x as isize, y as isize)) {
                print!("#");
            } else {
                match component {
                    Component::RightLeaningMirror => print!("\\"),
                    Component::LeftLeaningMirror => print!("/"),
                    Component::HorizontalSplitter => print!("-"),
                    Component::VerticalSplitter => print!("|"),
                    Component::EmptySpace => print!("."),
                }
            }
        });
        println!();
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let layout = parse(input);
    let mut beams = vec![Beam {
        pos: (0, 0),
        heading: Heading::R,
    }];

    let mut energized = HashSet::new();
    energized.insert((0, 0));
    let mut safety = 0;
    loop {
        // hopefully there won't be any cycles
        if beams.is_empty() {
            // println!("All cells energized and done");
            break;
        }
        let mut exiting = vec![];
        let mut entering = vec![];
        beams.iter_mut().for_each(|beam| {
            if beam.pos.0 > (layout[0].len() - 1) as isize
                || beam.pos.0 < 0
                || beam.pos.1 > (layout.len() - 1) as isize
                || beam.pos.1 < 0
            {
                // println!("beam {idx} exited");
                exiting.push(beam.pos);
            } else {
                // mark our spot
                let to_mark = (beam.pos.0, beam.pos.1);
                match (
                    &layout[beam.pos.1 as usize][beam.pos.0 as usize],
                    &beam.heading,
                ) {
                    (Component::HorizontalSplitter, Heading::U)
                    | (Component::HorizontalSplitter, Heading::D) => {
                        // split into two beams, if it's not already off the map, and it hasn't already been energized, since that just
                        // makes a cycle
                        if (beam.pos.0 as usize) < layout.len() - 1 && !energized.contains(&to_mark)
                        {
                            entering.push(Beam {
                                pos: (beam.pos.0 + 1, beam.pos.1),
                                heading: Heading::R,
                            });
                        }
                        // old beam is redirected
                        beam.heading = Heading::L;
                        beam.pos = (beam.pos.0 - 1, beam.pos.1);
                    }
                    (Component::VerticalSplitter, Heading::L)
                    | (Component::VerticalSplitter, Heading::R) => {
                        // split into two beams, if it's not already off the map, and it hasn't already been energized, since that just
                        // makes a cycle
                        if (beam.pos.1 as usize) < layout.len() - 1 && !energized.contains(&to_mark)
                        {
                            entering.push(Beam {
                                pos: (beam.pos.0, beam.pos.1 + 1),
                                heading: Heading::D,
                            });
                        }
                        // old beam is redirected
                        beam.heading = Heading::U;
                        beam.pos = (beam.pos.0, beam.pos.1 - 1);
                    }
                    (Component::LeftLeaningMirror, Heading::R) => {
                        beam.heading = Heading::U;
                        beam.pos = (beam.pos.0, beam.pos.1 - 1);
                    }
                    (Component::LeftLeaningMirror, Heading::U) => {
                        beam.heading = Heading::R;
                        beam.pos = (beam.pos.0 + 1, beam.pos.1);
                    }
                    (Component::LeftLeaningMirror, Heading::D) => {
                        beam.heading = Heading::L;
                        beam.pos = (beam.pos.0 - 1, beam.pos.1);
                    }
                    (Component::LeftLeaningMirror, Heading::L) => {
                        beam.heading = Heading::D;
                        beam.pos = (beam.pos.0, beam.pos.1 + 1);
                    }
                    (Component::RightLeaningMirror, Heading::R) => {
                        beam.heading = Heading::D;
                        beam.pos = (beam.pos.0, beam.pos.1 + 1);
                    }
                    (Component::RightLeaningMirror, Heading::U) => {
                        beam.heading = Heading::L;
                        beam.pos = (beam.pos.0 - 1, beam.pos.1);
                    }
                    (Component::RightLeaningMirror, Heading::D) => {
                        beam.heading = Heading::R;
                        beam.pos = (beam.pos.0 + 1, beam.pos.1);
                    }
                    (Component::RightLeaningMirror, Heading::L) => {
                        beam.heading = Heading::U;
                        beam.pos = (beam.pos.0, beam.pos.1 - 1);
                    }
                    _ => {
                        beam.pos = beam.next_pos();
                    }
                }
                energized.insert(to_mark);
            }
            safety += 1;
        });
        // display(&layout, &beams);
        beams.retain(|x| !exiting.contains(&x.pos));
        beams.append(&mut entering);
        // turns out we don't need this check, since we're eliminating beams spawning multiple times from the splitters

        // if safety % 100 == 0 {
        //     // every few rounds, see if we reached a steady state... will this work?
        //     if last_energized == energized.len() {
        //         println!("Think we reached a steady state, let's quit!");
        //         break;
        //     } else {
        //         last_energized = energized.len();
        //     }
        // }
    }
    // display(&layout, &energized);
    Some(energized.len() as u32)
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
