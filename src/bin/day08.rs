use std::fs;

#[derive(Debug)]
enum Instruction {
    Acc { offset: isize },
    Jmp { offset: isize },
    Nop,
    Halt,
}

#[derive(PartialEq)]
enum Event {
    InfiniteLoop,
    Halted,
}

#[derive(Clone)]
struct GameConsole {
    ip: usize,
    acc: isize,
    memory: Vec<String>,
    processed: Vec<bool>,
    running: bool,
}

impl GameConsole {
    fn from_program(program: &str) -> Self {
        Self {
            ip: 0,
            acc: 0,
            memory: program.lines().map(str::to_owned).collect(),
            processed: vec![false; program.lines().count()],
            running: false,
        }
    }

    fn set_running(&mut self) {
        self.running = true;
    }

    fn halt(&mut self) {
        self.running = false;
    }

    fn _running(&self) -> bool {
        self.running
    }

    fn _run(&mut self) {
        self.set_running();
        while self.running {
            self._step();
        }
    }

    fn run_until_event(&mut self) -> Option<Event> {
        self.set_running();
        while self.running {
            let inst = self.fetch_and_decode();
            if let Some(event) = self.execute(&inst) {
                return Some(event);
            }
        }
        None
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        if self.ip >= self.memory.len() {
            return Instruction::Halt;
        }
        let inst = self.memory[self.ip as usize].clone();

        let token: Vec<&str> = inst.split_whitespace().collect();

        let operation = token[0];
        let operand = token[1].parse::<isize>().unwrap();

        match operation {
            "acc" => Instruction::Acc { offset: operand },
            "jmp" => Instruction::Jmp { offset: operand },
            "nop" => Instruction::Nop,
            _ => panic!("unknown operation {}", operation),
        }
    }

    fn execute(&mut self, inst: &Instruction) -> Option<Event> {
        match inst {
            Instruction::Acc { offset } => {
                self.acc += offset;
                self.ip += 1;
            }
            Instruction::Jmp { offset } => {
                self.ip = (self.ip as isize + offset) as usize;
            }
            Instruction::Nop => {
                self.ip += 1;
            }
            Instruction::Halt => {
                self.halt();
                return Some(Event::Halted);
            }
        }

        if self.processed[self.ip - 1] {
            return Some(Event::InfiniteLoop);
        }
        self.processed[self.ip - 1] = true;

        None
    }

    fn _step(&mut self) {
        let inst = self.fetch_and_decode();
        self.execute(&inst);
    }

    fn change_operation(&mut self, address: usize, op: String) {
        self.memory[address] = op;
    }
}

struct Solution;

impl Solution {
    fn part1(console: &mut GameConsole) -> isize {
        if let Some(Event::InfiniteLoop) = console.run_until_event() {
            return console.acc;
        }
        -1
    }

    fn part2(console: &mut GameConsole) -> isize {
        let mem = console.memory.clone();

        for (address, mem_line) in mem.iter().enumerate() {
            if mem_line.starts_with("nop") || mem_line.starts_with("jmp") {
                let new_mem_line = mem_line.replace("nop", "jmp").replace("jmp", "nop");
                let mut console = console.clone();
                console.change_operation(address, new_mem_line);

                if let Some(Event::Halted) = console.run_until_event() {
                    return console.acc;
                }
            }
        }
        -1
    }
}

fn main() {
    let input = fs::read_to_string("./input/day08.txt").expect("File not found!");
    let mut console1 = GameConsole::from_program(&input);
    let mut console2 = console1.clone();
    println!("p1: {}", Solution::part1(&mut console2));
    println!("p2: {}", Solution::part2(&mut console1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let mut console1 = GameConsole::from_program(
            "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );
        let mut console2 = console1.clone();
        assert_eq!(Solution::part1(&mut console1), 5);
        assert_eq!(Solution::part2(&mut console2), 8);
    }
}
