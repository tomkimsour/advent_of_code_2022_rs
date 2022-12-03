use std::collections::HashSet;

fn get_common_item_value(line: &str) -> u32 {
    let len = line.len() / 2;
    let first_half = line[..len].chars();
    let second_half = line[len..].chars();

    // create a set with the first string and iterate with the second while looking in the hashmap O(n+m)
    let character_set: HashSet<char> = first_half.collect();
    for character in second_half.into_iter() {
        if character_set.contains(&character) {
            if character.is_lowercase() {
                return character as u32 - 96;
            } else {
                return character as u32 - 38;
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines {
        sum += get_common_item_value(line);
    }
    Some(sum)
}

fn get_badge_priority(lines1: &str, lines2: &str, lines3: &str) -> u32 {
    let mut character_set1: HashSet<char> = lines1.chars().collect();
    let character_set2: HashSet<char> = lines2.chars().collect();
    let character_set3: HashSet<char> = lines3.chars().collect();
    // let inter:HashSet<char> = character_set1.intersection(&character_set2).collect();
    character_set1.retain(|e| character_set2.contains(e));
    character_set1.retain(|e| character_set3.contains(e));
    let common_char = character_set1.into_iter().collect::<Vec<char>>()[0];
    if common_char.is_lowercase() {
        common_char as u32 - 96
    } else {
        common_char as u32 - 38
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for index in (0..lines.len() - 2).step_by(3) {
        sum += get_badge_priority(lines[index], lines[index + 1], lines[index + 2]);
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
