use crate::token;
use crate::char_utils;
use crate::number_fsm;
use substring::Substring;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

fn strip_final_newline(s: String) -> String {
    let lf = '\n';
    let cr = '\r';


    if s.ends_with(lf) {
        return s[0..s.len() - 1].to_string();
    }

    if s.ends_with(cr) {
        return s[0..s.len() - 1].to_string();
    }
    s
}

impl Lexer {
    pub fn all_tokens(& mut self) -> Vec<token::Token> {
        self.input = strip_final_newline(self.input.clone());
        self.position = 0;
        self.column = 1;
        self.line = 1;
        let mut token = self.next_token();
        let mut tokens = vec![];
   
        while token.token_type != token::EOF {
            tokens.push(token);
            token = self.next_token();
        }

        tokens.push(token);

        return tokens;
    }

    fn next_token(&mut self) -> token::Token {
        if self.position >= self.input.len() {
            return token::Token {
                token_type: token::EOF.to_string(),
                value: "".to_string(),
                line: self.line,
                column: self.column
            };
        }

        self.skip_whitespaces_and_new_lines();

        let character = self.input.as_bytes()[self.position] as char;

        if char_utils::is_letter(character) {
            return self.recognize_identifier();
        }

        if char_utils::is_digit(character) {
            return self.recognize_number();
        }

        if character == '"' {
            return self.recognize_string();
        }

        if character == '\'' {
            return self.recognize_rune();
        }

        if char_utils::is_operator(&character.to_string()) {
            return self.recognize_operator();
        }

        if char_utils::is_parenthesis(character) {
            return self.recognize_parenthesis();
        }

        if char_utils::is_punctuation(character) {
            return self.recognize_punctuation();
        }

        if char_utils::is_bracket(character) {
            return self.recognize_bracket();
        }

        panic!("Unknown character {} at {}:{}", character, self.line, self.column)
    }

    fn skip_whitespaces_and_new_lines(&mut self) {
        while self.position < self.input.len() && char_utils::is_whitespace_or_new_line(self.input.as_bytes()[self.position] as char) {
            if char_utils::is_new_line(self.input.as_bytes()[self.position] as char) {
                self.line += 1;
                self.column = 1;
             } else {
                 self.column += 1;
             }
             self.position += 1;
         }
    }

    fn recognize_identifier(& mut self) -> token::Token {
        let mut identifier = "".to_string();
        let line = self.line;
        let column = self.column;
        let mut position = self.position;
    
        while position < self.input.len() {
            let character = self.input.as_bytes()[position] as char;
      
            if !(char_utils::is_letter(character) || char_utils::is_digit(character) || character == '_') {
            break;
            }
        
            identifier += &character.to_string();
            position += 1;
        }
    
        self.position += identifier.len();
        self.column += identifier.len();

        if char_utils::is_identifier_reserved(&identifier) {
            return token::Token {
                token_type: identifier.clone(),
                value: identifier,
                line: line,
                column: column
            }
        }
        if char_utils::is_boolean_literal(&identifier) {
            return token::Token {
                token_type: token::BOOLEAN_LITERAL.to_string(),
                value: identifier,
                line: line,
                column: column
            }
        }
    
        return token::Token {
            token_type: token::IDENTIFIER.to_string(),
            value: identifier,
            line: line,
            column: column
        }
    }

    fn recognize_number(& mut self) -> token::Token {
        let line = self.line;
        let column = self.column;

        let fsm = number_fsm::NumberFSM::new();
        let fsm_input = self.input.substring(self.position, self.input.len() - 1);

        let run_result = fsm.run(fsm_input);
        if run_result.recognized {
            self.position += run_result.value.len();
            self.column += run_result.value.len();

            return token::Token {
                token_type: token::INTEGER_LITERAL.to_string(),
                value: run_result.value,
                line,
                column,
            };
        } else {
            return token::Token {
                token_type: token::UNKNOWN.to_string(),
                value: "".to_string(),
                line,
                column,
            }
        }
    }

