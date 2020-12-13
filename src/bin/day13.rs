use std::fs;
use std::time::Instant;

type Notes = (usize, Vec<(usize, usize)>);

fn parse(input: &str) -> Notes {
    let mut lines = input.split('\n');

    let earliest = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Given timestamp not a number");

    let intervals: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, c)| (i, c.parse::<usize>().unwrap()))
        .collect();

    (earliest, intervals)
}

fn find_next_departures(
    earliest: usize,
    intervals: &[(usize, usize)],
) -> Vec<(usize, usize, usize)> {
    intervals
        .iter()
        .map(|&(offset, int)| (int, offset, (earliest / int) * int + int))
        .collect()
}

struct Solution;

impl Solution {
    fn part1(notes: &Notes) -> usize {
        let (earliest, intervals) = notes;
        let next_departures = find_next_departures(*earliest, intervals);
        let (bus_id, _, departure) = next_departures.iter().min_by_key(|&(_, _, ts)| ts).unwrap();

        bus_id * (departure - earliest)
    }

    fn part2(notes: &Notes) -> usize {
        let mut t = 0;
        let intervals = notes.1.clone();

        loop {
            if find_next_departures(t, &intervals[1..])
                .iter()
                .all(|(_, offset, next_dep)| (t + offset) == *next_dep)
            {
                return t;
            }
            t += intervals[0].1;
        }
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
