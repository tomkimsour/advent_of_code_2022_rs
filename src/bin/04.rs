use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+)(?:,|$)").unwrap();

    let sum = input.lines().map(|line|{
        // X-Y, A-B -> 0-2,4-6
        // if X <= A and B <= Y
        // else A <= X and Y <= B
       if (x.clone().ge(a.clone())) && (b.clone().ge(y.clone())) {
        1
       } else if (a.ge(x)) && (y.ge(b)){
        1
       }else {
        0
       }
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
