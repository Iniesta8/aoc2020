use regex::Regex;
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

fn build_regex(rules: &HashMap<usize, Rule>, rule_to_eval: usize) -> String {
    if rule_to_eval == 8 {
        return format!("({}+)", build_regex(rules, 42));
    }
    if rule_to_eval == 11 {
        // let a = build_regex(rules, 42);
        // let b = build_regex(rules, 31);
        //
        // let mut s = "(?:".to_owned();
        //
        // for i in 1..100 {
        // s += &format!(
        // "{{{}}}{{{}}}{{{}}}{{{}}}",
        // a,
        // i.to_string(),
        // b,
        // i.to_string()
        // );
        //
        // if i < 100 - 1 {
        // s += "|";
        // }
        // }
        // s += ")";
        // return s;

        return format!(
            "(?P<X11>{}(?P&X11){}|{}{})",
            build_regex(rules, 42),
            build_regex(rules, 31),
            build_regex(rules, 42),
            build_regex(rules, 31)
        );
    }
    if let Some(Rule::Char(c)) = rules.get(&rule_to_eval) {
        return format!("({})", c);
    }
    match rules.get(&rule_to_eval) {
        Some(Rule::Rules(inner)) => {
            let mut s = "(".to_owned();
            for r in inner {
                s += &build_regex(rules, *r);
            }
            s += ")";
            s
        }
        Some(Rule::Or(subrules)) => {
            let mut s = "(".to_owned();
            let len = subrules.len() - 1;
            for (i, subrule) in subrules.iter().enumerate() {
                for r in subrule {
                    s += &build_regex(rules, *r);
                }
                if i < len {
                    s += "|";
                }
            }
            s += ")";
            s
        }
        _ => unreachable!(),
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

    fn part2(messages: &Vec<String>, rules: &HashMap<usize, Rule>) -> usize {
        let regex_string = format!("^{}$", build_regex(rules, 0));

        println!("{}", regex_string);
        0

        // let re = Regex::new(&regex_string).unwrap();
        // messages.iter().filter(|msg| re.is_match(msg)).count()
    }
}

fn main() {
    let input1 = fs::read_to_string("./input/day19.txt").expect("File not found!");
    let (rules, messages) = parse(&input1);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&messages, &rules),
        timer.elapsed()
    );

    let input2 = fs::read_to_string("./input/day19_2.txt").expect("File not found!");
    let (rules, messages) = parse(&input2);

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&messages, &rules),
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

    // #[test]
    // fn test_day19_part2() {
    // let input = "\
    // 42: 9 14 | 10 1
    // 9: 14 27 | 1 26
    // 10: 23 14 | 28 1
    // 1: \"a\"
    // 11: 42 31
    // 5: 1 14 | 15 1
    // 19: 14 1 | 14 14
    // 12: 24 14 | 19 1
    // 16: 15 1 | 14 14
    // 31: 14 17 | 1 13
    // 6: 14 14 | 1 14
    // 2: 1 24 | 14 4
    // 0: 8 11
    // 13: 14 3 | 1 12
    // 15: 1 | 14
    // 17: 14 2 | 1 7
    // 23: 25 1 | 22 14
    // 28: 16 1
    // 4: 1 1
    // 20: 14 14 | 1 15
    // 3: 5 14 | 16 1
    // 27: 1 6 | 14 18
    // 14: \"b\"
    // 21: 14 1 | 1 14
    // 25: 1 1 | 1 14
    // 22: 14 14
    // 8: 42
    // 26: 14 22 | 1 20
    // 18: 15 15
    // 7: 14 5 | 1 21
    // 24: 14 1
    //
    // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    // bbabbbbaabaabba
    // babbbbaabbbbbabbbbbbaabaaabaaa
    // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    // bbbbbbbaaaabbbbaaabbabaaa
    // bbbababbbbaaaaaaaabbababaaababaabab
    // ababaaaaaabaaab
    // ababaaaaabbbaba
    // baabbaaaabbaaaababbaababb
    // abbbbabbbbaaaababbbbbbaaaababb
    // aaaaabbaabaaaaababaa
    // aaaabbaaaabbaaa
    // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    // babaaabbbaaabaababbaabababaaab
    // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    // let (rules, messages) = parse(&input);
    // assert_eq!(Solution::part2(&messages, &rules), 3);
    // }
}
