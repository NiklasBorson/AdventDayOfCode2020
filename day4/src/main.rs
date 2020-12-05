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

    fn set_field(&mut self, name: &str, value: &str) {
        match name {
            "byr" => {
                if is_valid_number(value, 1920, 2002) {
                    self.field_mask |= FIELD_BYR;
                }
            },
            "iyr" => {
                if is_valid_number(value, 2010, 2020) {
                    self.field_mask |= FIELD_IYR;
                }
            },
            "eyr" => {
                if is_valid_number(value, 2020, 2030) {
                    self.field_mask |= FIELD_EYR;
                }
            },
            "hgt" => {
                if is_valid_height(value) {
                    self.field_mask |= FIELD_HGT;
                }
            },
            "hcl" => {
                if is_valid_hex_color(value) {
                    self.field_mask |= FIELD_HCL;
                }
            },
            "ecl" => {
                if is_valid_eye_color(value) {
                    self.field_mask |= FIELD_ECL;
                }
            },
            "pid" => {
                if is_valid_passport_id(value) {
                    self.field_mask |= FIELD_PID;
                }
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

fn is_valid_number(s: &str, min_value: u32, max_value: u32) -> bool {
    if let Ok(n) = s.parse::<u32>() {
        return n >= min_value && n <= max_value;
    }
    false
}

fn is_valid_passport_id(s: &str) -> bool {
    let mut digits = 0;
    for ch in s.chars() {
        if ch >= '0' && ch <= '9' {
            digits += 1;
        }
        else {
            return false;
        }
    }
    digits == 9
}

fn is_valid_height(s: &str) -> bool {
    if s.len() > 2 {
        let i = s.len() - 2;
        let units = &s[i..];
        let value = &s[..i];
        return match units {
            "cm" => is_valid_number(value, 150, 193),
            "in" => is_valid_number(value, 59, 76),
            _ => false
        };
    }
    false
}

fn is_hex_digit(ch: char) -> bool {
    (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f')
}

fn is_valid_hex_color(s: &str) -> bool {
    if s.len() != 7 { return false; }
    if &s[..1] != "#" { return false; }
    for ch in s[1..].chars() {
        if !is_hex_digit(ch) { return false; }
    }
    true
}

fn is_valid_eye_color(s: &str) -> bool {
    match s {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
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
