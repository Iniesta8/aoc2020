use std::fs;

struct Solution;

impl Solution {
    fn part1(pw_data: &Vec<(usize, usize, char, String)>) -> usize {
        pw_data
            .iter()
            .filter(|(min, max, ch, pw)| {
                let count = pw.chars().filter(|c| c == ch).count();
                count >= *min && count <= *max
            })
            .count()
    }

    fn part2(pw_data: &Vec<(usize, usize, char, String)>) -> usize {
        pw_data
            .iter()
            .map(|(min, max, ch, pw)| (min, max, ch, pw.chars().collect::<Vec<char>>()))
            .filter(|(&min, &max, &ch, pw)| (pw[min - 1] == ch) ^ (pw[max - 1] == ch))
            .count()
    }
}

fn main() {
    let lines: String = fs::read_to_string("./input/day02.txt").unwrap();
    let lines: Vec<&str> = lines.lines().collect();

    let mut pw_data: Vec<(usize, usize, char, String)> = vec![];

    for l in lines.iter() {
        let v: Vec<String> = l.split(": ").map(|x| x.to_string()).collect();

        let mut tmp: (usize, usize, char, String) = (0, 0, ' ', "".to_string());
        tmp.3 = v[1].clone();

        let policy: Vec<&str> = v[0].split(' ').collect();
        tmp.2 = policy[1].chars().nth(0).unwrap();

        let minmax: Vec<&str> = policy[0].split('-').collect();
        tmp.0 = minmax[0].parse::<usize>().unwrap();
        tmp.1 = minmax[1].parse::<usize>().unwrap();

        pw_data.push(tmp);
    }

    println!("p1: {}", Solution::part1(&pw_data));
    println!("p2: {}", Solution::part2(&pw_data));
}
