use std::fs;
use std::time::Instant;

fn parse(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.split('\n');

    let earliest_depart = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Given timestamp not a number");

    let intervals = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect();

    (earliest_depart, intervals)
}

fn find_next_departures(earliest_depart: usize, intervals: &[usize]) -> Vec<(usize, usize)> {
    intervals
        .iter()
        .map(|&int| (int, (earliest_depart / int) * int + int))
        .collect()
}

struct Solution;

impl Solution {
    fn part1(notes: &(usize, Vec<usize>)) -> usize {
        let (earliest_depart, intervals) = notes;
        let next_departures = find_next_departures(*earliest_depart, &intervals);
        let (bus_id, departure) = next_departures.iter().min_by_key(|&(_, ts)| ts).unwrap();

        bus_id * (departure - earliest_depart)
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
    }
}
