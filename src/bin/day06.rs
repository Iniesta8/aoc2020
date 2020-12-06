use std::{collections::HashSet, fs};

type GroupList = Vec<Vec<HashSet<char>>>;

fn parse(input: &String) -> GroupList {
    input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

struct Solution;

impl Solution {
    fn part1(groups: &GroupList) -> usize {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(HashSet::new(), |acc, s| {
                        acc.union(&s).map(|c| c.clone()).collect()
                    })
                    .len()
            })
            .sum()
    }

    fn part2(groups: &GroupList) -> usize {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(g[0].clone(), |acc, s| {
                        acc.intersection(&s).map(|c| c.clone()).collect()
                    })
                    .len()
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day06.txt").expect("File not found!");
    let groups = parse(&input);

    println!("p1: {}", Solution::part1(&groups));
    println!("p2: {}", Solution::part2(&groups));
}
