use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut valid_count = 0;
    let mut valid_count2 = 0;
    for line in BufReader::new(fs::File::open("day2-input.txt")?).lines() {
        let s = line?;
        if let Some((policy, password)) = parse_line(&s) {
            if is_valid_password(&policy, password) {
                valid_count += 1;
            }
            if is_valid_password2(&policy, password) {
                valid_count2 += 1;
            }
        }
    }
    println!("valid count = {}", valid_count);
    println!("valid count per second policy = {}", valid_count2);
    Ok(())
}

struct PasswordPolicy {
    ch : char,
    min_count : u32,
    max_count : u32
}

fn is_valid_password(policy: &PasswordPolicy, password: &str) -> bool {
    let mut actual_count = 0u32;
    for ch in password.chars() {
        if ch == policy.ch {
            actual_count += 1;
        }
    }
    actual_count >= policy.min_count && actual_count <= policy.max_count
}

fn is_valid_password2(policy: &PasswordPolicy, password: &str) -> bool {
    // In this policy, reinterpret min_count and max_count as one-based character indices.
    let i = (policy.min_count - 1) as usize;
    let j = (policy.max_count - 1) as usize;

    // Get the characters at indices i and j.
    if let Some(ch1) = password.chars().nth(i) {
        if let Some(ch2) = password.chars().nth(j) {

            // Return true if exactly one of the two characters equals policy.ch.
            return (ch1 == policy.ch) != (ch2 == policy.ch);
        }
    }
    false
}

fn string_to_char(s: &str) -> Option<char> {
    for ch in s.chars() {
        return Some(ch);
    }
    None
}

fn split_string(s: &str, delim: char) -> Option<(&str, &str)> {
    let i = s.find(delim)?;
    Some((&s[..i], &s[i + 1..]))
}

fn parse_password_policy(s: &str) -> Option<PasswordPolicy> {

    // Split string of the form "<min>-<max> <ch>" into three fields.
    let (field_min, right) = split_string(s, '-')?;
    let (field_max, field_ch) = split_string(right, ' ')?;

    // Fill in the struct.
    Some(PasswordPolicy{
        ch: string_to_char(field_ch)?,
        min_count: field_min.parse::<u32>().ok()?,
        max_count: field_max.parse::<u32>().ok()?
    })
}

fn parse_line(line: &str) -> Option<(PasswordPolicy, &str)> {
    let (policy, password) = split_string(line, ':')?;
    Some((parse_password_policy(policy.trim())?, password.trim()))
}
