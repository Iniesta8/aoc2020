use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn parse(input: &str) -> Vec<VecDeque<u32>> {
    input
        .split("\n\n")
        .map(|line| line.lines().skip(1).map(str::parse).flatten().collect())
        .collect()
}

fn calc_score(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .zip((1..=deck.len() as u32).rev())
        .map(|(score, i)| score * i)
        .sum()
}

fn play(decks: &mut [VecDeque<u32>]) -> u32 {
    while decks.iter().all(|d| d.len() > 0) {
        let d0 = decks[0].pop_front().unwrap();
        let d1 = decks[1].pop_front().unwrap();

        match d0.cmp(&d1) {
            std::cmp::Ordering::Less => {
                decks[1].push_back(d1);
                decks[1].push_back(d0);
            }
            std::cmp::Ordering::Equal => {
                unreachable!()
            }
            std::cmp::Ordering::Greater => {
                decks[0].push_back(d0);
                decks[0].push_back(d1);
            }
        }
    }

    calc_score(if !decks[0].is_empty() {
        &decks[0]
    } else {
        &decks[1]
    })
}

struct Solution;

impl Solution {
    fn part1(mut decks: &mut [VecDeque<u32>]) -> u32 {
        play(&mut decks)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day22.txt").expect("File not found!");
    let mut decks: Vec<VecDeque<u32>> = parse(&input);

    // dbg!(&decks);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&mut decks),
        timer.elapsed()
    );

    // let timer = Instant::now();
    // println!(
    // "p2: {} (runtime: {:?})",
    // Solution::part2(&food_list),
    // timer.elapsed()
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day22() {
        let input = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let mut decks: Vec<VecDeque<u32>> = parse(&input);
        assert_eq!(Solution::part1(&mut decks), 306);
    }
}
