use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {

    // Read the file into a vector of i32.
    let mut v = read_numbers("day1-input.txt");

    // Sort the numbers, so we can binary_search later.
    v.sort();

    // Find the first pair that sums to 2020.
    find_pair(&v[..]);
}

fn find_pair(v: &[i32]) {
    for i in 1usize..v.len() {
        let a = v[i - 1];
        let b = 2020 -a;
        if has_number(&v[i..], b) {
            println!("{} * {} = {}", a, b, a * b);
            break;
        }
    }
}

fn has_number(v: &[i32], n: i32) -> bool {
    match v.binary_search(&n) {
        Ok(_i) => true,
        Err(_e) => false
    }
}

fn read_numbers(path: &str) -> Vec::<i32> {
    let mut v = Vec::<i32>::new();

    if let Ok(file) = fs::File::open("day1-input.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(s) = line {
                if let Ok(n) = s.parse::<i32>() {
                    v.push(n);
                }
                else {
                    println!("{} is not a number.", s);
                }
            }
        }
    }
    else {
        println!("Error opening {}", path);
    }

    v
}
