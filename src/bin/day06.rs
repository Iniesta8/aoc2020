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
    fn solve(groups: &GroupList, part: usize) -> usize {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(g[0].clone(), |acc, s| match part {
                        1 => acc.union(&s).cloned().collect(),
                        2 => acc.intersection(&s).cloned().collect(),
                        _ => unimplemented!(),
                    })
                    .len()
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day06.txt").expect("File not found!");
    let groups = parse(&input);

    println!("p1: {}", Solution::solve(&groups, 1));
    println!("p2: {}", Solution::solve(&groups, 2));
}
