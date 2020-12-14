use std::fs;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let code = read_file("input.txt")?;

    part1(&code);

    Ok(())
}

fn part1(code : &[Instruction]) {
    let mut computer = Computer::new();

    for inst in code {
        computer.exec(inst);
    }

    let mut sum = 0;
    for (_address, value) in computer.memory {
        sum += value;
    }

    println!("Sum of memory = {}", sum);
}

enum Instruction {
    Mask {
        zero_bits : u64,
        one_bits : u64
    },
    Mem {
        address : u64,
        value : u64
    }
}

struct Computer {
    zero_bits : u64,
    one_bits : u64,
    memory : HashMap<u64, u64>
}

impl Computer {
    fn new() -> Computer {
        Computer{
            zero_bits : u64::MAX,
            one_bits : 0,
            memory : HashMap::new()
        }
    }

    fn exec(&mut self, inst : &Instruction) {
        match inst {
            Instruction::Mask{ zero_bits, one_bits } => {
                self.zero_bits = *zero_bits;
                self.one_bits = *one_bits;
            },
            Instruction::Mem{ address, value} => {
                let val = (*value & !self.zero_bits) | self.one_bits;
                self.memory.insert(*address, val);
            }
        }
    }
}

fn parse_instruction(line : &str) -> Option<Instruction> {
    const MASK : &str = "mask = ";
    const MEM1 : &str = "mem[";
    const MEM2 : &str = "] = ";

    if line.len() >= MASK.len() && &line[..MASK.len()] == MASK {
        let mut zero_bits = 0;
        let mut one_bits = 0;
        for ch in line[MASK.len()..].chars() {
            zero_bits <<= 1;
            one_bits <<= 1;
            match ch {
                'X' => {},
                '0' => { zero_bits |= 1; },
                '1' => { one_bits |= 1; },
                _ => {
                    println!("Invalid mask: {}", line);
                    return None;
                }
            }
        }
        return Some(Instruction::Mask{ zero_bits, one_bits });
    }
    else if line.len() >= MEM1.len() && &line[..MEM1.len()] == MEM1 {
        if let Some(i) = line.find(MEM2) {
            let address_field = &line[MEM1.len()..i];
            let value_field = &line[i + MEM2.len()..];
            if let Ok(address) = address_field.parse() {
                if let Ok(value) = value_field.parse() {
                    return Some(Instruction::Mem{ address, value });
                }
            }
        }
    }
    println!("Invalid instruction: {}", line);
    None
}

fn read_file(path: &str) -> std::io::Result<Vec::<Instruction>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        if let Some(inst) = parse_instruction(&line?) {
            v.push(inst);
        }
    }
    Ok(v)
}
