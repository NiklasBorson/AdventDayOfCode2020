use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {

    let mut answers_and : u32 = 0xFFFFFFFF;
    let mut answers_or : u32 = 0;
    let mut in_group = false;

    let mut groups_or = Vec::<u32>::new();
    let mut groups_and = Vec::<u32>::new();

    for line in BufReader::new(fs::File::open("day6-input.txt")?).lines() {
        let s = line?;
        if !s.is_empty() {
            let mut answer_bits : u32 = 0;

            // Set the bit corresponding to each answer, where 'a' is bit 0, etc.
            for ch in s.chars() {
                if ch >= 'a' && ch <= 'z' {
                    let i = (ch as i32) - ('a' as i32);
                    answer_bits |= 1u32 << i;
                }
            }

            answers_or |= answer_bits;
            answers_and &= answer_bits;
            in_group = true;
        }
        else if in_group {
            groups_or.push(answers_or);
            groups_and.push(answers_and);
            answers_and = 0xFFFFFFFF;
            answers_or = 0;
            in_group = false;
        }
    }

    // Add the last group's answers to the vector.
    if in_group {
        groups_or.push(answers_or);
        groups_and.push(answers_and);
    }

    println!("Total for part 1 (*any*): {}", count_answers(&groups_or));
    println!("Total for part 2 (*all*): {}", count_answers(&groups_and));

    Ok(())
}

fn count_answers(groups: &[u32]) -> u32 {
    return groups.iter()
        .map(|n| count_bits(*n))
        .fold(0, |sum,x| sum + x);
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