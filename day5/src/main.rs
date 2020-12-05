use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let passes = read_boarding_passes("day5-input.txt")?;

    let mut max_id : u32 = 0;

    for pass in &passes {
        let id = pass.get_id();
        if id > max_id { max_id = id; }
    }

    println!("max id = {}", max_id);

    Ok(())
}

struct BoardingPass {
    id : u32
}

impl BoardingPass {
    fn get_id(&self) -> u32 { self.id }
    //fn get_row(&self) -> u32 { self.id >> 8 }
    //fn get_col(&self) -> u32 { self.id & 7 }
}

fn parse_boarding_pass(s: &str) -> Option<BoardingPass> {
    let mut row_min : u32 = 0;
    let mut row_lim : u32 = 128;
    let mut col_min : u32 = 0;
    let mut col_lim : u32 = 8;

    for ch in s.chars() {
        match ch {
            'F' => { row_lim = (row_min + row_lim) >> 1; },
            'B' => { row_min = (row_min + row_lim) >> 1; },
            'L' => { col_lim = (col_min + col_lim) >> 1; },
            'R' => { col_min = (col_min + col_lim) >> 1; },
            _ => {
                println!("Error: {} => invalid character '{}'.", &s, ch);
                return None;
            }
        }
    }

    if row_min + 1 != row_lim || col_min + 1 != col_lim {
        println!("Error: {} => row {}..{}, col {}..{}", &s, row_min, row_lim, col_min, col_lim);
        return None;
    }

    Some(BoardingPass{ id: (row_min << 3) | col_min })
}

fn read_boarding_passes(path: &str) -> std::io::Result<Vec::<BoardingPass>> {
    let mut v = Vec::<BoardingPass>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if let Some(pass) = parse_boarding_pass(&s) {
            v.push(pass);
        }
    }
    Ok(v)
}
