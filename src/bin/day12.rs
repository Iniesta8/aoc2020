use std::fs;
use std::time::Instant;

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            let value = value.parse().expect("value not a number");

            match action {
                "N" => Instruction::North(value),
                "S" => Instruction::South(value),
                "E" => Instruction::East(value),
                "W" => Instruction::West(value),
                "L" => Instruction::Left(value),
                "R" => Instruction::Right(value),
                "F" => Instruction::Forward(value),
                _ => panic!("unknown action: {}", action),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Clone, Debug)]
struct Ferry {
    position: (i32, i32),
    facing: i32, // 0: North, 90: East, 180: South, 270: West
    waypoint: (i32, i32),
}

impl Ferry {
    fn new() -> Self {
        Self {
            position: (0, 0),
            facing: 90,
            waypoint: (10, 1),
        }
    }

    fn dist_from_start(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn move_forward(&mut self, dist: i32) {
        match self.facing {
            0 => self.position.1 += dist,
            90 => self.position.0 += dist,
            180 => self.position.1 -= dist,
            270 => self.position.0 -= dist,
            _ => panic!("unknown current facing: {}", self.facing),
        }
    }

    fn turn(&mut self, degrees: i32) {
        self.facing = (self.facing + degrees).rem_euclid(360);
    }

    fn process_instructions_immediate(&mut self, instructions: &[Instruction]) {
        for &instr in instructions {
            match instr {
                Instruction::North(dist) => self.position.1 += dist,
                Instruction::South(dist) => self.position.1 -= dist,
                Instruction::East(dist) => self.position.0 += dist,
                Instruction::West(dist) => self.position.0 -= dist,
                Instruction::Left(degrees) => self.turn(-degrees),
                Instruction::Right(degrees) => self.turn(degrees),
                Instruction::Forward(dist) => self.move_forward(dist),
            }
        }
    }

    fn rotate_waypoint(&mut self, degrees: i32) {
        match degrees.rem_euclid(360) {
            90 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            270 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            _ => panic!("unknown waypoint rotation by {} degrees", degrees),
        }
    }

    fn move_towards_waypoint(&mut self, times: i32) {
        self.position.0 += times * self.waypoint.0;
        self.position.1 += times * self.waypoint.1;
    }

    fn process_instructions_relative(&mut self, instructions: &[Instruction]) {
        for &instr in instructions {
            match instr {
                Instruction::North(value) => self.waypoint.1 += value,
                Instruction::South(value) => self.waypoint.1 -= value,
                Instruction::East(value) => self.waypoint.0 += value,
                Instruction::West(value) => self.waypoint.0 -= value,
                Instruction::Left(degrees) => self.rotate_waypoint(-degrees),
                Instruction::Right(degrees) => self.rotate_waypoint(degrees),
                Instruction::Forward(times) => self.move_towards_waypoint(times),
            }
        }
    }
}

struct Solution;

impl Solution {
    fn part1(ferry: &mut Ferry, instructions: &[Instruction]) -> i32 {
        ferry.process_instructions_immediate(instructions);
        ferry.dist_from_start()
    }

    fn part2(ferry: &mut Ferry, instructions: &[Instruction]) -> i32 {
        ferry.process_instructions_relative(instructions);
        ferry.dist_from_start()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day12.txt").expect("File not found!");
    let instructions = parse(&input);

    let mut ferry = Ferry::new();
    let mut ferry2 = ferry.clone();

    let mut timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&mut ferry, &instructions),
        timer.elapsed()
    );

    timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&mut ferry2, &instructions),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input = "\
F10
N3
F7
R90
F11";
        let mut ferry = Ferry::new();
        assert_eq!(Solution::part1(&mut ferry, &parse(input)), 25);
    }

    #[test]
    fn test_day12_part2() {
        let input = "\
F10
N3
F7
R90
F11";
        let mut ferry = Ferry::new();
        assert_eq!(Solution::part2(&mut ferry, &parse(input)), 286);
    }
}
