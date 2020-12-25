use std::fs;
use std::time::Instant;

fn read_pubkeys_from_file(input: &str) -> (usize, usize) {
    let keys: Vec<usize> = input.trim().lines().map(str::parse).flatten().collect();

    (keys[0], keys[1])
}

fn calc_loop_size(pubkey: usize) -> usize {
    let mut val = 1;
    let subj_num = 7;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        val *= subj_num;
        val %= 20201227;

        if val == pubkey {
            return loop_size;
        }
    }
}

fn calc_encryption_key(pubkey: usize, loop_size: usize) -> usize {
    let mut val = 1;

    for _ in 0..loop_size {
        val *= pubkey;
        val %= 20201227;
    }
    val
}

struct Solution;

impl Solution {
    fn part1(pubkeys: &(usize, usize)) -> usize {
        let &(cards_pubkey, doors_pubkey) = pubkeys;
        let cards_loop_size = calc_loop_size(cards_pubkey);
        calc_encryption_key(doors_pubkey, cards_loop_size)
    }
}

fn main() {
    let input = fs::read_to_string("./input/day25.txt").expect("File not found!");
    let pubkeys = read_pubkeys_from_file(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&pubkeys),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day25() {
        let pubkeys = (5764801, 17807724);
        let (cards_pubkey, doors_pubkey) = pubkeys;

        assert_eq!(calc_loop_size(cards_pubkey), 8);
        assert_eq!(calc_loop_size(doors_pubkey), 11);

        assert_eq!(Solution::part1(&pubkeys), 14897079);
    }
}
