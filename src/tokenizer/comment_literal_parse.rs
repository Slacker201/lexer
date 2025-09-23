

use std::{collections::HashMap, hash::Hash, sync::LazyLock};

use slk_tokenstream::tokenstream::TokenStream;

use crate::{lexer_error::LexerError, lexer_token::{KeyWord, LexerToken, TokenType}, tokenizer::symbol_parser::parse_symbol};


static KEYWORDS: LazyLock<HashMap<String, KeyWord>> = LazyLock::new(|| {
    let mut e = HashMap::new();
    e.insert("fn".to_string(), KeyWord::FunctionDeclaration);
    e.insert("let".to_string(), KeyWord::VariableDeclaration);
    e
});



pub(crate) fn strip_literals_and_comments(program: String) -> Result<Vec<LexerToken>, LexerError> {
    
    let mut chars = TokenStream::new(program.chars().collect());
    let mut tokens = Vec::new();
    let mut word = Vec::new();
    let mut word_start_pos = 0;
    loop {
        let char = match chars.peek() {
            Some(char) => *char,
            None => break,
        };
        if is_singleline_comment(&chars) {
            add_word(&mut tokens, &mut word, word_start_pos)?;
            let comment = parse_singleline_comment(&mut chars);
            tokens.push(comment);
            continue;
        }
        
        let symbol = parse_symbol(char);
        match symbol {
            Some(symbol) => {
                tokens.push(
                    LexerToken { 
                        token_type: crate::lexer_token::TokenType::Symbol(symbol), 
                        start_position: chars.cursor(), 
                        length: 1, end_position: 
                        chars.cursor()+1 
                    }
                );
                chars.consume();
                continue
            },
            None => {
                println!("Did not match char into symbol: {}", char);
                word.push(char);
            }
        }
    }

    Ok(tokens)
}

fn is_singleline_comment(token_stream: &TokenStream<char>) -> bool {
    matches!((token_stream.peek(), token_stream.peek_offset(1)), (Some(&'/'), Some('/')))
}

fn parse_singleline_comment(chars: &mut TokenStream<char>) -> LexerToken {
    let mut comment_src = Vec::new();
    let start = chars.cursor();
    comment_src.push('/');
    comment_src.push('/');
    chars.consume();
    chars.consume();
    loop {
        match chars.consume() {
            Some(char) => {
                comment_src.push(*char);
                match *char {
                    '\n' => {
                        comment_src.push('\n');
                        break;
                    }
                    _ => {

                    }
                }
            }
            None => {
                break;
            }
        }
    }
    let cleaned_comment = comment_src.clone()[2..].iter().collect();
    let raw_comment: String = comment_src.iter().collect();
    LexerToken { 
        start_position: start,
        length: raw_comment.len(),
        end_position: start + raw_comment.len(),
        token_type: crate::lexer_token::TokenType::Comment(
            crate::lexer_token::Comment::SingleLine { 
                value: raw_comment, 
                cleaned_value: cleaned_comment
            }
        ),
    }
}




fn add_word(tokens: &mut Vec<LexerToken>, word: &mut Vec<char>, word_start: usize) -> Result<(), LexerError> {
    {
        let word2: String = word.iter().collect();
        let val = KEYWORDS.get(&word2);
        match val {
            Some(keyword) => {
                let token = LexerToken {
                    token_type: TokenType::Keyword(*keyword),
                    start_position: word_start,
                    length: word2.len(),
                    end_position: word_start+word2.len()
                };
                tokens.push(token);
            }
            None => {
                // deal with identifiers n stuff
            }
        }
    }
    Ok(())
}