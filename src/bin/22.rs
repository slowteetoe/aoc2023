use glam::IVec3;
use itertools::Itertools;
use nom::multi::separated_list1;

advent_of_code::solution!(22);

#[derive(Debug)]
struct Brick {
    front_face: IVec3,
    back_face: IVec3,
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (f, b) = line.split_once("~").unwrap();
            let fs: Vec<_> = f
                .splitn(3, ',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            let bs: Vec<_> = b
                .splitn(3, ",")
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            Brick {
                front_face: IVec3 {
                    x: fs[0],
                    y: fs[1],
                    z: fs[2],
                },
                back_face: IVec3 {
                    x: bs[0],
                    y: bs[1],
                    z: bs[2],
                },
            }
        })
        .collect_vec()
}

// wth is this brick coordinate system?
pub fn part_one(input: &str) -> Option<u32> {
    let bricks = parse(input);
    dbg!(bricks);
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
