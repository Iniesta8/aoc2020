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
    while decks.iter().all(|d| !d.is_empty()) {
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

fn play_recursive_combat(decks: &mut [VecDeque<usize>], game: &mut usize, round: usize) -> usize {
    let mut round = round;
    let mut played_decks: HashSet<VecDeque<usize>> = HashSet::new();
    *game += 1;
    let mut next_game = *game + 1;
    let mut winner = 42;

    println!("=== Game {} ===\n", game);

    while decks.iter().all(|d| !d.is_empty()) {
        round += 1;

        println!("-- Round {} (Game {}) --", round, game);

        if !played_decks.insert(decks[0].clone()) {
            println!("Already played decks: {:?}/{:?}", decks[0], decks[1]);
            println!("Player 1 wins!\n");
            return 0;
        }

        println!("Player 1's deck: {:?}", decks[0]);
        println!("Player 2's deck: {:?}", decks[1]);
        let d0 = decks[0].pop_front().unwrap();
        let d1 = decks[1].pop_front().unwrap();

        println!("Player 1 plays: {:?}", d0);
        println!("Player 2 plays: {:?}", d1);

        if decks[0].len() >= d0 && decks[1].len() >= d1 {
            // start a subgame
            println!("Playing a sub-game to determine the winner...\n");
            let winner = play_recursive_combat(
                &mut [
                    decks[0].iter().take(d0).cloned().collect(),
                    decks[1].iter().take(d1).cloned().collect(),
                ],
                &mut next_game,
                0,
            );
            if winner == 0 {
                decks[0].push_back(d0);
                decks[0].push_back(d1);
            } else {
                decks[1].push_back(d1);
                decks[1].push_back(d0);
            }
            println!("...anyway, back to game {}", game);
            println!(
                "Player {} wins round {} of game {}!\n",
                winner + 1,
                round,
                game
            );
        } else {
            match d0.cmp(&d1) {
                std::cmp::Ordering::Less => {
                    winner = 1;
                    decks[1].push_back(d1);
                    decks[1].push_back(d0);
                    println!("Player 2 wins round {} of game {}!\n", round, game);
                }
                std::cmp::Ordering::Equal => {
                    unreachable!()
                }
                std::cmp::Ordering::Greater => {
                    winner = 0;
                    decks[0].push_back(d0);
                    decks[0].push_back(d1);
                    println!("Player 1 wins round {} of game {}!\n", round, game);
                }
            }
        }
    }

    println!("The winner of game {} is player {}!\n", game, winner + 1);
    winner
}

struct Solution;

impl Solution {
    fn part1(mut decks: &mut [VecDeque<usize>]) -> usize {
        let winner = play_crab_combat(&mut decks);
        calc_score(&decks[winner])
    }

    fn part2(mut decks: &mut [VecDeque<usize>]) -> usize {
        let winner = play_recursive_combat(&mut decks, &mut 0, 0);

        println!("== Post-game results ==");
        println!("Player 1's deck: {:?}", decks[0]);
        println!("Player 2's deck: {:?}", decks[1]);

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
