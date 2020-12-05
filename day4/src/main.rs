use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let v = read_records("day4-input.txt")?;
    let mut valid_count = 0;

    for rec in &v {
        if rec.is_valid() {
            valid_count += 1;
        }
    }

    println!("{} valid records of {}", valid_count, v.len());
    
    Ok(())
}

// Flag bits for each Record field.
const FIELD_BYR : u8 = 0x01; // birth year
const FIELD_IYR : u8 = 0x02; // issue year
const FIELD_EYR : u8 = 0x04; // expiration year
const FIELD_HGT : u8 = 0x08; // height
const FIELD_HCL : u8 = 0x10; // hair color
const FIELD_ECL : u8 = 0x20; // eye color
const FIELD_PID : u8 = 0x40; // passport id
const FIELD_CID : u8 = 0x80; // country id

// Masks of all fields and required fields.
const FIELD_ALL : u8 = 0xFF;
const FIELD_REQUIRED : u8 = FIELD_ALL ^ FIELD_CID;

struct Record {
    field_mask : u8
}

impl Record {
    fn is_empty(&self) -> bool {
        self.field_mask == 0
    }
    fn is_valid(&self) -> bool {
        (self.field_mask & FIELD_REQUIRED) == FIELD_REQUIRED
    }
    fn new() -> Record {
        Record{ field_mask : 0 }
    }
    fn set_field(&mut self, name: &str, _value: &str) {
        match name {
            "byr" => {
                self.field_mask |= FIELD_BYR;
            },
            "iyr" => {
                self.field_mask |= FIELD_IYR;
            },
            "eyr" => {
                self.field_mask |= FIELD_EYR;
            },
            "hgt" => {
                self.field_mask |= FIELD_HGT;
            },
            "hcl" => {
                self.field_mask |= FIELD_HCL;
            },
            "ecl" => {
                self.field_mask |= FIELD_ECL;
            },
            "pid" => {
                self.field_mask |= FIELD_PID;
            },
            "cid" => {
                self.field_mask |= FIELD_CID;
            },
            _ => {
                println!("Warning: unknown field {}", name);
            }
        }
    }
}

fn read_records(path: &str) -> std::io::Result<Vec::<Record>> {
    let mut v = Vec::<Record>::new();
    let mut rec = Record::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if s.len() != 0 {
            for field in s.split(' ') {
                if let Some(i) = field.find(':') {
                    rec.set_field(&field[..i], &field[i + 1..]);                    
                }
            }
        }
        else if !rec.is_empty() {
            v.push(rec);
            rec = Record::new();
        }
    }

    if !rec.is_empty() {
        v.push(rec);
    }

    Ok(v)
}
