use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut _input = read_file("input.txt");

    evaluate("9 + 3 * (9 * (9 + 3 + 5 * 3) * 5 * 3 * 5)");

    Ok(())
}

#[derive(Clone, Copy)]
enum OpType { Add, Mul }

#[derive(Clone, Copy)]
enum Token {
    None,
    Op (OpType),
    Number (u32),
    Open,
    Close
}

struct Stack {
    stack : Vec<Token>
}

impl Stack {
    fn new() -> Stack {
        Stack{ stack : Vec::new() }
    }

    fn top(&self) -> Token {
        if let Some(tok) = self.stack.last() {
            *tok
        }
        else {
            Token::None
        }
    }

    fn add_token(&mut self, tok : Token) -> Option<()> {
        match tok {
            Token::None => None,
            Token::Op(_op) => {
                if let Token::Number(_value) = self.top() {
                    self.stack.push(tok);
                    // TODO
                    Some(())
                }
                else {
                    None
                }
            },
            Token::Number(_value) => {
                // TODO
                self.stack.push(tok);
                Some(())
            }
            Token::Open => {
                // TODO
                self.stack.push(tok);
                Some(())
            }
            Token::Close => {
                // TODO
                Some(())
            }
        }
    }
}

fn evaluate(expr : &str) -> Option<u32> {

    //let mut iter = expr.chars();

    //      9 + 3 * (9 * (9 + 3 + 5 * 3) * 5 * 3 * 5)
    //
    //      "9"  push Number                   9
    //      "+"  push Op                    9 +
    //      "3"  pop [+ 9], push 12         12
    //      "*"  push Mul                   12 * 
    //      "("  push Group                 12 * G
    //      "9"  push Number                   12 * G 9
    //      "*"  push Op                    12 * G 9 *
    //      "("  push Group                 12 * G 9 * G
    //      "9"  push Number                   12 * G 9 * G 9
    //      "+"  push Op                    12 * G 9 * G 9 +
    //      "3"  pop [+ 9], push 12         12 * G 9 * G 12
    //      "+"  push Op                    12 * G 9 * G 12 +
    //      "5"  pop [+ 12], push 17        12 * G 9 * G 17
    //      "*"  push Op                    12 * G 9 * G 17 *
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
            '+' => { stack.add_token(Token::Op(OpType::Add))?; },
            '*' => { stack.add_token(Token::Op(OpType::Mul))?; },
            '(' => { stack.add_token(Token::Open)?; },
            ')' => { stack.add_token(Token::Close)?; },
            ' ' => {},
            _ => {
                if ch >= '0' && ch <= '9' {
                    stack.add_token(Token::Number(ch as u32 - '0' as u32))?;
                }
                else {
                    return None;
                }
            }
        }
    }

    if let Token::Number(value) = stack.top() {
        Some(value)
    }
    else {
        None
    }
}

fn read_file(path: &str) -> std::io::Result<Vec::<String>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        v.push(line?);
    }
    Ok(v)
}
