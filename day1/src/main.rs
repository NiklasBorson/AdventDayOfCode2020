use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {

    // Read the file into a vector of i32.
    let mut v = read_numbers("day1-input.txt")?;

    // Sort the numbers, so we can binary_search later.
    v.sort_unstable();

    // Find the first pair that sums to 2020.
    if let Some((a, b)) = find_pair(&v[..], 2020) {
        println!("{} * {} = {}", a, b, a * b);
    }
    else {
        println!("No pair found that sums to 2020.");
    }

    if let Some((a, b, c)) = find_triple(&v[..], 2020) {
        println!("{} * {} * {} = {}", a, b, c, a * b * c);
    }
    else {
        println!("No triple found that sums to 2020.");
    }

    Ok(())
}

fn find_pair(v: &[i32], sum: i32) -> Option<(i32, i32)> {
    for i in 1..v.len() {

        // Let a be the value before the slice starting at i.
        let a = v[i - 1];

        // Let b the other member of the pair such that a + b == sum.
        let b = sum - a;

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

fn find_triple(v: &[i32], sum : i32) -> Option<(i32, i32, i32)> {
    for i in 1..v.len() {

        // Let a be the value before the slice starting at i.
        let a = v[i - 1];

        // Look for a pair (b, c) that add up to (sum - a).
        if let Some((b, c)) = find_pair(&v[i..], sum - a) {
            return Some((a, b, c));
        }
    }
    None
}

fn read_numbers(path: &str) -> std::io::Result<Vec::<i32>> {
    let mut v = Vec::<i32>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if let Ok(n) = s.parse::<i32>() { v.push(n); }
    }
    Ok(v)
}
