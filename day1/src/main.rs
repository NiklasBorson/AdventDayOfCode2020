use std::fs;
use std::io::{prelude::*, BufReader};

fn main() {

    // Read the file into a vector of i32.
    let mut v = read_numbers("day1-input.txt");

    // Sort the numbers, so we can binary_search later.
    v.sort();

    find_pair(&v[..]);
}

fn find_pair(v: &[i32]) {
    for a in v {
        // Compute b such that a + b == 2020
        let b = 2020 - a;

        // If b exists then we have our pair.
        if has_number(v, b) {
            println!("{} * {} = {}", a, b, a * b);
            break;
        }
    }
}

fn has_number(v: &[i32], n: i32) -> bool {
    let mut begin = 0;
    let mut end = v.len();

    while begin < end {
        let i = (begin + end) / 2;
        let mid = v[i];
        if n < mid { end = i; }
        else if n > mid { begin = i + 1; }
        else { return true; }
    }
    false
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
