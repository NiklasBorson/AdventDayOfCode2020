use super::grammar::*;
use Rule::*;
use std::fs;
use std::io::{prelude::*, BufWriter};

const START_STATE : usize = 0;
const END_STATE : usize = 1;

pub struct Nfa{
    pub state_count : usize,
    pub transitions : Vec<(usize, Token, usize)>,
    pub transition_ranges : Vec<(usize, usize)>
}

impl Nfa{
    pub fn new(rules : &[Rule]) -> Nfa {

        // Initialize the NFA with no transitions and two states:
        // START_STATE (0) and END_STATE (1).
        let mut nfa = Nfa{
            state_count : 2,
            transitions : Vec::new(),
            transition_ranges : Vec::new()
        };

        // Recursively add states and transitions starting with rule 0.
        let end_state = nfa.add_states(rules, 0, START_STATE);

        // Add a transition from the final state of the rule 0 production to END_STATE.
        nfa.add_transition(end_state, Token::Nil, END_STATE);

        // Sort and optimize the transitions.
        nfa.finalize();

        nfa
    }

    fn add_states(&mut self, rules : &[Rule], rule_index : usize, prev_state : usize) -> usize {
        match &rules[rule_index] {
            Void => {
                prev_state
            },
            Terminal(token) => {
                // Add a new state.
                let new_state = self.state_count;
                self.state_count += 1;

                // Add a transition from prev_state to the new state.
                self.add_transition(prev_state, *token, new_state);

                // The new state is the end state of this production.
                new_state
            },
            Sequence(v) => {
                // Add states for a sequence, and return the end state of
                // the sequence.
                self.add_sequence(rules, &v, prev_state)
            },
            Choice(a, b) => {
                // Add states for each of the alternate sequences.
                // The previous state will have transitions to the start states
                // of both sequences, making this an NFA rather than a DFA.
                let end1 = self.add_sequence(rules, &a, prev_state);
                let end2 = self.add_sequence(rules, &b, prev_state);

                // Add final state for the choice.
                let new_state = self.state_count;
                self.state_count += 1;

                // Add transitions from each sequence's end state to the end state
                // for the choice. 
                self.add_transition(end1, Token::Nil, new_state);
                self.add_transition(end2, Token::Nil, new_state);

                new_state
            }
        }
    }

    fn add_transition(&mut self, from : usize, token : Token, to : usize) {
        self.transitions.push(( from, token, to ));
    }

    fn add_sequence(&mut self, rules : &[Rule], rule_indices : &[usize], prev_state : usize) -> usize {
        let mut last_state = prev_state;
        for &rule_index in rule_indices {
            last_state = self.add_states(rules, rule_index, last_state);
        }
        last_state
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

    fn finalize(&mut self) {
        // Resize transition_ranges to match the number of states.
        self.transition_ranges.resize(self.state_count, (0, 0));

        // Sort the transitions, so they're grouped by "from" state (the first field).
        self.transitions.sort_unstable();

        // Initialize the current range-start and state index.
        let mut current_range_start = 0;
        let mut current_state = self.transitions[0].0;

        // Iterate over all the transitions.
        for i in 1..self.transitions.len() {

            // If it's a new state, save the range of transition indices for the previous state.
            let state = self.transitions[i].0;
            if state != current_state {
                self.transition_ranges[current_state] = (current_range_start, i);
                current_range_start = i;
                current_state = state;
            }
        }

        // Save the range of transition indices for the last state.
        self.transition_ranges[current_state] = (current_range_start, self.transitions.len());
    }

    fn find_transitions(&self, state_index : usize) -> &[(usize, Token, usize)] {
        let (begin, end) = self.transition_ranges[state_index];
        &self.transitions[begin..end]
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
}

