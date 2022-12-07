#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


#[derive(Clone)]
struct NTree {
    parent : Option<Box<NTree>>,
    value : u32,
    children : Option<Vec<NTree>>,
    root : bool,
}

impl NTree {
    fn new(parent:Option<Box<NTree>>,value:u32,root:bool) ->NTree {
        NTree { parent: (parent), value: (value), children: (None), root: (root) }
    }    
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    for line in lines {
        // command
        let mut split_line = line.split(" ");
        let first_string = split_line.next();
        match first_string {
            Some("$") => {println!("Found $")},
            Some("dir") => {println!("Found dir")},
            _ => println!("{}",first_string.unwrap().parse::<u32>().unwrap()),
        };
        // if split_line. ==36 {
        //     line.chars().next();
        //     println!("{}",line.as_str());
        //     // cd change node pointer 
        //     // ls 
        // }
        // // file or dir
        // else {
        //     println!("this is a file or a dir");
        // }
    }
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(0));
    }
}
