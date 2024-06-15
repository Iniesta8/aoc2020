use std::{collections::HashSet, fs};

type Group = Vec<HashSet<char>>;

fn parse(input: &str) -> Vec<Group> {
    input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

enum Part {
    Part1,
    Part2,
}

struct Solution;

impl Solution {
    fn solve(groups: &[Group], part: Part) -> usize {
        groups
            .iter()
            .map(|g| {
                g.iter()
                    .fold(g[0].clone(), |acc, s| match part {
                        Part::Part1 => acc.union(s).cloned().collect(),
                        Part::Part2 => acc.intersection(s).cloned().collect(),
                    })
                    .len()
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day06.txt").expect("File not found!");
    let groups = parse(&input);

    println!("p1: {}", Solution::solve(&groups, Part::Part1));
    println!("p2: {}", Solution::solve(&groups, Part::Part2));
}
