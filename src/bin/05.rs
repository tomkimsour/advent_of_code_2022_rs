use core::str::Lines;
use regex::Regex;
use std::{collections::LinkedList, iter::Rev};

fn init_vec_stack(mut input: Rev<Lines>) -> Vec<Vec<String>> {
    let first_line = input.next().unwrap();
    let len = first_line.len();
    let mut array: Vec<Vec<String>> = Vec::new();
    for index in (1..len).step_by(4) {
        array.push(vec![first_line.chars().nth(index).unwrap().to_string()]);
    }

    input.for_each(|line| {
        for index in (1..len).step_by(4) {
            // println!("{}",(index-1)/4);
            let current_string = line.chars().nth(index).unwrap().to_string();
            if !current_string.trim().is_empty() {
                array[(index - 1) / 4].push(current_string);
            }
        }
    });
    array
}

pub fn part_one(input: &str) -> Option<String> {
    let mut iter = input.split("\n\n");
    let mut stack_input = iter.next().unwrap().lines().rev();

    // skipping useless number line
    stack_input.next();

    let mut stack_vector = init_vec_stack(stack_input);

    // execute every action
    let order_input = iter.next().unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in order_input.lines() {
        let cap = re.captures(line).unwrap();

        let first = cap[1].parse::<usize>().unwrap();
        let second = cap[2].parse::<usize>().unwrap() - 1;
        let third = cap[3].parse::<usize>().unwrap() - 1;
        for _ in 0..first {
            let moved_crate = stack_vector[second].pop().unwrap();
            stack_vector[third].push(moved_crate);
        }
    }

    // retrieve the top of each stack and put it in a string
    let mut res = "".to_string();
    for mut stack in stack_vector {
        res += &stack.pop().unwrap_or("".to_string());
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut iter = input.split("\n\n");
    let mut stack_input = iter.next().unwrap().lines().rev();

    // skipping useless number line
    stack_input.next();

    let mut stack_vector = init_vec_stack(stack_input);

    // execute every action
    let order_input = iter.next().unwrap();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in order_input.lines() {
        let cap = re.captures(line).unwrap();

        let first = cap[1].parse::<usize>().unwrap();
        let second = cap[2].parse::<usize>().unwrap() - 1;
        let third = cap[3].parse::<usize>().unwrap() - 1;
        let mut intermidiate_stack: LinkedList<String> = LinkedList::new();
        for _ in 0..first {
            let moved_crate = stack_vector[second].pop().unwrap();
            intermidiate_stack.push_front(moved_crate);
        }
        for crate_stack in intermidiate_stack {
            stack_vector[third].push(crate_stack);
        }
    }

    // retrieve the top of each stack and put it in a string
    let mut res = "".to_string();
    for mut stack in stack_vector {
        res += &stack.pop().unwrap_or("".to_string());
    }
    Some(res)
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
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
