use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let wins: HashMap<&str, u32> = HashMap::from([("A Y", 8), ("B Z", 9), ("C X", 7)]);
    let draws: HashMap<&str, u32> = HashMap::from([("A X", 4), ("B Y", 5), ("C Z", 6)]);
    let values: HashMap<&str, u32> = HashMap::from([("Y", 2), ("Z", 3), ("X", 1)]);
    let mut score: u32 = 0;
    for line in lines {
        match wins.get(line) {
            Some(win) => score += win,
            None => match draws.get(line) {
                Some(draw) => score += draw,
                None => match values.get(&line[line.len() - 1..]) {
                    Some(value) => score += value,
                    None => panic!("The input was found in no map"),
                },
            },
        }
    }

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

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
