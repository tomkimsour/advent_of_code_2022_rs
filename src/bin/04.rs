use itertools::Itertools;
use regex::Regex;
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+)(?:,|$)").unwrap();

    let mut sum = 0;
    input.lines().for_each(|line| {
        // X-Y, A-B -> 0-2,4-6
        // if X <= A and B <= Y
        // else A <= X and Y <= B
        let mut cap_iter = re.captures_iter(line);
        let mut cap = cap_iter.next().unwrap();
        let range1 = RangeInclusive::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        cap = cap_iter.next().unwrap();
        let range2 = RangeInclusive::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );

        if range1.contains(&range2.start()) && range1.contains(&range2.end()) {
            sum += 1
        } else if range2.contains(&range1.start()) && range2.contains(&range1.end()) {
            sum += 1
        } else {
            sum += 0
        }
    });
    Some(sum)
}

fn parse_line_regex(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let re = Regex::new(r"(\d+)-(\d+)(?:,|$)").unwrap();
    re.captures_iter(line)
        .map(|cap| cap[1].parse::<i32>().unwrap()..=cap[2].parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part_one_2(input: &str) -> Option<u32> {
    // let mut sum = 0;
    let sum = input
        .lines()
        .map(|line| {
            // X-Y, A-B -> 0-2,4-6
            // if X <= A and B <= Y
            // else A <= X and Y <= B
            let (range1, range2) = parse_line_regex(line);

            if range1.contains(&range2.start()) && range1.contains(&range2.end()) {
                1
            } else if range2.contains(&range1.start()) && range2.contains(&range1.end()) {
                1
            } else {
                0
            }
        })
        .count();
    Some(sum as u32)
}

fn parse_line_split(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    line.split_once(",").unwrap().map(|range| {
        let (start, end) = range
            .split_once("-")
            .unwrap()
            .map(|number| u32::from_str_radix(number, 10).unwrap());
        start..=end
    })
}

pub fn part_one_3(input: &str) -> Option<u32> {
    // let mut sum = 0;
    let sum = input
        .lines()
        .map(|line| {
            // X-Y, A-B -> 0-2,4-6
            // if X <= A and B <= Y
            // else A <= X and Y <= B
            let (range1, range2) = parse_line_split(line);

            if range1.contains(&range2.start()) && range1.contains(&range2.end()) {
                1
            } else if range2.contains(&range1.start()) && range2.contains(&range1.end()) {
                1
            } else {
                0
            }
        })
        .count();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let re = Regex::new(r"(\d+)-(\d+)(?:,|$)").unwrap();
    let mut sum = 0;
    for line in lines {
        let mut cap_iter = re.captures_iter(line);
        let mut cap = cap_iter.next().unwrap();
        let range1 = RangeInclusive::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        cap = cap_iter.next().unwrap();
        let range2 = RangeInclusive::new(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        if range1.contains(range2.start()) || range1.contains(range2.end()) {
            sum += 1
        } else if range2.contains(range1.start()) || range2.contains(range1.end()) {
            sum += 1
        }
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    // advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_one_2, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one_2(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
