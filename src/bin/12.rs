#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{str::Lines, collections::{HashMap, HashSet, BinaryHeap}, hash::{Hash, Hasher}};

use itertools::Itertools;
use nalgebra::{Point2};
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
struct Node {
    value : u8,
    visited : bool,
    cost: u32,
}

impl Node {
    fn new() -> Node{
        Node  {value: (0), visited: (false), cost: (0)}
    }

    fn from(value:u8) -> Node{
        Node { value: (value), visited: (false), cost: (0)}
    }
}


#[derive(Debug)]
struct HeightMap {
    adjacency_list: HashMap<u32,Vec<Node>>,
    start : Node,
    end : Node,
}

impl HeightMap {

    fn add_neighbour(row:usize, col:usize, node_matrix:&Vec<Vec<Node>>) -> Vec<Node>{
        let mut neighbours: Vec<Node> = Vec::new();
        // check top
        if row > 0{
            let top_diff = node_matrix[row][col].value as i8 - node_matrix[row-1][col].value as i8;
            if  top_diff.abs() <= 1 {
                neighbours.push(node_matrix[row-1][col].clone());
            }
        }

        // check left
        if col > 0 {
            let left_diff = node_matrix[row][col].value as i8 - node_matrix[row][col-1].value as i8;
            if left_diff.abs() <= 1 {
                neighbours.push(node_matrix[row][col-1].clone());
            }
        }

        // check right 
        if col < node_matrix[0].len() -1 {
            let right_diff = node_matrix[row][col].value as i8 - node_matrix[row][col+1].value as i8;
            if right_diff.abs() <= 1 {
                neighbours.push(node_matrix[row][col+1].clone());
            }
        }

        // check bottom 
        if row < node_matrix.len() -1 {
            let bottom_diff = node_matrix[row][col].value as i8 - node_matrix[row+1][col].value as i8;
            if bottom_diff.abs() <= 1 {
                neighbours.push(node_matrix[row+1][col].clone());
            }
        }

        neighbours
    }

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
        let mut adjacency_list: HashMap<u32,Vec<Node>> = HashMap::new();
        let row_len = node_matrix.len();
        let col_len= node_matrix[0].len();
        for row in 0..row_len {
            for col in 0..col_len {
                let adjacent_nodes =  HeightMap::add_neighbour(row,col,&node_matrix);
                adjacency_list.insert(row as u32 * node_matrix[0].len() as u32 + col as u32 +1, adjacent_nodes);
            }
        }

        let height_map: HeightMap = HeightMap { adjacency_list: (adjacency_list), start: (start), end: (end) };
        height_map
    }
}

fn dijkstra(adjacency_list: HashMap<u32,Vec<Node>>, start: Node, end: Node)-> u32{
    // Create a set of node that are unvisited
    let mut unvisited  = HashSet::new();
    for (key, value) in adjacency_list.iter() {
        for node in value {
            unvisited.insert((key,node.clone()));
        }
    }

    // create priority queue out of the adjacency list
    let mut dist: Vec<_> = (0..unvisited.len()).map(|_| usize::MAX).collect();
    dist[start.value as usize] = 0;

    let mut heap = BinaryHeap::from(dist);
    // while unvisited is not empty
    for i in 0..unvisited.len() {
        let v = heap.pop().unwrap();
        // for each neighbour of v
        for neighbour in adjacency_list.get(&(v as u32)).unwrap(){
            // if not visited and distance is smaller than the current value
            if !neighbour.visited && neighbour.value as usize < v.value + 1 {

            }
        }
    }
    
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let height_map = HeightMap::new(lines);
    println!("{:?}",height_map.adjacency_list);
    println!("{:?}",height_map.start);
    println!("{:?}",height_map.end);
    let min_path = dijkstra(height_map.adjacency_list, height_map.start, height_map.end);
    
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
