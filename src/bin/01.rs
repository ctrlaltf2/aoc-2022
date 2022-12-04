use std::cmp;
use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max: u32 = 0;

    for group in input.split("\n\n") {
        let mut calories_this_group: u32 = 0;
        for line in group.lines() {
            let calories = line.parse::<u32>().unwrap();
            calories_this_group += calories;
            max = cmp::max(max, calories_this_group);
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap = BinaryHeap::new();

    for group in input.split("\n\n") {
        let mut calories_this_group: u32 = 0;

        for line in group.lines() {
            let calories = line.parse::<u32>().unwrap();
            calories_this_group += calories;
        }

        heap.push(calories_this_group);
    }

    Some(heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
