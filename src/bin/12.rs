#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{str::Lines, collections::HashMap};

use itertools::Itertools;
use nalgebra::{Point2};

struct Node {
    value : u8,
    visited : bool,
    neighbor : HashMap<u8,Vec<Node>>,
}

impl Node {
    fn new() -> Node{
        Node { value: (0), visited: (false), neighbor: (HashMap::new()) }
    }
}

struct HeightMap {
    map : Vec<Vec<u8>>,
    start : Point2<u32>,
    end : Point2<u32>,
}

impl HeightMap {
    fn new(lines: Lines) -> HeightMap{
        let mut vec = Vec::new();
        let mut start : Point2<u32> = Point2::new(0, 0);
        let mut end : Point2<u32> = Point2::new(0, 0);
        for (i,line) in lines.enumerate() {
            let mut col:Vec<u8> = Vec::new();
            for (j,character) in line.bytes().enumerate() {
                // check if start
                if character == 83 {
                    start = Point2::new(i as u32, j as u32) ;
                } else if character == 69 {
                    end = Point2::new(i as u32, j as u32) ;
                }
                col.push(character);
            }
            vec.push(col);
        }

        // turn the matrix into a graph



        let height_map: HeightMap = HeightMap { map: (vec), start: (start), end: (end) };
        height_map
    }
}

fn dijkstra()-> u32{
    // Create a set of node that are unvisited

    // 
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let height_map = HeightMap::new(lines);
    for row in height_map.map {
        for col in row {
            print!("{} ",col);
        }
        println!();
    }
    println!("{}",height_map.start);
    println!("{}",height_map.end);
    
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(0));
    }
}
