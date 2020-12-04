use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {

    // Read the file into a vector of i32.
    match read_numbers("day1-input.txt") {
        Ok(mut v) => {

            // Sort the numbers, so we can binary_search later.
            v.sort();

            // Find the first pair that sums to 2020.
            if let Some((a, b)) = find_pair(&v[..]) {
                println!("{} * {} = {}", a, b, a * b);
            }
            else {
                println!("No pair found that sums to 2020.");
            }
        },
        Err(e) => {
            println!("Error {} reading input file.", e);
        }
    }
}

fn find_pair(v: &[i32]) -> Option<(i32, i32)> {
    for i in 1usize..v.len() {

        // Let a be the value before the slice starting at i.
        let a = v[i - 1];

        // Let b the other member of the pair such that a + b == 2020.
        let b = 2020 - a;

        if b < a {
            // Be can't possibly exist for values >= a.
            break;
        }

        // If be exists then we're done.
        if let Ok(_i) = v[i..].binary_search(&b) {
            return Some((a, b));
        }
    }
    None
}

fn read_numbers(path: &str) -> std::io::Result<Vec::<i32>> {
    match fs::File::open(path) {
        Ok(file) => {
            let mut v = Vec::<i32>::new();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(s) => {
                        if let Ok(n) = s.parse::<i32>() { v.push(n); }
                    },
                    Err(e) => {
                        return Err(e)
                    }
                }                
            }
            Ok(v)
        },
        Err(e) => Err(e)
    }
}