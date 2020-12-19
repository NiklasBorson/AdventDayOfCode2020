use std::fs;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let (_grammar, _input) = read_file("input.txt")?;

    // TODO

    Ok(())
}

enum Rule {
    Terminal(char),
    Sequence(Vec<u32>),
    Choice(Vec<u32>, Vec<u32>)
}
use Rule::*;

impl Rule {
    fn parse(input : &str) -> Option<Rule> {
        let mut chars = input.chars();
        let first_char = chars.next()?;
        if first_char == '\"' {
            let ch = chars.next()?;
            Some(Terminal(ch))
        }
        else if let Some(i) = input.find('|') {
            Some(Choice(
                Rule::parse_sequence(&input[0..i])?,
                Rule::parse_sequence(&input[i + 1..])?
            ))
        }
        else {
            Some(Sequence(Rule::parse_sequence(input)?))
        }
    }

    fn parse_sequence(input : &str) -> Option<Vec<u32>> {
        let mut vec = Vec::new();
        for s in input.trim().split(' ') {
            vec.push(s.parse::<u32>().ok()?);
        }
        Some(vec)
    }
}

struct Grammar {
    rules : HashMap<u32, Rule>
}

impl Grammar {
    fn new() -> Grammar {
        Grammar{ rules : HashMap::new() }
    }
    fn add_rule(&mut self, input : &str) -> Option<()> {
        // Get the rule id.
        let i = input.find(':')?;
        let id = input[0..i].parse::<u32>().ok()?;

        // Parse the rule.
        let rule = Rule::parse(input[i + 1..].trim())?;

        // Add the rule to the hashmap.
        self.rules.insert(id, rule);
        Some(())
    }
}

fn make_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
} 

fn read_file(path: &str) -> std::io::Result<(Grammar, Vec::<String>)> {
    let mut grammar = Grammar::new();
    let mut input = Vec::new();
    let mut in_grammar = true;

    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if s.is_empty() {
            in_grammar = false;
        }
        else if in_grammar {
            if grammar.add_rule(&s) == None {
                return Err(make_error(&format!("Invalid rule: {}", &s)));
            }
        }
        else {
            input.push(s);
        }
    }
    Ok((grammar, input))
}
