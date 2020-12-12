use std::fs;

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

#[derive(PartialEq)]
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

struct Ferry {
    facing: Direction,
    x: i32,
    y: i32,
    instructions: Vec<Instruction>,
}

impl Ferry {
    fn with_instructions(input: &str) -> Self {
        Self {
            facing: Direction::East,
            x: 0,
            y: 0,
            instructions: parse(&input),
        }
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

    fn process_instructions(&mut self) {
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

    fn dist_from_start(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct Solution;

impl Solution {
    fn part1(ferry: &mut Ferry) -> i32 {
        ferry.process_instructions();
        ferry.dist_from_start()
    }
}

fn main() {
    let input = fs::read_to_string("./input/day12.txt").expect("File not found!");
    let mut ferry = Ferry::with_instructions(&input);

    println!("p1: {}", Solution::part1(&mut ferry));
}
