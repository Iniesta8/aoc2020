use std::fs;
use std::{collections::HashMap, num::ParseIntError};

type Passport = HashMap<String, String>;

fn parse_data(passport: &String) -> Passport {
    let mut res = Passport::new();
    let data: Vec<String> = passport.split_whitespace().map(|x| x.to_owned()).collect();

    for p in data.iter() {
        let mut tmp = p.split(':');
        let k = tmp.next().unwrap().to_owned();
        let v = tmp.next().unwrap().to_owned();
        res.insert(k.clone(), v.clone());
    }
    res
}

fn passport_valid_p1(passport: &Passport) -> bool {
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn year_valid(year: &String, min: u32, max: u32) -> Result<bool, ParseIntError> {
    year.parse::<u32>().map(|y| min <= y && y <= max)
}

fn hgt_valid(hgt: &String) -> Result<bool, ParseIntError> {
    let len = hgt.len();

    if len < 4 || len > 5 {
        return Ok(false);
    }
    let (val, unit) = hgt.split_at(len - 2);

    match unit {
        "cm" => val.parse::<u32>().map(|h| 150 <= h && h <= 193),
        "in" => val.parse::<u32>().map(|h| 59 <= h && h <= 76),
        _ => Ok(false),
    }
}

fn hcl_valid(hcl: &String) -> bool {
    let len = hcl.len();

    if len != 7 {
        return false;
    }

    let mut hcl = hcl.chars();
    if hcl.next().unwrap() != '#' {
        return false;
    }

    hcl.all(|c| c.is_digit(10) || (c.is_lowercase() && c >= 'a' && c <= 'f'))
}

fn ecl_valid(ecl: &String) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
}

fn pid_valid(pid: &String) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
}

fn passport_valid_p2(passport: &Passport) -> bool {
    if !passport_valid_p1(passport) {
        return false;
    }

    passport.iter().all(|(k, v)| match k.as_str() {
        "byr" => year_valid(v, 1920, 2002).unwrap_or(false),
        "iyr" => year_valid(v, 2010, 2020).unwrap_or(false),
        "eyr" => year_valid(v, 2020, 2030).unwrap_or(false),
        "hgt" => hgt_valid(v).unwrap_or(false),
        "hcl" => hcl_valid(v),
        "ecl" => ecl_valid(v),
        "pid" => pid_valid(v),
        "cid" => true,
        _ => false,
    })
}

struct Solution;

impl Solution {
    fn part1(passports: &Vec<Passport>) -> usize {
        passports.iter().filter(|pp| passport_valid_p1(&pp)).count()
    }

    fn part2(passports: &Vec<Passport>) -> usize {
        passports.iter().filter(|pp| passport_valid_p2(&pp)).count()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day04.txt").unwrap();
    let values: Vec<String> = input.split("\n\n").map(|x| x.to_owned()).collect();
    let passports: Vec<Passport> = values.iter().map(|pdata| parse_data(&pdata)).collect();

    println!("p1: {}", Solution::part1(&passports));
    println!("p2: {}", Solution::part2(&passports));
}
