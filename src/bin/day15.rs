use std::collections::HashMap;
use std::time::Instant;

fn parse(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().ok()).flatten().collect()
}

struct Solution;

impl Solution {
    fn solve(starting_numbers: &[u32], target: u32) -> u32 {
        let mut spoken: HashMap<u32, (u32, u32)> = HashMap::new();
        let starting_turns = starting_numbers.len();

        for (i, num) in starting_numbers.iter().enumerate() {
            spoken.insert(*num, (i as u32 + 1, 0));
        }
        let mut last_spoken = starting_numbers[starting_turns - 1];

        for turn in starting_turns as u32 + 1..=target {
            match spoken[&last_spoken] {
                (last, sec_last) if sec_last != 0 => {
                    last_spoken = last - sec_last;
                }
                _ => last_spoken = 0,
            };
            if let Some(entry) = spoken.get_mut(&last_spoken) {
                *entry = (turn, entry.0);
            } else {
                spoken.insert(last_spoken, (turn, 0));
            }
        }
        last_spoken
    }
}

fn main() {
    let input = "5,2,8,16,18,0,1";
    let starting_numbers = parse(&input);

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
        assert_eq!(Solution::solve(&parse(&input), 2020), 436);
    }
}
