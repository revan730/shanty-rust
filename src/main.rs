mod token;
mod char_utils;
mod fsm;
mod lexer;

fn main() {
    let mut lexer = lexer::Lexer { input: "rune \"lolz\"\n\"lulz\" 'w'= ++ && ;, {}\n".to_string(), position: 0, line: 0, column: 0 };
    let tokens = lexer.all_tokens();
    println!("Tokens are: {:#?}", tokens);
}
