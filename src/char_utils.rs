use crate::token;

const OPERATORS: [&str; 19] = [
    token::PIPE,
    token::AMP,
    token::NOT,
    token::AND,
    token::OR,
    token::EQ,
    token::NEQ,
    token::LT,
    token::RT,
    token::LT_EQ,
    token::RT_EQ,
    token::PLUS,
    token::MINUS,
    token::TIMES,
    token::DIV,
    token::ASSIGN,
    token::DOT,
    token::POST_INCREMENT,
    token::POST_DECREMENT,
];

const COMP_OPERATORS: [char; 4] = [
    '=',
    '!',
    '<',
    '>',
];

const RESERVED: [&str; 18] = [
    token::INTEGER,
    token::RUNE,
    token::STRING,
    token::BOOLEAN,
    token::USER,
    token::REPO,
    token::CI_CONFIG,
    token::DEPLOYMENT,
    token::MANIFEST,
    token::IF,
    token::ELSE,
    token::WHILE,
    token::VOID,
    token::VAR,
    token::COMPLEX,
    token::RETURN,
    token::COMMAND,
    token::FUNC,
];

const POSTFIX_OPERATORS: [char; 2] = [
    '+',
    '-',
];

const LOGICAL_OPERATORS: [char; 2] = [
    '&',
    '|',
];

const PUNCTUATION_CHARS: [char; 2] = [
    ';',
    ','
];

const ARITHMETIC_OPERATORS: [&str; 5] = [
    token::PLUS,
    token::MINUS,
    token::TIMES,
    token::DIV,
    token::DOT,
];

pub fn is_letter(c: char) -> bool {
    c.is_alphabetic()
}

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn is_operator(s: &str) -> bool {
    OPERATORS.iter().any(|&op| op == s)
}

pub fn is_comparison_operator(c: char) -> bool {
    COMP_OPERATORS.iter().any(|&op| op == c)
}

pub fn is_arithmetic_operator(c: char) -> bool {
    ARITHMETIC_OPERATORS.iter().any(|&op| op == c.to_string())
}

pub fn is_postfix_operator(c: char) -> bool {
    POSTFIX_OPERATORS.iter().any(|&op| op == c)
}

pub fn is_logical_operator(c: char) -> bool {
    LOGICAL_OPERATORS.iter().any(|&op| op == c)
}

pub fn is_parenthesis(c: char) -> bool {
    c == '(' || c == ')'
}

pub fn is_new_line(c: char) -> bool {
    c == '\n'
}

pub fn is_whitespace_or_new_line(c: char) -> bool {
    c == ' ' || c == '\t' || is_new_line(c)
}

pub fn is_punctuation(c: char) -> bool {
    PUNCTUATION_CHARS.iter().any(|&op| op == c)
}

pub fn is_identifier_reserved(identifier: &str) -> bool {
    RESERVED.iter().any(|&op| op == identifier)
}

pub fn is_bracket(c: char) -> bool {
    c == '{' || c == '}'
}

pub fn is_boolean_literal(identifier: &str) -> bool {
    identifier == "true" || identifier == "false"
}

pub fn return_boolean_value(string_boolean: &str) -> bool {
    string_boolean == "true"
}
