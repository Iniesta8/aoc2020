use std::fs;

struct PasswordData {
    min: usize,
    max: usize,
    ch: char,
    pw: String,
}

impl PasswordData {
    fn parse_input(s: &str) -> Option<PasswordData> {
        let mut token: Vec<&str> = s.split_whitespace().collect();

        let pw = token.pop()?.to_string();
        let ch = token.pop()?.chars().nth(0)?;
        let minmax: Vec<&str> = token.pop()?.split('-').collect();

        let min_fromstr = minmax[0].parse().ok()?;
        let max_fromstr = minmax[1].parse().ok()?;

        Some(PasswordData {
            min: min_fromstr,
            max: max_fromstr,
            ch,
            pw,
        })
    }
}

struct Solution;
impl Solution {
    fn part1(pw_data: &Vec<PasswordData>) -> usize {
        pw_data
            .iter()
            .filter(|pdata| {
                let count = pdata.pw.chars().filter(|&c| c == pdata.ch).count();
                count >= pdata.min && count <= pdata.max
            })
            .count()
    }

    fn part2(pw_data: &Vec<PasswordData>) -> usize {
        pw_data
            .iter()
            .filter(|pdata| {
                (pdata.pw.chars().nth(pdata.min - 1).unwrap() == pdata.ch)
                    ^ (pdata.pw.chars().nth(pdata.max - 1).unwrap() == pdata.ch)
            })
            .count()
    }
}

fn main() {
    let input: String = fs::read_to_string("./input/day02.txt").unwrap();

    let pw_data: Vec<PasswordData> = input
        .lines()
        .map(|l| PasswordData::parse_input(l).unwrap())
        .collect();

    println!("p1: {}", Solution::part1(&pw_data));
    println!("p2: {}", Solution::part2(&pw_data));
}
