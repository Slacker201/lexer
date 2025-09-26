use crate::{lexer_error::LexerError, lexer_token::LexerToken, tokenizer::comment_literal_parse::strip_literals_and_comments};





mod comment_literal_parse;
mod symbol_parser;

pub(crate) fn tokenize(program: String, file: String) -> Result<Vec<LexerToken>, LexerError> {
    let first_pass_tokens = strip_literals_and_comments(program, file)?;
    println!("{:?}", first_pass_tokens);


    Ok(first_pass_tokens)
}