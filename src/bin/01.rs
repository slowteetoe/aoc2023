advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
                format!("{}{}", digits[0], digits[digits.len() - 1])
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let t = input
        .lines()
        .map(|line| {
            // build up a new vec and just use the solution from part 1
            let chars: Vec<_> = line.chars().collect();
            let mut translated = vec![];
            let bound = chars.len();
            let mut i = 0usize;
            while i < bound {
                if chars[i].is_numeric() {
                    translated.push(chars[i]);
                    i += 1;
                    continue;
                }
                match chars[i] {
                    'o' => {
                        if i + 2 < bound && chars[i + 1] == 'n' && chars[i + 2] == 'e' {
                            translated.push('1');
                            i += 2;
                            continue;
                        }
                    }
                    't' => {
                        if i + 2 < bound && chars[i + 1] == 'w' && chars[i + 2] == 'o' {
                            translated.push('2');
                            i += 2;
                            continue;
                        } else if i + 4 < bound
                            && chars[i + 1] == 'h'
                            && chars[i + 2] == 'r'
                            && chars[i + 3] == 'e'
                            && chars[i + 4] == 'e'
                        {
                            translated.push('3');
                            i += 4;
                            continue;
                        }
                    }
                    'f' => {
                        if i + 3 < bound
                            && chars[i + 1] == 'o'
                            && chars[i + 2] == 'u'
                            && chars[i + 3] == 'r'
                        {
                            translated.push('4');
                            i += 3;
                            continue;
                        } else if i + 3 < bound
                            && chars[i + 1] == 'i'
                            && chars[i + 2] == 'v'
                            && chars[i + 3] == 'e'
                        {
                            translated.push('5');
                            i += 3;
                            continue;
                        }
                    }
                    's' => {
                        if i + 2 < bound && chars[i + 1] == 'i' && chars[i + 2] == 'x' {
                            translated.push('6');
                            i += 2;
                            continue;
                        } else if i + 4 < bound
                            && chars[i + 1] == 'e'
                            && chars[i + 2] == 'v'
                            && chars[i + 3] == 'e'
                            && chars[i + 4] == 'n'
                        {
                            translated.push('7');
                            i += 4;
                            continue;
                        }
                    }
                    'e' => {
                        if i + 4 < bound
                            && chars[i + 1] == 'i'
                            && chars[i + 2] == 'g'
                            && chars[i + 3] == 'h'
                            && chars[i + 4] == 't'
                        {
                            translated.push('8');
                            i += 4;
                            continue;
                        }
                    }
                    'n' => {
                        if i + 3 < bound
                            && chars[i + 1] == 'i'
                            && chars[i + 2] == 'n'
                            && chars[i + 3] == 'e'
                        {
                            translated.push('9');
                            i += 3;
                            continue;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            translated
        })
        .map(|digits| {
            format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    // dbg!(&t);
    Some(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, Some(281));
    }
}
