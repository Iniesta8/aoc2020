use std::fs;

struct Solution;

type PasswordData = (usize, usize, char, String);

impl Solution {
    fn part1(pw_data: &Vec<PasswordData>) -> usize {
        pw_data
            .iter()
            .filter(|(min, max, ch, pw)| {
                let count = pw.chars().filter(|c| c == ch).count();
                count >= *min && count <= *max
            })
            .count()
    }

    fn part2(pw_data: &Vec<PasswordData>) -> usize {
        pw_data
            .iter()
            .map(|(min, max, ch, pw)| (min, max, ch, pw.chars().collect::<Vec<char>>()))
            .filter(|(&min, &max, &ch, pw)| (pw[min - 1] == ch) ^ (pw[max - 1] == ch))
            .count()
    }
}

fn main() {
    let input: String = fs::read_to_string("./input/day02.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut pw_data: Vec<PasswordData> = vec![];

    for l in lines.iter() {
        let mut token: Vec<&str> = l.split_whitespace().collect();

        let pw = token.pop().unwrap();
        let ch = token.pop().unwrap().chars().nth(0).unwrap();
        let minmax: Vec<&str> = token.pop().unwrap().split('-').collect();

        pw_data.push((
            minmax[0].parse().unwrap(),
            minmax[1].parse().unwrap(),
            ch,
            pw.to_string(),
        ));
    }

    println!("p1: {}", Solution::part1(&pw_data));
    println!("p2: {}", Solution::part2(&pw_data));
}
