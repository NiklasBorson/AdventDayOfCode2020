use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {

    let mut groups = Vec::<u32>::new();
    let mut answer_bits : u32 = 0;
    for line in BufReader::new(fs::File::open("day6-input.txt")?).lines() {
        let s = line?;
        if !s.is_empty() {
            // Set the bit corresponding to each answer, where 'a' is bit 0, etc.
            for ch in s.chars() {
                if ch >= 'a' && ch <= 'z' {
                    let i = (ch as i32) - ('a' as i32);
                    answer_bits |= 1u32 << i;
                }
            }
        }
        else {
            // Blank line: end of a group.
            if answer_bits != 0 {
                groups.push(answer_bits);
                answer_bits = 0;
            }
        }
    }

    // Add the last group's answers to the vector.
    if answer_bits != 0 {
        groups.push(answer_bits);
    }

    // Procedural way of adding up total answers.
    let mut total_answers = 0;
    for answers in &groups[..] {
        total_answers += count_bits(*answers);
    }
    println!("total answers = {}", total_answers);

    // Add up total answers using higher-order functions.
    let total = groups.iter()
        .map(|n| count_bits(*n))
        .fold(0, |sum,x| sum + x);
    println!("total answers = {}", total);

    Ok(())
}

fn count_bits(n : u32) -> u32 {
    let mut bits = n;
    let mut c = 0;
    while bits != 0 {
        c += 1;
        bits &= bits - 1;
    }
    return c;
}