use std::cmp::Ordering;
use std::fs;

struct Solution;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(str::parse).flatten().collect()
}

fn find_invalid_number(data: &[usize], wlen: usize) -> Option<usize> {
    'outer: for w in data.windows(wlen + 1) {
        let target = w[wlen];

        for i in 0..wlen {
            for j in 0..wlen {
                if w[i] + w[j] == target && i != j {
                    continue 'outer;
                }
            }
        }
        return Some(target);
    }
    None
}

fn find_enc_weakness(data: &[usize], target: usize) -> usize {
    let mut lower = 0;
    let mut upper = 1;

    loop {
        match data[lower..upper].iter().sum::<usize>().cmp(&target) {
            Ordering::Greater => lower += 1,
            Ordering::Equal => break,
            Ordering::Less => upper += 1,
        }
    }

    data[lower..upper].iter().min().unwrap() + data[lower..upper].iter().max().unwrap()
}

impl Solution {
    fn part1(data: &[usize]) -> Option<usize> {
        find_invalid_number(&data, 25)
    }

    fn part2(data: &[usize], target: usize) -> usize {
        find_enc_weakness(&data, target)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day09.txt").expect("File not found!");
    let data = parse(&input);

    let invalid_number = Solution::part1(&data).unwrap();
    println!("p1: {}", invalid_number);
    println!("p2: {}", Solution::part2(&data, invalid_number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console() {
        let input = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let data = parse(&input);
        assert_eq!(find_invalid_number(&data, 5), Some(127));
        assert_eq!(find_enc_weakness(&data, 127), 62);
    }
}
