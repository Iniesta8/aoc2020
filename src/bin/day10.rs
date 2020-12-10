use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(str::parse).flatten().collect()
}

fn build_chain(adapters: &mut Vec<usize>) {
    // add charging outlet's effective rating
    adapters.push(0);

    adapters.sort_unstable();

    // add device's built-in adapter
    adapters.push(adapters[adapters.len() - 1] + 3);
}

fn get_diffs(adapters: &[usize]) -> Vec<usize> {
    adapters.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn get_diff_one_sequences(adapters: &[usize]) -> Vec<Vec<usize>> {
    get_diffs(adapters)
        .split(|&val| val == 3)
        .map(|e| e.to_vec())
        .collect()
}

struct Solution;

impl Solution {
    fn part1(adapters: &[usize]) -> usize {
        let (ones, threes) =
            adapters
                .windows(2)
                .fold((0, 0), |(ones, threes), pair| match pair[1] - pair[0] {
                    1 => (ones + 1, threes),
                    3 => (ones, threes + 1),
                    _ => (ones, threes),
                });
        ones * threes
    }

    fn part2(adapters: &[usize]) -> usize {
        // number of possible combinations per diff-one-sequence's len
        // (0 -> 1)
        // 1 -> 1
        // 2 -> 2
        // 3 -> 4
        // 4 -> 7
        // ...
        let num_combinations = [1, 1, 2, 4, 7];
        let one_sequences = get_diff_one_sequences(adapters);

        one_sequences
            .iter()
            .map(|seq| num_combinations[seq.len()])
            .product()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day10.txt").expect("File not found!");
    let mut adapters = parse(&input);
    build_chain(&mut adapters);

    println!("p1: {}", Solution::part1(&adapters));
    println!("p2: {}", Solution::part2(&adapters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
16
10
15
5
1
11
7
19
6
12
4";
        let mut adapters = parse(&input);
        build_chain(&mut adapters);
        assert_eq!(Solution::part1(&adapters), 35);
    }
}
