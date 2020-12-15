use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut _input = read_file("input.txt");

    // TODO

    Ok(())
}

fn read_file(path: &str) -> std::io::Result<Vec::<String>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        v.push(line?);
    }
    Ok(v)
}
