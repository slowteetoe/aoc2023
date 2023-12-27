use std::collections::BTreeMap;

advent_of_code::solution!(15);

pub fn hash(s: &str) -> u32 {
    s.chars().fold(0u32, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[derive(Debug)]
enum Instruction {
    Remove(String),
    Insert(Lens),
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .replace("\n", "")
        .split(",")
        .into_iter()
        .map(|s| hash(s))
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace("\n", "");
    let instructions: Vec<_> = input
        .split(",")
        .into_iter()
        .map(|s| {
            if s.contains("=") {
                let (label, focal_len) = s.split_once("=").unwrap();
                (
                    hash(&label),
                    Instruction::Insert(Lens {
                        label: label.to_owned(),
                        focal_length: focal_len.parse().unwrap(),
                    }),
                )
            } else {
                let label = s.split_once("-").unwrap().0.to_owned();
                (hash(&label), Instruction::Remove(label))
            }
        })
        .collect();

    let mut b = BTreeMap::new();
    instructions.iter().for_each(|(target, instr)| match instr {
        Instruction::Insert(lens) => {
            let slot = b.entry(target).or_insert(vec![]);
            if let Some(index) = slot
                .iter()
                .position(|boxed_lens: &&Lens| boxed_lens.label == lens.label)
            {
                slot[index] = lens;
            } else {
                slot.push(lens);
            }
        }
        Instruction::Remove(label) => {
            let slot = b.entry(target).or_insert(vec![]);
            if let Some(index) = slot.iter().position(|lens| lens.label == label.as_str()) {
                slot.remove(index);
            }
        }
    });
    let score = b
        .iter()
        .map(|(slot_num, v)| {
            v.iter().enumerate().fold(0, |acc, (pos_in_box, lens)| {
                acc + (**slot_num + 1) * ((pos_in_box + 1) as u32 * lens.focal_length)
            })
        })
        .sum();
    Some(score)
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
        assert_eq!(result, Some(145));
    }
}
