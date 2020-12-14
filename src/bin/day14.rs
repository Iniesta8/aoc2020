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
            let inst: Vec<&str> = line.split("=").map(str::trim).collect();
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
// AND-mask to set 0's
// OR-mask to set 1's
fn create_masks(raw_mask: &String) -> (usize, usize) {
    raw_mask
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| *c == '0' || *c == '1')
        .fold((usize::MAX, 0 as usize), |(and, or), (i, c)| match c {
            '0' => (and & !(1 << i), or),
            '1' => (and, or | (1 << i)),
            _ => (and, or),
        })
}

fn write_to_mem(mem: &mut Vec<usize>, address: usize, mask: &String, new_value: usize) {
    let mem_size = mem.len();

    if address >= mem_size {
        let mut new_mem = vec![0; address - mem_size + 1];
        mem.append(&mut new_mem);
    }

    let (and, or) = create_masks(&mask);

    let mut val_to_write = new_value;
    val_to_write &= and;
    val_to_write |= or;

    mem[address] = val_to_write;
}

fn process(mut memory: &mut Vec<usize>, instructions: &Vec<Instruction>) {
    let mut mask = String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

    // dbg!(&instructions);

    for inst in instructions.iter() {
        // dbg!(&memory);
        match inst {
            Instruction::Mask(new_mask) => {
                mask = new_mask.clone();
            }
            Instruction::Write(address, new_value) => {
                write_to_mem(&mut memory, *address, &mask, *new_value)
            }
        }
    }
}

struct Solution;

impl Solution {
    fn part1(instructions: &Vec<Instruction>) -> usize {
        let mut memory = vec![0];
        process(&mut memory, &instructions);
        memory.iter().sum()
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
    // let timer = Instant::now();
    // println!(
    // "p2: {} (runtime: {:?})",
    // Solution::part2(&notes),
    // timer.elapsed()
    // );
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
        assert_eq!(Solution::part1(&parse(&input)), 165);
    }

    // #[test]
    // fn test_day13_part2() {
    // assert_eq!(Solution::part2(&parse(&"0\n7,13,x,x,59,x,31,19")), 1068781);
    // assert_eq!(Solution::part2(&parse(&"0\n17,x,13,19")), 3417);
    // assert_eq!(Solution::part2(&parse(&"0\n67,7,59,61")), 754018);
    // assert_eq!(Solution::part2(&parse(&"0\n67,x,7,59,61")), 779210);
    // assert_eq!(Solution::part2(&parse(&"0\n67,7,x,59,61")), 1261476);
    // assert_eq!(Solution::part2(&parse(&"0\n1789,37,47,1889")), 1202161486);
    //
    // assert_eq!(
    // Solution::part2_alt(&parse(&"0\n7,13,x,x,59,x,31,19")),
    // 1068781
    // );
    // }
}
