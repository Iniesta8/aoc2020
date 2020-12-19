use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Rule {
    Char(char),
    Rules(Vec<usize>),
    Or(Vec<Vec<usize>>),
}

fn parse(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let input_data: Vec<&str> = input.split("\n\n").collect();
    let recv_messages: Vec<String> = input_data[1].lines().map(str::to_owned).collect();
    let mut rules = HashMap::new();

    for line in input_data[0].trim().lines() {
        let line_token: Vec<&str> = line.split(':').collect();
        let num = line_token[0].parse::<usize>().unwrap();

        let rule_raw: &str = line_token[1].trim();

        if rule_raw.contains(&"|") {
            let rule_tok: Vec<&str> = rule_raw.split('|').collect();

            assert_eq!(rule_tok.len(), 2);

            let or_rules: Vec<Vec<usize>> = rule_tok
                .iter()
                .map(|rules| {
                    rules
                        .trim()
                        .split_whitespace()
                        .flat_map(str::parse)
                        .collect::<Vec<usize>>()
                })
                .collect();

            rules.insert(num, Rule::Or(or_rules));
        } else if rule_raw.starts_with('\"') {
            rules.insert(num, Rule::Char(rule_raw.chars().nth(1).unwrap()));
        } else {
            rules.insert(
                num,
                Rule::Rules(
                    rule_raw
                        .split_whitespace()
                        .flat_map(str::parse::<usize>)
                        .collect(),
                ),
            );
        }
    }

    (rules, recv_messages)
}

fn evaluate(
    msg: &str,
    pos: usize,
    rules: &HashMap<usize, Rule>,
    rule_to_eval: usize,
) -> Option<usize> {
    match rules.get(&rule_to_eval) {
        Some(Rule::Char(c)) => {
            if msg[pos..].starts_with(*c) {
                Some(pos + 1)
            } else {
                None
            }
        }
        Some(Rule::Rules(subrules)) => {
            let mut current_pos = pos;
            let mut failed = false;
            for rule_num in subrules.iter() {
                if let Some(pos) = evaluate(msg, current_pos, rules, *rule_num) {
                    current_pos = pos;
                } else {
                    failed = true;
                    break;
                }
            }

            if !failed {
                return Some(current_pos);
            }
            None
        }
        Some(Rule::Or(subrules)) => {
            for subrule in subrules {
                let mut current_pos = pos;
                let mut failed = false;
                for rule_num in subrule.iter() {
                    if let Some(pos) = evaluate(msg, current_pos, rules, *rule_num) {
                        current_pos = pos;
                    } else {
                        failed = true;
                        break;
                    }
                }

                if !failed {
                    return Some(current_pos);
                }
            }

            None
        }
        None => unreachable!(),
    }
}

fn message_valid(msg: &str, rules: &HashMap<usize, Rule>) -> bool {
    if let Some(pos) = evaluate(msg, 0, rules, 0) {
        pos == msg.len()
    } else {
        false
    }
}

struct Solution;

impl Solution {
    fn part1(messages: &Vec<String>, rules: &HashMap<usize, Rule>) -> usize {
        messages
            .iter()
            .filter(|msg| message_valid(msg, rules))
            .count()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day19.txt").expect("File not found!");

    let (rules, messages) = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&messages, &rules),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_part1() {
        let input = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let (rules, messages) = parse(&input);
        assert_eq!(Solution::part1(&messages, &rules), 2);
    }
}
