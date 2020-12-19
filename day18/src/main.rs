use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let input = read_file("input.txt")?;

    let mut total = 0;

    for line in &input {
        if let Some(n) = eval(&line) {
            total += n;
        }
        else {
            println!("Error evaluating expression: {}", &line);
            break;
        }
    }

    println!("total = {}", total);

    Ok(())
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
    stack : Vec<Token>
}

impl Stack {
    fn new() -> Stack {
        Stack{ stack : Vec::new() }
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

fn eval(expr : &str) -> Option<u64> {

    //let mut iter = expr.chars();

    //      9 + 3 * (9 * (9 + 3 + 5 * 3) * 5 * 3 * 5)
    //
    //      "9"  push Number                   9
    //      "+"  push Operator                    9 +
    //      "3"  pop [+ 9], push 12         12
    //      "*"  push Mul                   12 * 
    //      "("  push Group                 12 * G
    //      "9"  push Number                   12 * G 9
    //      "*"  push Operator                    12 * G 9 *
    //      "("  push Group                 12 * G 9 * G
    //      "9"  push Number                   12 * G 9 * G 9
    //      "+"  push Operator                    12 * G 9 * G 9 +
    //      "3"  pop [+ 9], push 12         12 * G 9 * G 12
    //      "+"  push Operator                    12 * G 9 * G 12 +
    //      "5"  pop [+ 12], push 17        12 * G 9 * G 17
    //      "*"  push Operator                    12 * G 9 * G 17 *
    //      "3"  pop [* 17], push 51        12 * G 9 * G 51
    //      ")"  pop 
    //      "*"
    //      "5"
    //      "*"
    //      "3"
    //      "*"
    //      "5"
    //
    
    let mut stack = Stack::new();

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
