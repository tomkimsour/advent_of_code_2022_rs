use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let wins: HashMap<&str, u32> = HashMap::from([("A Y", 8), ("B Z", 9), ("C X", 7)]);
    let draws: HashMap<&str, u32> = HashMap::from([("A X", 4), ("B Y", 5), ("C Z", 6)]);
    let values: HashMap<&str, u32> = HashMap::from([("A Z", 3), ("B X", 1), ("C Y", 2)]);
    let mut score: u32 = 0;
    for line in lines {
        match wins.get(line) {
            Some(win) => score += win,
            None => match draws.get(line) {
                Some(draw) => score += draw,
                None => match values.get(line) {
                    Some(value) => score += value,
                    None => panic!("The input was found in no map"),
                },
            },
        }
    }

    Some(score)
}

pub fn part_one_2(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| match line {
            "A Y" => 8,
            "B Z" => 9,
            "C X" => 7,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "A Z" => 3,
            "B X" => 1,
            "C Y" => 2,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let lose: HashMap<&str, u32> = HashMap::from([("A", 3), ("B", 1), ("C", 2)]); // this is directly mapped to the losing value
    let draws: HashMap<&str, u32> = HashMap::from([("A", 4), ("B", 5), ("C", 6)]);
    let wins: HashMap<&str, u32> = HashMap::from([("A", 8), ("B", 9), ("C", 7)]);
    let mut score: u32 = 0;
    for line in lines {
        match line.chars().last().unwrap() {
            'X' => match lose.get(&line[..1]) {
                Some(value) => score += value,
                None => panic!("No corresponding value in the losing map"),
            },
            'Y' => match draws.get(&line[..1]) {
                Some(value) => score += value,
                None => panic!("No corresponding value in the draw map"),
            },
            'Z' => match wins.get(&line[..1]) {
                Some(value) => score += value,
                None => panic!("No corresponding value in the winning map"),
            },
            _ => panic!("The last character of a line is neither x y or z"),
        }
    }
    Some(score)
}

pub fn part_two_2(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| match line {
            "A Y" => 4,
            "B Z" => 9,
            "C X" => 2,
            "A X" => 3,
            "B Y" => 5,
            "C Z" => 7,
            "A Z" => 8,
            "B X" => 1,
            "C Y" => 6,
            _ => unreachable!(),
        })
        .sum();
    Some(score)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    // advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_one_2, input);
    // advent_of_code::solve!(2, part_two, input);
    advent_of_code::solve!(2, part_two_2, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_one_2() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one_2(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_part_two_2() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two_2(&input), Some(12));
    }
}
