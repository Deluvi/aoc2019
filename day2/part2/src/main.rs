use std::io::{stdin, Read};
use std::str::FromStr;

pub struct IntCode {
    program: Vec<u32>,
    current_position: usize,
}

impl IntCode {
    pub fn from_intcode_str(intcode_str: &str, noun: u32, verb: u32) -> IntCode {
        let mut program: Vec<u32> = intcode_str
            .split(',')
            .map(|string| u32::from_str(string.trim()).expect("the string is not an integer"))
            .collect();
        program[1] = noun;
        program[2] = verb;
        IntCode::new(program)
    }
    pub fn new(program: Vec<u32>) -> IntCode {
        IntCode {
            program,
            current_position: 0,
        }
    }

    pub fn next_step(&mut self) -> bool {
        let opcode = self.program[self.current_position];
        match opcode {
            1 => {
                let result = self.program[self.program[self.current_position + 1] as usize]
                    + self.program[self.program[self.current_position + 2] as usize];
                let address = self.program[self.current_position + 3] as usize;
                self.program[address] = result;
            }
            2 => {
                let result = self.program[self.program[self.current_position + 1] as usize]
                    * self.program[self.program[self.current_position + 2] as usize];
                let address = self.program[self.current_position + 3] as usize;
                self.program[address] = result;
            }
            99 => {
                return true;
            }
            _ => panic!("Invalid opcode"),
        };
        self.current_position += 4;
        false
    }

    pub fn get_vec(&self) -> &[u32] {
        &self.program
    }

    pub fn solve_intcode(&mut self) -> u32 {
        while !self.next_step() {}
        self.program[0]
    }
}

fn main() {
    let stdin = stdin();
    let mut intcode_string = String::new();
    stdin.lock().read_to_string(&mut intcode_string).unwrap();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = IntCode::from_intcode_str(&intcode_string, noun, verb);
            if intcode.solve_intcode() == 19690720 {
                println!("The noun is {} and the verb is {}.", noun, verb);
                println!("The answer is {}", (100 * noun) + verb);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_intcode_interpreter() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut intcode = IntCode::new(program);
        assert_eq!(intcode.solve_intcode(), 3500);
    }
}
