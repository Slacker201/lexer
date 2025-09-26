use crate::{lexer_error::LexerError, lexer_token::LexerToken, tokenizer::tokenize};

pub mod lexer_token;
pub mod lexer_error;
mod tokenizer;

pub fn lex(program: String) -> Result<Vec<LexerToken>, LexerError> {
    tokenize(program, "test.slk".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let program = "let test = 21;\n//\"test\"\n//test2\nfn identifier".to_string();
        let tokens = lex(program).unwrap();
        println!("{:?}", tokens);
    }
}
