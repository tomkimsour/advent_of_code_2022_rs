#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str;

use std::{collections::LinkedList, str::Chars};

pub fn part_one(input: &str) -> Option<u32> {
    let characters = input.as_bytes().iter();
    // create subroutine
    let mut subroutine = characters.clone().take(3).collect::<LinkedList<&u8>>();
    let mut value_hashmap_tracker = HashMap::new();
    characters.clone().take(3).for_each(|val| {
        *value_hashmap_tracker.entry(val).or_insert(0) +=1;
    });
    for (index,byte) in characters.skip(3).enumerate(){
        subroutine.push_back(&byte);
        *value_hashmap_tracker.entry(&byte).or_insert(0)+= 1;
        if subroutine.clone().iter().map(|key| *value_hashmap_tracker.get(key).unwrap() as u8).sum::<u8>() == 4 {
            return Some(index as u32 + 4);
        } else {
            let value = subroutine.pop_front().unwrap();
            *value_hashmap_tracker.entry(value).or_insert(1) -= 1;
        }
    };
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let characters = input.as_bytes().iter();
    // create subroutine
    let mut subroutine = characters.clone().take(13).collect::<LinkedList<&u8>>();
    let mut value_hashmap_tracker = HashMap::new();
    characters.clone().take(13).for_each(|val| {
        *value_hashmap_tracker.entry(val).or_insert(0) +=1;
    });
    println!("-----init-----");
        println!("[13] hash : {:?}",value_hashmap_tracker);
        println!("[13] subroutine : {:?}",subroutine);
        println!("[13] condition : {:?}",subroutine.clone().iter().map(|key| *value_hashmap_tracker.get(key).unwrap() as u8).sum::<u8>());
println!();
    println!("-----boucle-----");
    for (index,byte) in characters.skip(13).enumerate(){
        subroutine.push_back(&byte);
        *value_hashmap_tracker.entry(&byte).or_insert(0)+= 1;
        println!("[{}] hash : {:?}",index+14,value_hashmap_tracker);
        println!("[{}] subroutine : {:?}",index+14,subroutine);
        println!("[{}] condition : {:?}",index+14,subroutine.clone().iter().map(|key| *value_hashmap_tracker.get(key).unwrap() as u8).sum::<u8>());
        if subroutine.clone().iter().map(|key| *value_hashmap_tracker.get(key).unwrap() as u8).sum::<u8>() == 14 {
            return Some(index as u32 + 14);
        } else {
            let value = subroutine.pop_front().unwrap();
            *value_hashmap_tracker.entry(value).or_insert(1) -= 1;
        }
    println!();
    };
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
        let input = advent_of_code::read_file("examples", 61);
        assert_eq!(part_one(&input), Some(5));
        let input = advent_of_code::read_file("examples", 62);
        assert_eq!(part_one(&input), Some(6));
        let input = advent_of_code::read_file("examples", 63);
        assert_eq!(part_one(&input), Some(10));
        let input = advent_of_code::read_file("examples", 64);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
        let input = advent_of_code::read_file("examples", 61);
        assert_eq!(part_one(&input), Some(23));
        let input = advent_of_code::read_file("examples", 62);
        assert_eq!(part_one(&input), Some(23));
        let input = advent_of_code::read_file("examples", 63);
        assert_eq!(part_one(&input), Some(29));
        let input = advent_of_code::read_file("examples", 64);
        assert_eq!(part_one(&input), Some(26));
    }
}
