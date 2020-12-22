use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn parse(input: &str) -> Vec<VecDeque<usize>> {
    input
        .split("\n\n")
        .map(|line| line.lines().skip(1).map(str::parse).flatten().collect())
        .collect()
}

fn calc_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(score, i)| score * i)
        .sum()
}

fn play_crab_combat(decks: &mut [VecDeque<usize>]) -> usize {
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

    if !decks[0].is_empty() {
        0
    } else {
        1
    }
}

fn play_recursive_combat(decks: &mut [VecDeque<usize>], game: usize, round: usize) -> usize {
    let mut game = game;
    let mut round = round;
    let mut played_decks: HashSet<VecDeque<usize>> = HashSet::new();
    while decks.iter().all(|d| d.len() > 0) {
        round += 1;

        if !played_decks.insert(decks[0].clone()) {
            println!("Already played decks: {:?}/{:?}", decks[0], decks[1]);
            println!("Player 1 wins!");
            return 0;
        }

        println!("Player 1's deck: {:?}", decks[0]);
        println!("Player 2's deck: {:?}", decks[1]);
        let d0 = decks[0].pop_front().unwrap();
        let d1 = decks[1].pop_front().unwrap();

        println!("Player 1 plays: {:?}", d0);
        println!("Player 2 plays: {:?}", d1);

        if decks[0].len() >= d0 && decks[1].len() >= d1 {
            // start subgame
            println!("Playing a sub-game to determine the winner...");
            game += 1;
            let winner = play_recursive_combat(
                &mut vec![
                    decks[0].iter().take(d0).cloned().collect(),
                    decks[1].iter().take(d1).cloned().collect(),
                ],
                game,
                0,
            );
            if winner == 0 {
                decks[0].push_back(d0);
                decks[0].push_back(d1);
            } else {
                decks[1].push_back(d1);
                decks[1].push_back(d0);
            }
        } else {
            match d0.cmp(&d1) {
                std::cmp::Ordering::Less => {
                    decks[1].push_back(d1);
                    decks[1].push_back(d0);
                    println!("Player 2 wins round {} of game {}!", round, game);
                }
                std::cmp::Ordering::Equal => {
                    unreachable!()
                }
                std::cmp::Ordering::Greater => {
                    decks[0].push_back(d0);
                    decks[0].push_back(d1);
                    println!("Player 1 wins round {} of game {}!", round, game);
                }
            }
        }
    }

    if !decks[0].is_empty() {
        0
    } else {
        1
    }
}

struct Solution;

impl Solution {
    fn part1(mut decks: &mut [VecDeque<usize>]) -> usize {
        let winner = play_crab_combat(&mut decks);
        calc_score(&decks[winner])
    }

    fn part2(mut decks: &mut [VecDeque<usize>]) -> usize {
        let winner = play_recursive_combat(&mut decks, 1, 0);
        calc_score(&decks[winner])
    }
}

fn main() {
    let input = fs::read_to_string("./input/day22.txt").expect("File not found!");
    let mut decks: Vec<VecDeque<usize>> = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&mut decks),
        timer.elapsed()
    );

    decks = parse(&input);
    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&mut decks),
        timer.elapsed()
    );
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
        let mut decks: Vec<VecDeque<usize>> = parse(&input);
        assert_eq!(Solution::part1(&mut decks), 306);
        decks = parse(&input);
        assert_eq!(Solution::part2(&mut decks), 291);
    }
}
