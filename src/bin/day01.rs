use std::collections::HashMap;
use std::fs;

struct Solution;

impl Solution {
    fn find_sum_pair(data: &[i32], target: i32) -> Option<(i32, i32)> {
        let mut set = HashMap::new();

        for v in data {
            let comp = target - v;
            match set.get(&comp) {
                Some(val) => return Some((comp, *val)),
                None => set.insert(*v, comp),
            };
        }
        None
    }

    fn find_sum_triple(data: &[i32], target: i32) -> Option<(i32, i32, i32)> {
        for v in data {
            let comp = target - v;
            if let Some((a, b)) = Self::find_sum_pair(data, comp) {
                if a != b && a != *v && b != *v {
                    return Some((a, b, *v));
                }
            }
        }
        None
    }

    fn part1(data: &[i32], target: i32) -> i32 {
        match Self::find_sum_pair(data, target) {
            Some((a, b)) => a * b,
            None => -1,
        }
    }

    fn part2(data: &[i32], target: i32) -> i32 {
        match Self::find_sum_triple(data, target) {
            Some((a, b, c)) => a * b * c,
            None => -1,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/day01.txt").expect("File not found!");
    let values: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("p1: {}", Solution::part1(&values, 2020));
    println!("p2: {}", Solution::part2(&values, 2020));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_find_sum_pair() {
        assert_eq!(
            Solution::find_sum_pair(&vec![1721, 979, 366, 299, 675, 1456], 2020),
            Some((1721, 299))
        );
    }

    #[test]
    fn test_day01_find_sum_triple() {
        assert_eq!(
            Solution::find_sum_triple(&vec![1721, 979, 366, 299, 675, 1456], 2020),
            Some((366, 675, 979))
        );
    }
}
