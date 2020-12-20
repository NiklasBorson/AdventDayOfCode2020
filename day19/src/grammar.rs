use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Nil,
    A,
    B
}

#[derive(Clone)]
pub enum Rule {
    Void,
    Terminal(Token),
    Sequence(Vec<usize>),
    Choice(Vec<usize>, Vec<usize>)
}
use Rule::*;

impl Rule {
    fn parse(input : &str) -> Option<Rule> {
        let mut chars = input.chars();
        let first_char = chars.next()?;
        if first_char == '\"' {
            match chars.next()? {
                'a' => Some(Terminal(Token::A)),
                'b' => Some(Terminal(Token::B)),
                _ => None
            }
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

    fn parse_sequence(input : &str) -> Option<Vec<usize>> {
        let mut vec = Vec::new();
        for s in input.trim().split(' ') {
            vec.push(s.parse::<usize>().ok()?);
        }
        if vec.is_empty() { None } else { Some(vec) }
    }
}

struct Grammar {
    rules : Vec<Rule>
}

impl Grammar {
    fn new() -> Grammar {
        Grammar{ rules : Vec::new() }
    }
    fn add_rule(&mut self, input : &str) -> Option<()> {
        // Get the rule id.
        let i = input.find(':')?;
        let rule_num = input[0..i].parse::<usize>().ok()?;

        // Parse the rule.
        let rule = Rule::parse(input[i + 1..].trim())?;

        if self.rules.len() <= rule_num {
            self.rules.resize(rule_num + 1, Rule::Void);
        }

        self.rules[rule_num] = rule;
        Some(())
    }
}

fn make_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
} 

pub fn read_file(path: &str) -> std::io::Result<(Vec<Rule>, Vec::<String>)> {
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
    Ok((grammar.rules, input))
}
