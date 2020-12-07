use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Bags = HashMap<String, HashSet<(u32, String)>>;

fn parse(input: &str) -> Bags {
    let mut bags: Bags = HashMap::new();

    for line in input.lines() {
        let mut bag_iter = line.split(" bags contain");

        let outer_bag = bag_iter.next().unwrap();
        let inner_bag_list = bag_iter.next().unwrap();

        let mut inner_bags: HashSet<(u32, String)> = HashSet::new();

        for bag in inner_bag_list
            .trim()
            .split(", ")
            .filter(|s| !s.starts_with("no "))
        {
            let bag = bag
                .replace('.', "")
                .replace(" bags", "")
                .replace(" bag", "");

            let bag: Vec<&str> = bag.splitn(2, ' ').collect();
            let (n, color): (u32, String) = (bag[0].parse::<u32>().unwrap(), bag[1].to_owned());

            inner_bags.insert((n, color));
        }
        bags.insert(outer_bag.to_owned(), inner_bags);
    }
    bags
}

const MY_BAG: &'static str = "shiny gold";

struct Solution;

impl Solution {
    fn part1(bags: &Bags) -> u32 {
        fn contains_shiny_gold(bag: &str, bags: &Bags) -> bool {
            bags[bag]
                .iter()
                .any(|(_, b)| b == MY_BAG || contains_shiny_gold(b, bags))
        }

        bags.keys()
            .filter(|bag| contains_shiny_gold(bag, bags))
            .count() as u32
    }

    fn part2(bags: &Bags) -> u32 {
        fn sum_bags(bag: &str, bags: &Bags) -> u32 {
            bags[bag]
                .iter()
                .map(|(n, b)| n * sum_bags(b, bags))
                .sum::<u32>()
                + 1
        }

        sum_bags(MY_BAG, &bags) - 1 as u32
    }
}

fn main() {
    let input = fs::read_to_string("./input/day07.txt").expect("File not found!");
    let bags = parse(&input);

    println!("p1: {}", Solution::part1(&bags));
    println!("p2: {}", Solution::part2(&bags));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let bags = parse(
            "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );
        assert_eq!(Solution::part1(&bags), 4);
    }

    #[test]
    fn test_part2() {
        let bags = parse(
            "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
        );
        assert_eq!(Solution::part2(&bags), 126);
    }
}
