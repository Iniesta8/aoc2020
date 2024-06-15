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
            .1
            .iter()
            .map(|&(i, bus_id)| (-(i as isize), bus_id as isize))
            .unzip();

        chinese_remainder_theorem(&u, &m)
            .unwrap()
            .rem_euclid(m.iter().product::<isize>()) as usize
    }

    fn part2_alt(notes: &Notes) -> usize {
        let mut running_product = 1;
        let mut ans = 0;

        for &(i, int) in notes.1.iter() {
            while (ans + i) % int != 0 {
                ans += running_product;
            }
            running_product *= int;
        }
        ans
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

    let timer = Instant::now();
    println!(
        "p2 (alternative): {} (runtime: {:?})",
        Solution::part2_alt(&notes),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1() {
        assert_eq!(Solution::part1(&parse("939\n7,13,x,x,59,x,31,19")), 295);
    }

    #[test]
    fn test_day13_part2() {
        assert_eq!(Solution::part2(&parse("0\n7,13,x,x,59,x,31,19")), 1068781);
        assert_eq!(Solution::part2(&parse("0\n17,x,13,19")), 3417);
        assert_eq!(Solution::part2(&parse("0\n67,7,59,61")), 754018);
        assert_eq!(Solution::part2(&parse("0\n67,x,7,59,61")), 779210);
        assert_eq!(Solution::part2(&parse("0\n67,7,x,59,61")), 1261476);
        assert_eq!(Solution::part2(&parse("0\n1789,37,47,1889")), 1202161486);

        assert_eq!(
            Solution::part2_alt(&parse("0\n7,13,x,x,59,x,31,19")),
            1068781
        );
    }
}
