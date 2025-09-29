use crate::{lexer_error::LexerError, lexer_token::LexerToken, tokenizer::tokenize};

pub mod lexer_token;
pub mod lexer_error;
mod tokenizer;

pub fn lex(program: String, file: String) -> Result<Vec<LexerToken>, LexerError> {
    tokenize(program, file)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let program = "\"test\"".to_string();
        let tokens = lex(program, "test_file".to_string()).unwrap();
        println!("tokens: {:?}", tokens);
    }
}
