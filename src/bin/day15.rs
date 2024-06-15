use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

struct Solution;

impl Solution {
    fn solve(starting_numbers: &[usize], target: usize) -> usize {
        let mut spoken: Vec<(usize, usize)> = vec![(0, 0); target + 1];

        let len_starting = starting_numbers.len();
        for (i, &num) in starting_numbers.iter().enumerate() {
            spoken[num] = (i + 1, 0);
        }

        let mut last_spoken = starting_numbers[len_starting - 1];
        for turn in len_starting + 1..=target {
            match spoken[last_spoken] {
                (last, sec_last) if sec_last != 0 => {
                    last_spoken = last - sec_last;
                }
                _ => last_spoken = 0,
            };
            let (last, _) = spoken[last_spoken];
            spoken[last_spoken] = (turn, if last != 0 { last } else { 0 });
        }
        last_spoken
    }
}

fn main() {
    let input = "5,2,8,16,18,0,1";
    let starting_numbers = parse(input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::solve(&starting_numbers, 2020),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::solve(&starting_numbers, 30_000_000),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        let input = "0,3,6";
        assert_eq!(Solution::solve(&parse(input), 2020), 436);
    }
}
