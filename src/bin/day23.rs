use std::char;
use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn list_to_vec(cups: &[usize], current_cup: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut ptr = current_cup;
    for _ in 0..cups.len() {
        res.push(ptr);
        ptr = cups[ptr];
    }
    res
}

fn single_move(cups_list: &mut [usize], current_cup: usize) -> usize {
    let n = cups_list.len() - 1;

    // pick up
    let p1 = cups_list[current_cup];
    let p2 = cups_list[p1];
    let p3 = cups_list[p2];
    let picks: Vec<usize> = vec![p1, p2, p3];

    cups_list[current_cup] = cups_list[p3];

    let mut dst = if current_cup > 1 { current_cup - 1 } else { n };
    while picks.contains(&dst) {
        dst = if dst > 1 { dst - 1 } else { n };
    }

    cups_list[p3] = cups_list[dst];
    cups_list[dst] = p1;

    cups_list[current_cup]
}

struct Solution;

impl Solution {
    fn part1(cups: Vec<usize>) -> String {
        let num_cups = cups.len();

        // build poor man's linked list
        let mut cups_list: Vec<usize> = vec![0; num_cups + 1];
        for w in cups.windows(2) {
            cups_list[w[0]] = w[1];
        }
        cups_list[cups[num_cups - 1]] = cups[0];

        let mut current_cup = cups[0];
        for _ in 0..100 {
            current_cup = single_move(&mut cups_list, current_cup);
        }
        let final_cups = list_to_vec(&cups_list, 1);
        final_cups
            .iter()
            .filter(|&c| *c != 1)
            .map(|c| char::from_digit(*c as u32, 10).unwrap())
            .collect()
    }

    fn part2(cups: Vec<usize>) -> usize {
        let num_cups = 1_000_000;

        // build poor man's linked list
        let mut cups_list: Vec<usize> = (1..(num_cups + 2)).into_iter().collect();
        cups_list[num_cups] = cups[0];
        for w in cups.windows(2) {
            cups_list[w[0]] = w[1];
        }
        cups_list[cups[cups.len() - 1]] = cups.len() + 1;

        let mut current_cup = cups[0];
        for _ in 0..10_000_000 {
            current_cup = single_move(&mut cups_list, current_cup);
        }
        let final_cups = list_to_vec(&cups_list, 1);
        final_cups[1] * final_cups[2]
    }
}

fn main() {
    let input = "123487596";
    let cups: Vec<usize> = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(cups.clone()),
        timer.elapsed()
    );

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(cups),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day23() {
        let input = "389125467";
        let cups: Vec<usize> = parse(&input);
        assert_eq!(Solution::part1(cups.clone()), String::from("67384529"));
        assert_eq!(Solution::part2(cups), 149245887792);
    }
}
