use super::grammar::*;
use Rule::*;
use std::fs;
use std::io::{prelude::*, BufWriter};

const START_STATE : usize = 0;
const END_STATE : usize = 1;

pub struct Nfa{
    pub state_count : usize,
    pub transitions : Vec<(usize, Token, usize)>,
}

impl Nfa{
    pub fn new(rules : &[Rule]) -> Nfa {

        // Create an NFA with two states for each rule:
        //  - initial state at rule_index * 2
        //  - final state at rule_index * 2 + 1
        let mut nda = Nfa{
            state_count : rules.len() * 2,
            transitions : Vec::new()
        };

        // Add state transitions for each rule.
        for rule_num in 0..rules.len() {
            let initial_state = rule_num * 2;
            let final_state = initial_state + 1;

            match &rules[rule_num] {
                Void => {
                },
                Terminal(token) => {
                    nda.add_transition(initial_state, *token, final_state);
                },
                Sequence(v) => {
                    nda.add_sequence(initial_state, final_state, &v);
                },
                Choice(a, b) => {
                    nda.add_sequence(initial_state, final_state, &a);
                    nda.add_sequence(initial_state, final_state, &b);
                }
            }
        }

        nda.transitions.sort_unstable();

        nda
    }

    fn add_transition(&mut self, from : usize, token : Token, to : usize) {
        self.transitions.push(( from, token, to ));
    }

    fn add_sequence(&mut self, initial_state : usize, final_state : usize, sequence : &[usize]) {
        let mut last_state = initial_state;
        for &rule_num in sequence {
            // Add a transition from the last state to the rule's initial state.
            let rule_initial_state = rule_num * 2;
            self.add_transition(last_state, Token::Nil, rule_initial_state);

            // We will transition from the rule's final state to whatever is next.
            last_state = rule_initial_state + 1;
        }

        // Add a transition from the last element's final state to the final state of the sequence.
        self.add_transition(last_state, Token::Nil, final_state);
    }

    pub fn write_transitions(&self, path : &str) -> std::io::Result<()> {
        let mut writer = BufWriter::new(fs::File::create(path)?);
        for &(from, token, to) in &self.transitions {
            let line = format!(
                "{}, {}, {}\n",
                from,
                match token { Token::A => "a", Token::B => "b", _ => "" },
                to
            );
            writer.write(line.as_bytes())?;
        }
        Ok(())
    }

    pub fn is_match(&self, input : &str) -> bool {
        let chars : Vec<char> = input.chars().collect();
        self.match_rule(START_STATE, &chars)
    }

    fn match_rule(&self, state_index : usize, input : &[char]) -> bool {
        if input.is_empty() {
            self.match_end(state_index)
        }
        else {
            let t = match input[0] {
                'a' => Token::A,
                'b' => Token::B,
                _ => { return false; }
            };
            for &(from, token, to) in self.find_transitions(state_index) {
                assert_eq!(from, state_index);
                if token == t && self.match_rule(to, &input[1..]) {
                    return true;
                }
                else if token == Token::Nil && self.match_rule(to, input) {
                    return true;
                }
            }
            false
        }
    }

    fn match_end(&self, state_index : usize) -> bool {
        if state_index == END_STATE {
            return true;
        }
        for &(from, token, to) in self.find_transitions(state_index) {
            assert_eq!(state_index, from);
            if token == Token::Nil && self.match_end(to) {
                return true;
            }
        }
        false
    }

    fn find_transitions(&self, state_index : usize) -> &[(usize, Token, usize)] {
        let v = &self.transitions[..];
        let mut begin = 0;
        let mut end = v.len();
        while begin < end {
            let i = (begin + end) / 2;
            let n = v[i].0;            
            if state_index > n {
                begin = i + 1;
            }
            else if state_index < n {
                end = i;
            }
            else {
                begin = i;
                while begin > 0 && v[begin - 1].0 == state_index {
                    begin -= 1;
                }
                end = i + 1;
                while end < v.len() && v[end].0 == state_index {
                    end += 1;
                }
                break;
            }
        }
        &v[begin..end]
    }
}

