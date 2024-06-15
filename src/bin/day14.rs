use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Write(usize, usize),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let inst: Vec<&str> = line.split('=').map(str::trim).collect();
            match inst[0] {
                "mask" => Instruction::Mask(inst[1].to_owned()),
                token if token.starts_with("mem") => {
                    let address: Vec<&str> = token.split(|p| p == '[' || p == ']').collect();
                    let address = address[1].trim().parse::<usize>().unwrap();
                    Instruction::Write(address, inst[1].parse::<usize>().unwrap())
                }
                _ => panic!("unknown instruction: {}", line),
            }
        })
        .collect()
}

// Returns two bitmasks: (AND-mask, OR-mask)
// AND-mask to set 0's (clear 1's)
// OR-mask to set 1's
fn create_masks_v1(raw_mask: &str) -> (usize, usize) {
    raw_mask
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| *c == '0' || *c == '1')
        .fold((usize::MAX, 0_usize), |(and, or), (i, c)| match c {
            '0' => (and & !(1 << i), or),
            '1' => (and, or | (1 << i)),
            _ => (and, or),
        })
}

fn calculate_dest_addresses(raw_mask: &str, raw_address: usize) -> Vec<usize> {
    let mut res = vec![];
    let len = raw_mask.chars().filter(|&c| c == 'X').count();

    for mut i in 0..(1 << len) {
        let mut tmp_mask = raw_mask
            .chars()
            .zip(format!("{:036b}", raw_address).chars())
            .map(|(a, b)| match (a, b) {
                ('X', _) => 'X',
                ('1', _) => '1',
                ('0', val) => val,
                _ => panic!("given bitmask not supported!"),
            })
            .collect::<String>();

        for _ in 0..len {
            tmp_mask = tmp_mask.replacen('X', &format!("{}", i & 1), 1);
            i >>= 1;
        }
        res.push(usize::from_str_radix(&tmp_mask, 2).unwrap());
    }
    res
}

fn write_to_mem(
    version: usize,
    mem: &mut HashMap<usize, usize>,
    address: usize,
    mask: &str,
    new_value: usize,
) {
    match version {
        1 => {
            let (and, or) = create_masks_v1(mask);
            mem.insert(address, (new_value & and) | or);
        }
        2 => {
            for dest_address in calculate_dest_addresses(mask, address) {
                mem.insert(dest_address, new_value);
            }
        }
        _ => (),
    }
}

fn process(version: usize, memory: &mut HashMap<usize, usize>, instructions: &[Instruction]) {
    let mut mask = String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    for inst in instructions.iter() {
        match inst {
            Instruction::Mask(new_mask) => {
                mask = new_mask.clone();
            }
            Instruction::Write(address, new_value) => {
                write_to_mem(version, memory, *address, &mask, *new_value)
            }
        }
    }
}

struct Solution;

impl Solution {
    fn part1(instructions: &[Instruction]) -> usize {
        let mut memory = HashMap::new();
        process(1, &mut memory, instructions);
        memory.values().sum()
    }

    fn part2(instructions: &[Instruction]) -> usize {
        let mut memory = HashMap::new();
        process(2, &mut memory, instructions);
        memory.values().sum()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day14.txt").expect("File not found!");
    let instructions = parse(&input);

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&instructions),
        timer.elapsed()
    );
    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&instructions),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(Solution::part1(&parse(input)), 165);
    }

    #[test]
    fn test_day14_part2() {
        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(Solution::part2(&parse(input)), 208);
    }
}
