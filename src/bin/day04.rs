use std::collections::HashMap;
use std::fs;

type Passport = HashMap<String, String>;

fn parse_batch(batch: &str) -> Passport {
    let mut pp = Passport::new();
    let fields: Vec<String> = batch.split_whitespace().map(str::to_owned).collect();

    for f in fields.iter() {
        let mut tmp = f.split(':');
        let k = tmp.next().unwrap().to_owned().clone();
        let v = tmp.next().unwrap().to_owned().clone();
        pp.insert(k, v);
    }
    pp
}

fn passport_valid_p1(passport: &Passport) -> bool {
    let len = passport.len();
    (7 <= len || len <= 8)
        && ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| passport.contains_key(*k))
}

fn year_valid(year: &str, min: u32, max: u32) -> bool {
    (min..=max).contains(&year.parse::<u32>().unwrap_or_default())
}

fn hgt_valid(height: &str) -> bool {
    let (val, unit) = height.split_at(height.len() - 2);
    matches!(
        (val.parse::<u32>(), unit),
        (Ok(150..=193), "cm") | (Ok(59..=76), "in")
    )
}

fn hcl_valid(hcl: &str) -> bool {
    hcl.len() == 7 && hcl.starts_with('#') && hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

fn ecl_valid(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
}

fn pid_valid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
}

fn passport_valid_p2(passport: &Passport) -> bool {
    if !passport_valid_p1(passport) {
        return false;
    }

    passport.iter().all(|(k, v)| match k.as_str() {
        "byr" => year_valid(v, 1920, 2002),
        "iyr" => year_valid(v, 2010, 2020),
        "eyr" => year_valid(v, 2020, 2030),
        "hgt" => hgt_valid(v),
        "hcl" => hcl_valid(v),
        "ecl" => ecl_valid(v),
        "pid" => pid_valid(v),
        "cid" => true,
        _ => false,
    })
}

struct Solution;

impl Solution {
    fn part1(passports: &[Passport]) -> usize {
        passports.iter().filter(|pp| passport_valid_p1(pp)).count()
    }

    fn part2(passports: &[Passport]) -> usize {
        passports.iter().filter(|pp| passport_valid_p2(pp)).count()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day04.txt").expect("File not found!");
    let batch: Vec<&str> = input.split("\n\n").collect();
    let passports: Vec<Passport> = batch.into_iter().map(parse_batch).collect();

    println!("p1: {}", Solution::part1(&passports));
    println!("p2: {}", Solution::part2(&passports));
}
