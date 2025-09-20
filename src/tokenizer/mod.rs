use crate::{lexer_error::LexerError, lexer_token::LexerToken, tokenizer::comment_literal_parse::strip_literals_and_comments};





mod comment_literal_parse;
mod keyword_symbol_parser;

pub(crate) fn tokenize(program: String) -> Result<Vec<LexerToken>, LexerError> {
    let first_pass_tokens = strip_literals_and_comments(program);
    println!("{:?}", first_pass_tokens);
    let second_pass_tokens = parse_keywords_symbols(first_pass_tokens);

    Ok(Vec::new())
}