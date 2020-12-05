use std::fs;

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

    fn part2(seat_ids: &mut Vec<u32>) -> Option<u32> {
        seat_ids.sort_unstable();

        match seat_ids.windows(2).find(|w| w[0] + 1 != w[1]) {
            Some(w) => Some(w[0] + 1),
            _ => None,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/day05.txt").expect("File not found!");
    let mut seat_ids: Vec<u32> = input.trim().lines().map(parse).collect();

    println!("p1: {}", Solution::part1(&seat_ids));
    println!(
        "p2: {}",
        Solution::part2(&mut seat_ids).expect("Empty seat not found!")
    );
}
