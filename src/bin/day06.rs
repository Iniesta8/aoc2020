use std::{collections::HashSet, fs};

struct Solution;

impl Solution {
    fn part1(groups: &Vec<Vec<HashSet<char>>>) -> u32 {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(HashSet::new(), |acc, s| {
                        acc.union(&s).map(|c| c.clone()).collect()
                    })
                    .len() as u32
            })
            .sum()
    }

    fn part2(groups: &Vec<Vec<HashSet<char>>>) -> u32 {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(g[0].clone(), |acc, s| {
                        acc.intersection(&s).map(|c| c.clone()).collect()
                    })
                    .len() as u32
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day06.txt").expect("File not found!");
    let groups: Vec<Vec<HashSet<char>>> = input
        .trim()
        .split("\n\n")
        .map(|g| {
            g.to_owned()
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().collect())
                .collect()
        })
        .collect();

    println!("p1: {}", Solution::part1(&groups));
    println!("p2: {}", Solution::part2(&groups));
}
