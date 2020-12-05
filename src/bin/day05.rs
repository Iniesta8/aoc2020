use std::{collections::HashSet, fs};

fn parse(input: &str) -> u32 {
    input
        .chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("Invalid input!"),
        })
        .fold(0, |a, b| a << 1 | b)
}

struct Solution;

impl Solution {
    fn part1(input: &String) -> u32 {
        input.trim().lines().map(parse).max().unwrap()
    }

    fn part2(input: &String) -> u32 {
        let seats: HashSet<u32> = input.trim().lines().map(parse).collect();

        for id in 1..=(128 * 8) {
            if seats.contains(&(id - 1)) && !seats.contains(&id) && seats.contains(&(id + 1)) {
                return id;
            }
        }
        0
    }
}

fn main() {
    let input = fs::read_to_string("./input/day05.txt").expect("File not found!");

    println!("p1: {}", Solution::part1(&input));
    println!("p2: {:?}", Solution::part2(&input));
}
