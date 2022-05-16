mod token;
mod char_utils;
mod fsm;
mod lexer;
mod number_fsm;

fn main() {
    let mut lexer = lexer::Lexer { input: "rune \"lolz\"\n\"lulz\" 'w'= ++ && ;, -12.5 {}\n".to_string(), position: 0, line: 0, column: 0 };
    let tokens = lexer.all_tokens();
    println!("Tokens are: {:#?}", tokens);
}
