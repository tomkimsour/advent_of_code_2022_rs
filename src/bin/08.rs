#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{str::Lines, fmt::write};

struct Tree {
    height : u8,
    visible : bool,
    visibility_right:u32,
    visibility_left:u32,
    visibility_top:u32,
    visibility_bottom:u32,
}

impl std::fmt::Display for Tree{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.visible {
            write!(f, "\x1b[33m{}\x1b[0m",self.height)
        } else {
            write!(f, "{}",self.height)
            // write!(f, "9")
        }
    }
}
struct Forest {
    trees : Vec<Vec<Tree>>,
    visible_tree:u32,
}

impl Forest {
    fn print(&self) {
        for row in &self.trees {
            for col in row {
                print!("{}",col);
            }
            println!();
        }
    }
}

fn init_matrix(lines:Lines) -> Forest { 
    let mut vec = Vec::new();
    for line in lines {
        let col = line.chars().map(|c| {Tree{height : c.to_digit(10).unwrap() as u8, visible: false, visibility_bottom: 0, visibility_left: 0, visibility_right:0,visibility_top:0}}).collect::<Vec<Tree>>();
        vec.push(col);
    }
    let row_len = vec.len();
    let col_len = vec[0].len();
    for index in 0..row_len {
        vec[index][0].visible = true;   
        vec[index][col_len-1].visible = true;   
    }
    for index in 0..col_len{
        vec[0][index].visible = true;   
        vec[row_len-1][index].visible = true;   
    }
    Forest {trees: vec , visible_tree: 0}
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut forest_matrix = init_matrix(lines); 
    let row_len = forest_matrix.trees.len();
    let col_len = forest_matrix.trees[0].len();
    forest_matrix.visible_tree = (2 * row_len as u32)+ (2 * col_len as u32) - 4;
    // top
    for col in 1..col_len-1{
        let mut tallest = forest_matrix.trees[0][col].height;
        for row in 1..row_len{
            if forest_matrix.trees[row][col].height > tallest {
                if !forest_matrix.trees[row][col].visible {
                    forest_matrix.visible_tree +=1;
                    forest_matrix.trees[row][col].visible = true;
                }
                tallest = forest_matrix.trees[row][col].height;
            }
        }
    }
    // left
    for row in 1..row_len-1{
        let mut tallest = forest_matrix.trees[row][0].height;
        for col in 1..col_len{
            if forest_matrix.trees[row][col].height > tallest {
                if !forest_matrix.trees[row][col].visible {
                    forest_matrix.visible_tree +=1;
                    forest_matrix.trees[row][col].visible = true;
                }
                tallest = forest_matrix.trees[row][col].height;
            }
        }
    }
    // right
    for row in 1..row_len-1{
        let mut tallest = forest_matrix.trees[row][col_len-1].height;
        for col in (1..col_len-1).rev(){
            if forest_matrix.trees[row][col].height > tallest {
                if !forest_matrix.trees[row][col].visible {
                    forest_matrix.visible_tree +=1;
                    forest_matrix.trees[row][col].visible = true;
                }
                tallest = forest_matrix.trees[row][col].height;
            }
        }
    }
    // bottom
    for col in 1..col_len-1{
        let mut tallest = forest_matrix.trees[row_len-1][col].height;
        for row in (1..row_len-1).rev(){
            if forest_matrix.trees[row][col].height > tallest {
                if !forest_matrix.trees[row][col].visible {
                    forest_matrix.visible_tree +=1;
                    forest_matrix.trees[row][col].visible = true;
                }
                tallest = forest_matrix.trees[row][col].height;
            }
        }
    }
    // forest_matrix.print();
    Some(forest_matrix.visible_tree)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut forest_matrix = init_matrix(lines); 
    let row_len = forest_matrix.trees.len();
    let col_len = forest_matrix.trees[0].len();
    forest_matrix.visible_tree = 0;
    for i in 1..row_len{
        for j in 1..col_len{
            let tree_height = forest_matrix.trees[i][j].height;
            //top
            for row in (0..i).rev(){
                    forest_matrix.trees[i][j].visibility_top +=1;
                if forest_matrix.trees[row][j].height < tree_height {
                    if !forest_matrix.trees[row][j].visible {
                        forest_matrix.trees[row][j].visible = true;
                    }
                } else{
                    break;
                }
            }
            //left
            for col in (0..j).rev(){
                    forest_matrix.trees[i][j].visibility_left+=1;
                if forest_matrix.trees[i][col].height < tree_height {
                    if !forest_matrix.trees[i][col].visible {
                        forest_matrix.trees[i][col].visible = true;
                    } 
                } else {
                    break;
                }
            }
            //right
            for col in j+1..col_len{
                    forest_matrix.trees[i][j].visibility_right+=1;
                if forest_matrix.trees[i][col].height < tree_height{
                    if !forest_matrix.trees[i][col].visible {
                        forest_matrix.trees[i][col].visible = true;
                    } 
                } else {
                    break;
                }
            }
            //botoom
            for row in i+1..row_len{
                    forest_matrix.trees[i][j].visibility_bottom+=1;
                if forest_matrix.trees[row][j].height < tree_height{
                    if !forest_matrix.trees[row][j].visible {
                        forest_matrix.trees[row][j].visible = true;
                    }
                } else {
                    break;
                }
            }

            let res: u32 = forest_matrix.trees[i][j].visibility_bottom *forest_matrix.trees[i][j].visibility_right *forest_matrix.trees[i][j].visibility_left * forest_matrix.trees[i][j].visibility_top;

            if forest_matrix.visible_tree < res{
                forest_matrix.visible_tree = res;
            }
        }
    }
    Some(forest_matrix.visible_tree)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
