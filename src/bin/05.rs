#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use regex::Regex;

pub fn part_one(input: &str) -> Option<&str> {
    let mut iter = input.split("\n\n");
    let stack_input = iter.next().unwrap().lines();
    let len = stack_input.last().unwrap();
    println!("{:}",len);

    // for line in stack_input{

    // }

    // let order_input = iter.next().unwrap();
    // let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    // for line in order_input.lines() {
    //     let cap = re.captures(line).unwrap();

    //     let first = cap[1].parse::<i32>().unwrap();
    //     let second = cap[2].parse::<i32>().unwrap();
    //     let third = cap[3].parse::<i32>().unwrap();
    // }

    Some("".clone())
}

pub fn part_two(input: &str) -> Option<&str> {
    let lines = input.lines();
    Some("")
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ"));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(""));
    }
}
