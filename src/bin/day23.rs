use std::char;
use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect()
}

fn play(cups: Vec<usize>) -> Vec<usize> {
    let mut cups = cups;
    let mut current_cup = 0;

    for i in 0..100 {
        println!("-- move {} --", i + 1);
        println!("cups: {:?}", cups);

        let current_cup_label = cups[current_cup];
        let mut pick_up: Vec<usize> = vec![];
        let mut pick_pos = current_cup + 1;
        for _ in 0..3 {
            if pick_pos < cups.len() {
                pick_up.push(cups.remove(pick_pos));
            } else {
                pick_pos = 0;
                pick_up.push(cups.remove(pick_pos));
            }
        }

        let lowest_cup = *cups.iter().min().unwrap();
        let highest_cup = *cups.iter().max().unwrap();
        let mut destination_cup = current_cup_label - 1;

        println!("current cup label: {}", current_cup_label);
        println!("pick up: {:?}", pick_up);

        if destination_cup < lowest_cup {
            destination_cup = highest_cup;
        }

        while pick_up.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < lowest_cup {
                destination_cup = highest_cup;
            }
        }
        println!("destination: {}", destination_cup);

        let new_pos = (cups.iter().position(|&c| c == destination_cup).unwrap() + 1) % cups.len();

        if new_pos >= cups.len() {
            cups.append(&mut pick_up);
        } else {
            for j in 0..pick_up.len() {
                cups.insert(new_pos + j, pick_up[j]);
            }
        }

        current_cup = (cups.iter().position(|&c| c == current_cup_label).unwrap() + 1) % cups.len();
    }

    cups
}

struct Solution;

impl Solution {
    fn part1(cups: Vec<usize>) -> String {
        let final_cups = play(cups);

        final_cups
            .iter()
            .filter(|&c| *c != 1)
            .map(|c| char::from_digit(*c as u32, 10).unwrap())
            .collect()
    }
}

fn main() {
    let input = "123487596";
    let cups: Vec<usize> = parse(&input);
    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(cups),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day22() {
        let input = "389125467";
        let cups: Vec<usize> = parse(&input);
        assert_eq!(Solution::part1(cups), String::from("67384529"));
    }
}
