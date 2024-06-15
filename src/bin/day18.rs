use std::fs;
use std::time::Instant;

peg::parser!( grammar arithmetic() for str {
    rule number() -> i64
        = _ n:$(['0'..='9']+) _ { n.parse().unwrap() }

    pub(crate) rule calculate_p1() -> i64 = precedence!{
        x:(@) "+" y:@ { x + y }
        x:(@) "*" y:@ { x * y }
        --
        n:number() {n}
        _ "(" v:calculate_p1() ")" _ { v }
    }

    pub(crate) rule calculate_p2() -> i64 = precedence!{
        x:(@) "*" y:@ { x * y }
        --
        x:(@) "+" y:@ { x + y }
        --
        n:number() {n}
        _ "(" v:calculate_p2() ")" _ { v }
    }

    rule _() = quiet!{[c if c.is_whitespace()]*}
});

struct Solution;

impl Solution {
    fn part1(input: &str) -> i64 {
        input.lines().flat_map(arithmetic::calculate_p1).sum()
    }

    fn part2(input: &str) -> i64 {
        input.lines().flat_map(arithmetic::calculate_p2).sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day18.txt").expect("File not found!");

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&input),
        timer.elapsed()
    );

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&input),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_part1() {
        assert_eq!(arithmetic::calculate_p1("2 * 3 + (4 * 5)"), Ok(26));
        assert_eq!(
            arithmetic::calculate_p1("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            Ok(437)
        );
        assert_eq!(
            arithmetic::calculate_p1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(12240)
        );
        assert_eq!(
            arithmetic::calculate_p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(13632)
        );
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(
            arithmetic::calculate_p2("1 + (2 * 3) + (4 * (5 + 6))"),
            Ok(51)
        );
        assert_eq!(arithmetic::calculate_p2("2 * 3 + (4 * 5)"), Ok(46));
        assert_eq!(
            arithmetic::calculate_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            Ok(1445)
        );
        assert_eq!(
            arithmetic::calculate_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(669060)
        );
        assert_eq!(
            arithmetic::calculate_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(23340)
        );
    }
}
