use std::fs;
use std::io::{prelude::*, BufReader};

const PREAMBLE_LENGTH : usize = 25;

fn main() -> std::io::Result<()> {
    let numbers = read_numbers("day9-input.txt")?;

    // Find the first invalid number -- one which is not the sume of any
    // two of the previous 25 numbers.
    if let Some(invalid) = find_invalid_num(&numbers) {

        // Print the invalid number, which is the answer to part 1.
        println!("Invalid number is: {}", invalid);

        // Find a contiguous sequence of two or more numbers that sum to invalid.
        if let Some((begin, end)) = find_sequence(&numbers, invalid) {
            let sequence = &numbers[begin..end];

            // The sum of the minimum and maximum values in this sequence
            // is the answer to part 2.
            let min = find_min(sequence);
            let max = find_max(sequence);
            println!("{} + {} = {}", min, max, min + max);
        }
    }

    Ok(())
}

fn find_min(numbers: &[u64]) -> u64 {
    let mut best = u64::MAX;
    for n in numbers {
        if *n < best {
            best = *n;
        }
    }
    best
}

fn find_max(numbers: &[u64]) -> u64 {
    let mut best = 0;
    for n in numbers {
        if *n > best {
            best = *n;
        }
    }
    best
}

fn find_invalid_num(numbers: &[u64]) -> Option<u64> {
    for i in PREAMBLE_LENGTH..numbers.len() {
        if !is_valid(&numbers[i-PREAMBLE_LENGTH .. i], numbers[i]) {
            return Some(numbers[i]);
        }
    }
    None
}

fn find_sequence(numbers: &[u64], target_sum : u64) -> Option<(usize, usize)> {
    for i in 1..numbers.len() {
        let mut sum = numbers[i - 1];
        if sum < target_sum {
            for j in i..numbers.len() {
                sum += numbers[j];
                if sum == target_sum {
                    return Some((i - 1, j));
                }
                if sum > target_sum {
                    break;
                }
            }
        }
    }
    None
}

fn is_valid(preamble : &[u64], target_sum : u64) -> bool {
    for i in 0..preamble.len() {
        let a = preamble[i];
        if a < target_sum && a != target_sum - a {
            let b = target_sum - a;
            for j in i + 1..preamble.len() {
                if preamble[j] == b {
                    return true;
                }
            }
        }
    }
    false
}

fn read_numbers(path: &str) -> std::io::Result<Vec::<u64>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if let Ok(n) = s.parse::<u64>() { v.push(n); }
    }
    Ok(v)
}
