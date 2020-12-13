use std::fs;
use std::time::Instant;

use ring_algorithm::chinese_remainder_theorem;

type Notes = (usize, Vec<(usize, usize)>);

fn parse(input: &str) -> Notes {
    let mut lines = input.split('\n');
    let earliest = lines.next().unwrap().parse::<usize>().unwrap();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, c)| (i, c.parse::<usize>().unwrap()))
        .collect();

    (earliest, buses)
}

fn find_next_departures(earliest: usize, buses: &[(usize, usize)]) -> Vec<(usize, usize, usize)> {
    buses
        .iter()
        .map(|&(offset, int)| (int, offset, (earliest / int) * int + int))
        .collect()
}

struct Solution;

impl Solution {
    fn part1(notes: &Notes) -> usize {
        let (earliest, buses) = notes;
        let next_departures = find_next_departures(*earliest, buses);
        let (id, _, departure) = next_departures.iter().min_by_key(|&(_, _, ts)| ts).unwrap();

        id * (departure - earliest)
    }

    fn part2(notes: &Notes) -> usize {
        let (u, m): (Vec<_>, Vec<_>) = notes
            .1 //intervals
            .iter()
            .map(|&(i, bus_id)| (-(i as isize), bus_id as isize))
            .unzip();

        chinese_remainder_theorem(&u, &m).unwrap() as usize
    }
}

fn main() {
    let input = fs::read_to_string("./input/day13.txt").expect("File not found!");
    let notes = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&notes),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&notes),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
939
7,13,x,x,59,x,31,19";
        let notes = parse(&input);
        assert_eq!(Solution::part1(&notes), 295);
        assert_eq!(Solution::part2(&notes), 1068781);
    }
}
