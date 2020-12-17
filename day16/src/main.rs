use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut input = read_file("input.txt")?;

    // Part 1 - count the total invalid fields.
    let mut sum = 0;
    for ticket in &input.nearby_tickets {
        for &field in ticket {
            if input.valid_field_bits(field) == 0 {
                sum += field;
            }
        }
    }
    println!("{} invalid fields", sum);

    // Part 2
    input.remove_invalid_tickets();
    input.set_input_indices();
    let mut product = 1;
    for field in &input.field_defs {
        if field.name.starts_with("departure") {
            if let Some(i) = field.input_index {
                let val = input.my_ticket[i as usize] as usize;
                println!("'{}' (column {}) = {}", field.name, i, val);
                product *= val;
            }
        }
    }
    println!("Product = {}", product);

    Ok(())
}

#[derive(Clone, Copy)]
struct Range {
    min : u32,
    max : u32
}

impl Range {
    fn new(min : u32, max : u32) -> Range {
        Range{ min, max }
    }
    fn parse(s : &str) -> Option<Range> {
        let i = s.find('-')?;
        let &min = &s[..i].trim().parse::<u32>().ok()?;
        let &max = &s[i + 1..].trim().parse::<u32>().ok()?;
        Some(Range::new(min, max))
    }
    fn includes(&self, val : u32) -> bool {
        val >= self.min && val <= self.max
    }
}

struct FieldDef  {
    name : String,
    first_range : Range,
    second_range : Range,
    input_index : Option<u32>
}

impl FieldDef {
    fn new(name : &str, first_range : Range, second_range : Range) -> FieldDef {
        FieldDef{ name : name.to_string(), first_range, second_range, input_index : None }
    }
    fn parse(s : &str) -> Option<FieldDef> {
        let i = s.find(':')?;
        let name = &s[..i];
        let s = &s[i + 1..].trim();

        const OR : &str = " or ";
        let i = s.find(OR)?;
        let first_range = Range::parse(&s[..i])?;
        let second_range = Range::parse(&s[i + OR.len()..])?;

        Some(FieldDef::new(name, first_range, second_range))
    }
    fn includes(&self, val : u32) -> bool {
        self.first_range.includes(val) || self.second_range.includes(val)
    }
}

struct Input {
    field_defs : Vec<FieldDef>,
    my_ticket : Vec<u32>,
    nearby_tickets : Vec<Vec<u32>>
}

impl Input {
    fn new() -> Input {
        Input{
            field_defs : Vec::new(),
            my_ticket : Vec::new(),
            nearby_tickets : Vec::new()
        }
    }

    fn valid_field_bits(&self, val : u32) -> u32 {
        let mut valid_bits = 0;
        for i in 0..self.field_defs.len() {
            let def = &self.field_defs[i];
            if def.input_index.is_none() && def.includes(val) {
                valid_bits |= 1u32 << i;
            }
        }
        valid_bits
    }

    fn is_valid_ticket(&self, ticket : &[u32]) -> bool {
        for &field in ticket {
            if self.valid_field_bits(field) == 0 {
                return false;
            }
        }
        true
    }

    fn remove_invalid_tickets(&mut self) {
        let mut valid = Vec::new();

        for ticket in &self.nearby_tickets {
            if self.is_valid_ticket(ticket) {
                valid.push(ticket.clone());
            }
        }

        self.nearby_tickets = valid;
    }

    fn set_input_indices(&mut self) {
        let field_count = self.my_ticket.len();
        let all_input_bits = (1u32 << field_count) - 1;
        let mut used_input_bits = 0;

        while used_input_bits != all_input_bits {
            let mut assigned_index = false;

            for input_index in 0..field_count {
                if test_bit(used_input_bits, input_index) {
                    continue;
                }

                // Initially assume this input could go with any field.
                let mut field_bits = all_input_bits;

                for ticket in &self.nearby_tickets {
                    field_bits &= self.valid_field_bits(ticket[input_index]);
                }

                if field_bits == 0 {
                    println!("Can't identify field for input column {}.", input_index);
                    return;
                }

                // If only one bit is set then it can only belong to that field.
                if (field_bits & (field_bits - 1)) == 0 {
                    for i in 0..field_count {
                        if test_bit(field_bits, i) {
                            self.field_defs[i].input_index = Some(input_index as u32);
                        }
                    }
                    used_input_bits |= 1 << input_index;
                    assigned_index = true;
                    break;
                }
            }

            // If we didn't make any progress, exit the loop.
            if !assigned_index {
                println!("Failed to set input indices.");
                return;
            }
        }
    }
}

fn test_bit(bits : u32, index : usize) -> bool {
    ((bits >> index) & 1) != 0
}

fn to_result<T>(opt : Option<T>) -> std::io::Result<T> {
    opt.ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input."))
}

fn parse_ticket(s : &str, fields : &mut Vec<u32>) -> Option<()> {
    for field in s.split(',') {
        fields.push(field.parse::<u32>().ok()?);
    }
    Some(())
}

fn read_file(path: &str) -> std::io::Result<Input> {
    let mut input = Input::new();

    let mut reader = BufReader::new(fs::File::open(path)?);
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        let s = line.trim();
        if !s.is_empty() {
            // Parse the field definition.
            input.field_defs.push(to_result(FieldDef::parse(&s))?);
            line.clear();
        }
        else {
            // Blank line: advance to the next line, and exit the loop.
            line.clear();
            reader.read_line(&mut line)?;
            break;
        }
    }

    // We should now be positioned on the "your ticket" section heading.
    if line.trim() == "your ticket:" {
        // Read the next line, which should contain the fields.
        line.clear();
        reader.read_line(&mut line)?;

        // Parse the ticket.
        to_result(parse_ticket(line.trim(), &mut input.my_ticket))?;

        // Read the next line, which should be blank.
        line.clear();
        reader.read_line(&mut line)?;

        // Advance past the blank line.
        if line.trim().is_empty() {
            line.clear();
            reader.read_line(&mut line)?;
        }
    }

    // We should now be positioned on the "nearby tickets" section heading.
    if line.trim() == "nearby tickets:" {
        while { line.clear(); reader.read_line(&mut line)? } != 0 {
            let mut ticket = Vec::new();
            to_result(parse_ticket(line.trim(), &mut ticket))?;
            input.nearby_tickets.push(ticket);
        }
    }

    Ok(input)
}
