use std::cmp::Reverse;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut max: u32 = 0;
    let mut elf_calories: u32 = 0;
    for line in lines {
        if line.is_empty() {
            if elf_calories > max {
                max = elf_calories;
            }
            elf_calories = 0;
        } else {
            elf_calories = elf_calories + line.parse::<u32>().unwrap();
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut a_max: Vec<u32> = vec![0, 0, 0];
    let mut elf_calories: u32 = 0;
    for line in lines {
        if line.is_empty() {
            a_max.push(elf_calories);
            a_max.sort_by_key(|w| Reverse(*w));
            a_max.pop();
            elf_calories = 0;
        } else {
            elf_calories = elf_calories + line.parse::<u32>().unwrap();
        }
    }
    Some(a_max.into_iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(48000));
    }
}
