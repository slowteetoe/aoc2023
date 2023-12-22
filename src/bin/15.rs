advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .replace("\n", "")
        .split(",")
        .into_iter()
        .map(|s| {
            (
                s,
                s.chars().fold(0u32, |mut acc, c| {
                    acc += c as u32;
                    acc = acc * 17;
                    acc = acc % 256;
                    acc
                }),
            )
        })
        // .inspect(|v| {
        //     dbg!(&v);
        // })
        .map(|(_, val)| val)
        .sum();
    Some(answer)
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
