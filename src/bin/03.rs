advent_of_code::solution!(3);
use std::ops::Deref;

#[derive(Debug)]
struct Grid(Vec<Vec<char>>);
impl Grid {
    fn is_symbol(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            // println!("ignoring [{}][{}], outside grid", x, y);
            return false;
        }
        let x = x as usize;
        let y = y as usize;

        if x > self[0].len() - 1 || y > self.len() - 1 {
            // println!("ignoring [{}][{}], outside grid", x, y);
            false
        } else {
            let c = self[y][x];
            // println!("grid[{}][{}] = '{}'", x, y, &c);
            c != '.' && !c.is_numeric()
        }
    }
}
impl Deref for Grid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut parts = vec![];
    grid.iter().enumerate().for_each(|(y, grid_line)| {
        let mut adj = false;
        let mut part_num = vec![];
        grid_line.iter().enumerate().for_each(|(x, c)| {
            if !c.is_numeric() || x == grid_line.len() - 1 {
                if x == grid_line.len() - 1 && c.is_numeric() {
                    // AAAAAAARGH edge conditions, literally.
                    part_num.push(*c);
                }

                if !part_num.is_empty() {
                    // print!("finished part num {:?}", part_num);
                    if adj {
                        // println!(", part is valid.");
                        parts.push(part_num.iter().collect::<String>().parse::<u32>().unwrap());
                    } else {
                        // println!(", part is NOT adjacent a symbol");
                    }
                }
                part_num.clear();
                adj = false;
            } else if c.is_numeric() {
                part_num.push(*c);
                let x = x as isize;
                let y = y as isize;
                if !adj {
                    adj = grid.is_symbol(x - 1, y)
                        || grid.is_symbol(x + 1, y)
                        || grid.is_symbol(x, y - 1)
                        || grid.is_symbol(x, y + 1)
                        || grid.is_symbol(x + 1, y + 1)
                        || grid.is_symbol(x + 1, y - 1)
                        || grid.is_symbol(x - 1, y - 1)
                        || grid.is_symbol(x - 1, y + 1)
                }
            }
        })
    });
    // dbg!(&parts);
    Some(parts.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_grid(input: &str) -> Grid {
    Grid(input.lines().map(|line| line.chars().collect()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