    fn recognize_string(& mut self) -> token::Token {
        let mut string_literal = "\"".to_string();
        let line = self.line;
        let column = self.column;
        let mut position = self.position + 1;
    
        while position < self.input.len()  {
            let character = self.input.as_bytes()[position] as char;
            string_literal += &character.to_string();
            if character == '"' || char_utils::is_new_line(character) {
                break;
            }
        
            position += 1;
        }
        

        self.position += string_literal.len();
        self.column += string_literal.len();

        if string_literal.as_bytes()[string_literal.len() - 1] as char != '"' {
            return token::Token {
                token_type: token::UNKNOWN.to_string(),
                value: "".to_string(),
                line: line,
                column: column,
            }
        } else {
            return token::Token {
                token_type: token::STRING_LITERAL.to_string(),
                value: string_literal[1..string_literal.len() - 1].to_string(),
                line: line,
                column: column,
            }
        }
    }

    fn recognize_rune(& mut self) -> token::Token {
        let line = self.line;
        let column = self.column;
        let position = self.position;
        let char_after_quote = self.input.as_bytes()[self.position + 1] as char;
        let char_that_must_be_quote = self.input.as_bytes()[self.position + 2] as char;

        self.position += 3;
        self.column += 3;

        if self.input.as_bytes()[position + 2] as char != '\'' {
            return token::Token {
                token_type: token::UNKNOWN.to_string(),
                value: format!("{}{}{}", '\'', char_after_quote, char_that_must_be_quote),
                line: line,
                column: column,
            }
        } else {
            return token::Token {
                token_type: token::RUNE_LITERAL.to_string(),
                value: char_after_quote.to_string(),
                line: line,
                column: column,
            }
        }
    }

    fn recognize_operator(& mut self) -> token::Token {
        let character = self.input.as_bytes()[self.position] as char;

        if char_utils::is_comparison_operator(character) {
            return self.recognize_comparison_operator();
        }

        if char_utils::is_arithmetic_operator(character) {
            return self.recognize_arithmetic_operator();
        }

        if char_utils::is_logical_operator(character) {
            return self.recognize_logical_operator();
        }

        panic!("Unknown operator type {}", character)
    }

