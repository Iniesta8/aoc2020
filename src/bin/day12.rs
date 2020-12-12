use std::fs;
use std::time::Instant;

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            let value = value
                .parse::<i32>()
                .expect("syntax error: value not a number");

            match action {
                "N" => Instruction::North(value),
                "S" => Instruction::South(value),
                "E" => Instruction::East(value),
                "W" => Instruction::West(value),
                "L" => Instruction::Left(value),
                "R" => Instruction::Right(value),
                "F" => Instruction::Forward(value),
                _ => panic!("unknown action"),
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

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
}

#[derive(Clone)]
struct Ferry {
    facing: Direction,
    x: i32,
    y: i32,
    instructions: Vec<Instruction>,
    waypoint: (i32, i32),
}

impl Ferry {
    fn with_instructions(input: &str) -> Self {
        Self {
            facing: Direction::East,
            x: 0,
            y: 0,
            instructions: parse(&input),
            waypoint: (10, 1),
        }
    }

    fn dist_from_start(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn drift(&mut self, dir: Direction, dist: i32) {
        match dir {
            Direction::North => self.y += dist,
            Direction::East => self.x += dist,
            Direction::South => self.y -= dist,
            Direction::West => self.x -= dist,
        }
    }

    fn move_forward(&mut self, dist: i32) {
        match self.facing {
            Direction::North => self.y += dist,
            Direction::East => self.x += dist,
            Direction::South => self.y -= dist,
            Direction::West => self.x -= dist,
        }
    }

    fn turn(&mut self, turn: Turn, degrees: i32) {
        match turn {
            Turn::Left => match degrees {
                90 if self.facing == Direction::North => self.facing = Direction::West,
                90 if self.facing == Direction::East => self.facing = Direction::North,
                90 if self.facing == Direction::South => self.facing = Direction::East,
                90 if self.facing == Direction::West => self.facing = Direction::South,
                180 if self.facing == Direction::North => self.facing = Direction::South,
                180 if self.facing == Direction::East => self.facing = Direction::West,
                180 if self.facing == Direction::South => self.facing = Direction::North,
                180 if self.facing == Direction::West => self.facing = Direction::East,
                270 if self.facing == Direction::North => self.facing = Direction::East,
                270 if self.facing == Direction::East => self.facing = Direction::South,
                270 if self.facing == Direction::South => self.facing = Direction::West,
                270 if self.facing == Direction::West => self.facing = Direction::North,
                _ => panic!("unknown turn to the left by {} degrees", degrees),
            },
            Turn::Right => match degrees {
                90 => self.turn(Turn::Left, 270),
                180 => self.turn(Turn::Left, 180),
                270 => self.turn(Turn::Left, 90),
                _ => panic!("unknown turn to the right by {} degrees", degrees),
            },
        }
    }

    fn process_instructions_immediate(&mut self) {
        let instructions = self.instructions.clone();
        for inst in instructions.iter() {
            match inst {
                Instruction::North(dist) => self.drift(Direction::North, *dist),
                Instruction::South(dist) => self.drift(Direction::South, *dist),
                Instruction::East(dist) => self.drift(Direction::East, *dist),
                Instruction::West(dist) => self.drift(Direction::West, *dist),
                Instruction::Left(degrees) => self.turn(Turn::Left, *degrees),
                Instruction::Right(degrees) => self.turn(Turn::Right, *degrees),
                Instruction::Forward(dist) => self.move_forward(*dist),
            }
        }
    }

    fn update_waypoint(&mut self, instr: Instruction) {
        match instr {
            Instruction::North(value) => self.waypoint.1 += value,
            Instruction::South(value) => self.waypoint.1 -= value,
            Instruction::East(value) => self.waypoint.0 += value,
            Instruction::West(value) => self.waypoint.0 -= value,
            Instruction::Left(degrees) => self.rotate_waypoint(Turn::Left, degrees),
            Instruction::Right(degrees) => self.rotate_waypoint(Turn::Right, degrees),
            _ => panic!("unknown waypoint instruction {:?}", instr),
        }
    }

    fn rotate_waypoint(&mut self, turn: Turn, degrees: i32) {
        match turn {
            Turn::Left => match degrees {
                90 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
                180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
                270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
                _ => panic!(
                    "unknown waypoint rotation to the left by {} degrees",
                    degrees
                ),
            },
            Turn::Right => match degrees {
                90 => self.rotate_waypoint(Turn::Left, 270),
                180 => self.rotate_waypoint(Turn::Left, 180),
                270 => self.rotate_waypoint(Turn::Left, 90),
                _ => panic!(
                    "unknown waypoint rotation to the right by {} degrees",
                    degrees
                ),
            },
        }
    }

    fn move_towards_waypoint(&mut self, times: i32) {
        self.x += times * self.waypoint.0;
        self.y += times * self.waypoint.1
    }

    fn process_instructions_relative(&mut self) {
        let instructions = self.instructions.clone();
        for inst in instructions.iter() {
            match inst {
                Instruction::Forward(times) => self.move_towards_waypoint(*times),
                _ => self.update_waypoint(*inst),
            }
        }
    }
}

struct Solution;

impl Solution {
    fn part1(ferry: &mut Ferry) -> i32 {
        ferry.process_instructions_immediate();
        ferry.dist_from_start()
    }

    fn part2(ferry: &mut Ferry) -> i32 {
        ferry.process_instructions_relative();
        ferry.dist_from_start()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day12.txt").expect("File not found!");
    let mut ferry = Ferry::with_instructions(&input);

    let mut ferry2 = ferry.clone();

    let mut timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&mut ferry),
        timer.elapsed()
    );

    timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&mut ferry2),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
F10
N3
F7
R90
F11";
        let mut ferry = Ferry::with_instructions(&input);
        assert_eq!(Solution::part1(&mut ferry), 25);
    }

    #[test]
    fn test_part2() {
        let input = "\
F10
N3
F7
R90
F11";
        let mut ferry = Ferry::with_instructions(&input);
        assert_eq!(Solution::part2(&mut ferry), 286);
    }
}
