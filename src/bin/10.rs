use std::ops::Add;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy)]
enum Heading {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy)]
struct Traveler {
    steps_taken: u32,
    heading: Heading,
    current_pos: Point,
}

impl Traveler {
    fn new(heading: Heading, pos: Point) -> Self {
        Traveler {
            steps_taken: 1,
            heading,
            current_pos: pos,
        }
    }
    fn move_to(&mut self, delta: Point, heading: Heading) {
        self.steps_taken += 1;
        self.heading = heading;
        self.current_pos = self.current_pos + delta;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point(i32, i32);

// not really Point, but was handy for being able to add current pos and a delta
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn next_step(heading: &Heading, symbol: &char) -> (Heading, Point) {
    match symbol {
        '|' => {
            // N,S
            match heading {
                Heading::N => (Heading::N, Point(0, -1)),
                Heading::S => (Heading::S, Point(0, 1)),
                _ => unreachable!(),
            }
        }
        '-' => {
            // E,W
            match heading {
                Heading::E => (Heading::E, Point(1, 0)),
                Heading::W => (Heading::W, Point(-1, 0)),
                _ => unreachable!(),
            }
        }
        'L' => {
            // N, E
            match heading {
                Heading::W => (Heading::N, Point(0, -1)),
                Heading::S => (Heading::E, Point(1, 0)),
                _ => unreachable!(),
            }
        }
        'J' => {
            // N, W
            match heading {
                Heading::E => (Heading::N, Point(0, -1)),
                Heading::S => (Heading::W, Point(-1, 0)),
                _ => unreachable!(),
            }
        }
        '7' => {
            // S, W
            match heading {
                Heading::E => (Heading::S, Point(0, 1)),
                Heading::N => (Heading::W, Point(-1, 0)),
                _ => unreachable!(),
            }
        }
        'F' => {
            // S, E
            match heading {
                Heading::W => (Heading::S, Point(0, 1)),
                Heading::N => (Heading::E, Point(1, 0)),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Point, Vec<Vec<char>>) {
    let mut starting_point = Point(0, 0);
    let mut map = vec![];
    input.lines().enumerate().for_each(|(ypos, line)| {
        let mut this_row = vec![];
        line.chars().enumerate().for_each(|(xpos, c)| {
            this_row.push(c);
            if c == 'S' {
                starting_point = Point(xpos as i32, ypos as i32)
            }
        });
        map.push(this_row);
    });
    (starting_point, map)
}

// if we find the starting point and navigate from there in both directions, where those meet up should be furthest point
pub fn part_one(input: &str) -> Option<u32> {
    let (starting_pos, map) = parse(input);
    // ugh, we have to look around to see which directions S can go first - there should only be 2
    // and they should be connected
    // x, y must be >= 0
    let (x, y) = (starting_pos.0 as usize, starting_pos.1 as usize);
    // (steps taken, heading, current pos)
    let mut travelers = vec![];

    if y > 0 && (map[y - 1][x] == '|' || map[y - 1][x] == '7' || map[y - 1][x] == 'F') {
        // north path
        travelers.push(Traveler::new(Heading::N, Point(x as i32, (y - 1) as i32)));
    }
    if x > 0 && (map[y][x - 1] == '-' || map[y][x - 1] == 'L' || map[y][x - 1] == 'F') {
        // west path
        travelers.push(Traveler::new(Heading::W, Point((x - 1) as i32, y as i32)));
    }
    if y < map.len() - 1 && (map[y + 1][x] == '|' || map[y + 1][x] == 'L' || map[y + 1][x] == 'J') {
        // south path
        travelers.push(Traveler::new(Heading::S, Point(x as i32, (y + 1) as i32)));
    }
    if x < map[0].len() - 1
        && (map[y][x + 1] == '-' || map[y][x + 1] == 'J' || map[y][x + 1] == '7')
    {
        // east path
        travelers.push(Traveler::new(Heading::E, Point((x + 1) as i32, y as i32)));
    }
    // now just run around the pipes until the two travelers meet at same spot
    // don't need to do any bounds checking since the pipes must be connected
    let mut t1 = travelers[0];
    let mut t2 = travelers[1];
    loop {
        if &map[t1.current_pos.1 as usize][t1.current_pos.0 as usize] == &'S' {
            panic!("loop completed without encountering t2, fail!");
        }
        if t1.current_pos == t2.current_pos {
            // when they intersect, it's the further point
            return Some(t1.steps_taken);
        }
        let t1next = next_step(
            &t1.heading,
            &map[t1.current_pos.1 as usize][t1.current_pos.0 as usize],
        );
        t1.move_to(t1next.1, t1next.0);

        let t2next = next_step(
            &t2.heading,
            &map[t2.current_pos.1 as usize][t2.current_pos.0 as usize],
        );
        t2.move_to(t2next.1, t2next.0);
    }
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