    fn recognize_comparison_operator(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[self.position] as char;

        // 'lookahead' is the next character in the input
        // or 'null' if 'character' was the last character.
        let mut lookahead: char = '\0';
        if position + 1 < self.input.len() {
            lookahead = self.input.as_bytes()[position + 1] as char;
        }

        // Whether the 'lookahead' character is the equal symbol '='.
        let is_lookahead_equal_symbol = lookahead != '\0' && lookahead == '=';

        self.position += 1;
        self.column += 1;

        if is_lookahead_equal_symbol {
            self.position += 1;
            self.column += 1;
        }

        match character {
            '>' => {
                if is_lookahead_equal_symbol {
                    return token::Token {
                        token_type: token::RT_EQ.to_string(),
                        value: token::RT_EQ.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::RT.to_string(),
                        value: token::RT.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '<' => {
                if is_lookahead_equal_symbol {
                    return token::Token {
                        token_type: token::LT_EQ.to_string(),
                        value: token::LT_EQ.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::LT.to_string(),
                        value: token::LT.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '=' => {
                if is_lookahead_equal_symbol {
                    return token::Token {
                        token_type: token::EQ.to_string(),
                        value: token::EQ.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::ASSIGN.to_string(),
                        value: token::ASSIGN.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '!' => {
                if is_lookahead_equal_symbol {
                    return token::Token {
                        token_type: token::NEQ.to_string(),
                        value: token::NEQ.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::NOT.to_string(),
                        value: token::NOT.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            _ => {
                panic!("Unknow comparison op {} at {}:{}", character, line, column);
            }
        }
    }

    fn recognize_arithmetic_operator(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[self.position] as char;

        // 'lookahead' is the next character in the input
        // or 'null' if 'character' was the last character.
        let mut lookahead: char = '\0';
        if position + 1 < self.input.len() {
            lookahead = self.input.as_bytes()[position + 1] as char;
        }
        
        // Whether the 'lookahead' character is the equal symbol '='.
        let is_lookahead_plus_symbol = lookahead != '\0' && lookahead == '+';
        let is_lookahead_minus_symbol = lookahead != '\0' && lookahead == '-';
        
        self.position += 1;
        self.column += 1;

        if is_lookahead_plus_symbol || is_lookahead_minus_symbol {
            self.position += 1;
            self.column += 1;
        }

        match character {
            '+' => {
                if is_lookahead_plus_symbol {
                    return token::Token {
                        token_type: token::POST_INCREMENT.to_string(),
                        value: token::POST_INCREMENT.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::PLUS.to_string(),
                        value: token::PLUS.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '-' => {
                if is_lookahead_minus_symbol {
                    return token::Token {
                        token_type: token::POST_DECREMENT.to_string(),
                        value: token::POST_DECREMENT.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::MINUS.to_string(),
                        value: token::MINUS.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '*' => {
                return token::Token {
                    token_type: token::TIMES.to_string(),
                    value: token::TIMES.to_string(),
                    line: line,
                    column: column,
                }
            },
            '/' => {
                return token::Token {
                    token_type: token::DIV.to_string(),
                    value: token::DIV.to_string(),
                    line: line,
                    column: column,
                }
            },
            '.' => {
                // Not really an arithmetic op, but fit here so well
                return token::Token {
                    token_type: token::DOT.to_string(),
                    value: token::DOT.to_string(),
                    line: line,
                    column: column,
                }
            },
            _ => {
                panic!("Unknown arithmetic op {} at {}:{}", character, line, column);
            }
        }
    }

    fn recognize_logical_operator(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[self.position] as char;

        // 'lookahead' is the next character in the input
        // or 'null' if 'character' was the last character.
        let mut lookahead: char = '\0';
        if position + 1 < self.input.len() {
            lookahead = self.input.as_bytes()[position + 1] as char;
        }

        let is_lookahead_amp_symbol = lookahead != '\0' && lookahead == '&';
        let is_lookahead_pipe_symbol = lookahead != '\0' && lookahead == '|';

        self.position += 1;
        self.column += 1;

        if is_lookahead_amp_symbol || is_lookahead_pipe_symbol {
            self.position += 1;
            self.column += 1;
        }

        match character {
            '&' => {
                if is_lookahead_amp_symbol {
                    return token::Token {
                        token_type: token::AND.to_string(),
                        value: token::AND.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::AMP.to_string(),
                        value: token::AMP.to_string(),
                        line: line,
                        column: column,
                    }
                }
            },
            '|' => {
                if is_lookahead_pipe_symbol {
                    return token::Token {
                        token_type: token::OR.to_string(),
                        value: token::OR.to_string(),
                        line: line,
                        column: column,
                    }
                } else {
                    return token::Token {
                        token_type: token::UNKNOWN.to_string(),
                        value: format!("{}{}", character, lookahead),
                        line: line,
                        column: column,
                    }
                }
            },
            _ => {
                panic!("Unknow logical op {} at {}:{}", character, line, column);
            }
        }
    }

    fn recognize_parenthesis(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[position] as char;

        self.position += 1;
        self.column += 1;

        if character == '(' {
            return token::Token {
                token_type: token::L_PARENT.to_string(),
                value: token::L_PARENT.to_string(),
                line: line,
                column: column,
            }
        }

        return token::Token {
            token_type: token::R_PARENT.to_string(),
            value: token::R_PARENT.to_string(),
            line: line,
            column: column,
        }
    }

    fn recognize_punctuation(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[position] as char;

        self.position += 1;
        self.column += 1;

        if character == ';' {
            return token::Token {
                token_type: token::SEMI.to_string(),
                value: token::SEMI.to_string(),
                line: line,
                column: column,
            }
        }

        return token::Token {
            token_type: token::COMMA.to_string(),
            value: token::COMMA.to_string(),
            line: line,
            column: column,
        }
    }

    fn recognize_bracket(& mut self) -> token::Token {
        let position = self.position;
        let line = self.line;
        let column = self.column;
        let character = self.input.as_bytes()[position] as char;

        self.position += 1;
        self.column += 1;

        if character == '{' {
            return token::Token {
                token_type: token::L_BRACE.to_string(),
                value: token::L_BRACE.to_string(),
                line: line,
                column: column,
            }
        }

        return token::Token {
            token_type: token::R_BRACE.to_string(),
            value: token::R_BRACE.to_string(),
            line: line,
            column: column,
        }
    }
}
