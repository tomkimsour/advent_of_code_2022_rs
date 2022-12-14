#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{str::Lines, collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use itertools::Itertools;
use nalgebra::{Point2};

struct Node {
    value : u8,
    visited : bool,
    neighbours : HashMap<u32,Vec<Node>>,
}

impl Node {
    fn new() -> Node{
        Node  {value: (0), visited: (false), neighbours: (HashMap::new())}
    }

    fn from(value:u8) -> Node{
        Node { value: (value), visited: (false), neighbours: (HashMap::new())}
    }

    fn add_neighbour(&self,row:usize, col:usize, node_matrix:Vec<Vec<Node>>){
        // check top
        // check left
        // check right 
        // check bottom 
    }
}

struct HeightMap {
    map : HashMap<u32,Vec<Node>>,
    start : Point2<u32>,
    end : Point2<u32>,
}

impl HeightMap {
    fn new(lines: Lines) -> HeightMap{

        // create a matrix of nodes
        let mut node_matrix= Vec::new();
        let mut start = Node::new();
        let mut end = Node::new();
        for (i,line) in lines.enumerate() {
            let mut col:Vec<Node> = Vec::new();
            for (j,character) in line.bytes().enumerate() {
                // check if start
                if character == 83 {
                    start = Node::from(b'a');
                    col.push(start);
                } else if character == 69 {
                    end = Node::from(b'z');
                    col.push(end);
                } else {
                    col.push(Node::from(character));
                } 
            }
            node_matrix.push(col);
        }

        // fill node neighbours and create set of unvisited node
        let mut pending = HashSet::new();
        for row in 0..node_matrix.len() {
            for col in 0..node_matrix[0].len() {
                node_matrix[row][col].add_neighbour(row,col,node_matrix);
                pending.insert(&node_matrix[row][col]);
            }
        }

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
