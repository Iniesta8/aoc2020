use std::collections::HashMap;
use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().ok()).flatten().collect()
}

struct Solution;

impl Solution {
    fn solve_with_hashmap(starting_numbers: &[usize], target: usize) -> usize {
        let mut spoken: HashMap<usize, (usize, usize)> = HashMap::new();
        let starting_turns = starting_numbers.len();

        for (i, num) in starting_numbers.iter().enumerate() {
            spoken.insert(*num, (i + 1, 0));
        }
        let mut last_spoken = starting_numbers[starting_turns - 1];

        for turn in starting_turns + 1..=target {
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

    fn solve_with_vector(starting_numbers: &[usize], target: usize) -> usize {
        let mut spoken: Vec<(usize, usize)> = vec![(0, 0); target + 1];
        let starting_turns = starting_numbers.len();

        for (i, &num) in starting_numbers.iter().enumerate() {
            spoken[num] = (i + 1, 0);
        }
        let mut last_spoken = starting_numbers[starting_turns - 1];

        for turn in starting_turns + 1..=target {
            match spoken[last_spoken] {
                (last, sec_last) if sec_last != 0 => {
                    last_spoken = last - sec_last;
                }
                _ => last_spoken = 0,
            };
            let (last, _) = spoken[last_spoken];
            if last != 0 {
                spoken[last_spoken] = (turn, last);
            } else {
                spoken[last_spoken] = (turn, 0);
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
        "p1: {} (HashMap, runtime: {:?})",
        Solution::solve_with_hashmap(&starting_numbers, 2020),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p1: {} (Vec, runtime: {:?})",
        Solution::solve_with_vector(&starting_numbers, 2020),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p2: {} (HashMap, runtime: {:?})",
        Solution::solve_with_hashmap(&starting_numbers, 30_000_000),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p2: {} (Vec, runtime: {:?})",
        Solution::solve_with_vector(&starting_numbers, 30_000_000),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        let input = "0,3,6";
        assert_eq!(Solution::solve_with_hashmap(&parse(&input), 2020), 436);
        assert_eq!(Solution::solve_with_vector(&parse(&input), 2020), 436);
    }
}
