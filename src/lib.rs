use crate::{lexer_error::LexerError, lexer_token::LexerToken, tokenizer::tokenize};
use slk_tokenstream::tokenstream;

pub mod lexer_token;
pub mod lexer_error;
mod tokenizer;

pub fn lex(program: String) -> Result<Vec<LexerToken>, LexerError> {
    tokenize(program)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let program = "let test = 26; // this is a comment\nfn main() {} println!(\"Test\");/* multiline\ncomment*/more code testing a literal \"test\" \"//fake comment\"".to_string();
        let tokens = lex(program).unwrap();
        println!("{:?}", tokens);
    }
}
