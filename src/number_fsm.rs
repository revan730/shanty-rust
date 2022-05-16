use crate::char_utils;
use crate::fsm;

#[derive(PartialEq)]
enum States {
    Initial,
    Integer,
    BeginNumberWithFractionalPart,
    NumberWithFractionalPart,
    BeginNumberWithExponent,
    BeginNumberWithSignedExponent,
    NumberWithExponent,
    NoNextState = -1,
}

fn next_state(current_state: i8, character: char) -> i8 {
    if current_state == States::Initial as i8 && char_utils::is_digit(character) {
        return States::Integer as i8;
    }

    if character == '.' {
        return States::BeginNumberWithFractionalPart as i8;
    }

    if current_state == States::NumberWithExponent as i8 && char_utils::is_digit(character) {
        return States::NumberWithExponent as i8;
    }

    if current_state == States::Integer as i8 {
        if char_utils::is_digit(character) {
            return States::Integer as i8;
        }

        if character.to_lowercase().to_string() == "e" {
            return States::BeginNumberWithExponent as i8;
        }
    }

    if current_state == States::BeginNumberWithFractionalPart as i8 {
        if char_utils::is_digit(character) {
            return States::NumberWithFractionalPart as i8;
        }
    }

    if current_state == States::NumberWithFractionalPart as i8 {
        if char_utils::is_digit(character) {
            return States::NumberWithFractionalPart as i8;
        }

        if character.to_lowercase().to_string() == "e" {
            return States::BeginNumberWithExponent as i8;
        }
    }

    if current_state == States::BeginNumberWithExponent as i8 {
        if character == '+' || character == '-' {
            return States::BeginNumberWithSignedExponent as i8;
        }

        if char_utils::is_digit(character) {
            return States::NumberWithExponent as i8;
        }
    }

    if current_state == States::BeginNumberWithSignedExponent as i8 {
        if char_utils::is_digit(character) {
            return States::NumberWithExponent as i8;
        }
    }

    States::NoNextState as i8
}

pub struct NumberFSM<'a> {
    fsm: fsm::FSM<'a>
}

impl NumberFSM<'_> {
    pub(crate) fn run(&self, input: &str) -> fsm::RunResult {
        self.fsm.run(input)
    }

    pub(crate) fn new() -> Self {
        let accepting_states = vec![States::Integer as i8, States::NumberWithFractionalPart as i8, States::NumberWithExponent as i8];
        let fsm = fsm::FSM { states: vec![], initial_state: States::Initial as i8, accepting_states, next_state: &next_state }; // TODO: It seems like states are not needed ?? lolz
        Self { fsm }
    }
}
