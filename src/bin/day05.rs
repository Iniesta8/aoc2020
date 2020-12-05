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
    fn part1(seat_ids: &Vec<u32>) -> u32 {
        *seat_ids.iter().max().unwrap()
    }

    fn part2(seat_ids: &Vec<u32>) -> Option<u32> {
        let seats: HashSet<u32> = seat_ids.iter().cloned().collect();

        (2..(128 * 8)).into_iter().find(|id| {
            seats.contains(&(id - 1)) && !seats.contains(&id) && seats.contains(&(id + 1))
        })
    }
}

fn main() {
    let input = fs::read_to_string("./input/day05.txt").expect("File not found!");

    let seat_ids: Vec<u32> = input.trim().lines().map(parse).collect();

    println!("p1: {}", Solution::part1(&seat_ids));
    println!(
        "p2: {}",
        Solution::part2(&seat_ids).expect("Empty seat not found!")
    );
}
