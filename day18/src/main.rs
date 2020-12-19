use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let input = read_file("input.txt")?;

    if let Some(total) = get_total(&input, false) {
        println!("Part 1 total = {}", total);
    }

    if let Some(total) = get_total(&input, true) {
        println!("Part 2 total = {}", total);
    }

    Ok(())
}

fn get_total(input : &[String], use_precedence : bool) -> Option<u64> {
    let mut total = 0;
    for line in input {
        if let Some(n) = eval(&line, use_precedence) {
            total += n;
        }
        else {
            println!("Error evaluating expression: {}", &line);
            return None;
        }
    }
    Some(total)
}

#[derive(Clone, Copy, PartialEq)]
enum OpType { Add, Mul }

use OpType::*;

impl OpType {
    fn eval(&self, lhs : u64, rhs : u64) -> u64 {
        match self {
            Add => lhs + rhs,
            Mul => lhs * rhs
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Token {
    None,
    Operator (OpType),
    Number (u64),
    Open,
    Close
}

use Token::Operator;
use Token::Number;
use Token::Open;
use Token::Close;

struct Stack {
    stack : Vec<Token>,
    use_precedence : bool
}

impl Stack {
    fn new(use_precedence : bool) -> Stack {
        Stack{ stack : Vec::new(), use_precedence }
    }

    fn frame(&self, index : usize) -> Token {
        let c = self.stack.len();
        if index < c {
            self.stack[c - 1 - index]
        }
        else {
            Token::None
        }
    }

    fn top(&self) -> Token {
        self.frame(0)
    }

    fn result(&self) -> Option<u64> {
        if self.stack.len() == 1 {
            if let Number(n) = self.stack[0] {
                return Some(n);
            }
        }
        None
    }

    fn add_token(&mut self, tok : Token) -> Option<()> {
        match tok {
            Token::None =>
                None,
            Operator(_op) => {
                self.stack.push(tok);
                Some(())
            },
            Number(value) => {
                let value = self.reduce(value)?;
                self.stack.push(Token::Number(value));
                Some(())
            }
            Open => {
                self.stack.push(tok);
                Some(())
            }
            Close => {
                let value = self.close_group()?;
                let value = self.reduce(value)?;
                self.stack.push(Token::Number(value));
                Some(())
            }
        }
    }

    fn close_group(&mut self) -> Option<u64> {
        if let Number(value) = self.top() {
            self.stack.pop();
            if self.top() == Open {
                self.stack.pop();
                return Some(value);
            }
        }
        None
    }

    fn reduce(&mut self, value : u64) -> Option<u64> {
        let mut rhs = value;
        while let Operator(op) = self.top() {
            self.stack.pop();
            if let Number(lhs) = self.top() {
                rhs = op.eval(lhs, rhs);
                self.stack.pop();
            }
            else {
                return None;
            }
        }
        Some(rhs)
    }
}

fn eval(expr : &str, use_precedence : bool) -> Option<u64> {
    let mut stack = Stack::new(use_precedence);

    for ch in expr.chars() {
        match ch {
            '+' => { stack.add_token(Operator(OpType::Add))?; },
            '*' => { stack.add_token(Operator(OpType::Mul))?; },
            '(' => { stack.add_token(Open)?; },
            ')' => { stack.add_token(Close)?; },
            ' ' => {},
            _ => {
                if ch >= '0' && ch <= '9' {
                    stack.add_token(Number(ch as u64 - '0' as u64))?;
                }
                else {
                    return None;
                }
            }
        }
    }

    stack.result()
}

fn read_file(path: &str) -> std::io::Result<Vec::<String>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        v.push(line?);
    }
    Ok(v)
}
