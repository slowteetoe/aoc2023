use itertools::Itertools;

advent_of_code::solution!(11);

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect_vec()
}

// now that I look at this, it probably would have been easier to keep a string, or maybe convert to a bitmap of sorts
pub fn expand(image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded = vec![];
    image.iter().for_each(|row| {
        if row.iter().filter(|c| **c != '.').count() == 0 {
            expanded.push(row.clone());
        }
        expanded.push(row.clone());
    });
    let mut delta = 0; // going to have to keep track of how many cols we added
    (0..image[0].len()).for_each(|col| {
        let mut row = 0;
        loop {
            if row == image.len() {
                // this row needs to be duplicated
                (0..expanded.len()).for_each(|row| {
                    expanded[row].insert(col + delta, '.');
                });
                delta += 1;
                break;
            } else if image[row][col] == '#' {
                // no need to expand this col
                break;
            }
            row += 1;
        }
    });
    expanded
}

fn points_from(image: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    image
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(y, ch)| {
                    if *ch == '#' {
                        Some((x.clone(), y))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let image = parse(input);
    let expanded = expand(image);
    // dbg!(&expanded);
    let points = points_from(&expanded);
    let n = points.len();
    println!(
        "we will have to look at {} unique pairs of points",
        n * (n - 1) / 2
    );
    let unique_points = points.iter().combinations(2).collect_vec();
    unique_points.iter().for_each(|pt| {
        println!(
            "need to calculate  distance between {:?} and {:?}",
            pt[0], pt[1]
        );
    });
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
