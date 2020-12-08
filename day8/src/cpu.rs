use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Copy, Clone)]
pub enum OpCode {
    Acc,
    Jmp,
    Nop
}

impl OpCode {
    fn parse(s : &str) -> Option<OpCode> {
        match s {
            "acc" => Some(OpCode::Acc),
            "jmp" => Some(OpCode::Jmp),
            "nop" => Some(OpCode::Nop),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub op_code : OpCode,
    pub operand : i32
}

impl Instruction {
    fn parse(line : &str) -> Option<Instruction> {
        let i = line.find(' ')?;
        let op_code = OpCode::parse(&line[0..i])?;
        let operand = line[i + 1..].parse::<i32>().ok()?;
        Some(Instruction{ op_code : op_code, operand : operand })
    }
}

pub struct Computer {
    instructions : Vec<Instruction>,
    instruction_index : i32,
    accumulator : i32
}

impl Computer {
    pub fn load_program(path : &str) -> std::io::Result<Computer> {
        let mut instructions = Vec::new();
        for line in BufReader::new(fs::File::open(path)?).lines() {
            if let Some(instruction) = Instruction::parse(&line?) {
                instructions.push(instruction);
            }
        }
        Ok(Computer{ 
            instructions : instructions, 
            instruction_index : 0, 
            accumulator : 0 
        })
    }

    pub fn get_instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn get_instruction_index(&self) -> usize {
        self.instruction_index as usize
    }

    pub fn get_accumulator(&self) -> i32 {
        self.accumulator
    }

    pub fn in_bounds(&self) -> bool {
        let index = self.instruction_index;
        index >= 0 && (index as usize) < self.instructions.len()
    }

    pub fn step(&mut self) {
        if self.in_bounds() {
            let index = self.instruction_index;
            let instruction = self.instructions[index as usize];
            self.instruction_index = index + match instruction.op_code {
                OpCode::Acc => { self.accumulator += instruction.operand; 1 },
                OpCode::Jmp => instruction.operand,
                OpCode::Nop => 1
            };
        }
    }
}
