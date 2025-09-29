

use std::{collections::HashMap, sync::LazyLock};

use slk_tokenstream::tokenstream::TokenStream;

use crate::{lexer_error::LexerError, lexer_token::{KeyWord, LexerToken, Literal, TokenType}, tokenizer::symbol_parser::parse_symbol};


static KEYWORDS: LazyLock<HashMap<String, KeyWord>> = LazyLock::new(|| {
    let mut e = HashMap::new();
    e.insert("fn".to_string(), KeyWord::FunctionDeclaration);
    e.insert("let".to_string(), KeyWord::VariableDeclaration);
    e.insert("return".to_string(), KeyWord::Return);
    e.insert("break".to_string(), KeyWord::Break);
    e.insert("continue".to_string(), KeyWord::Continue);
    e.insert("for".to_string(), KeyWord::ForDeclaration);
    e.insert("if".to_string(), KeyWord::IfDeclaration);
    e.insert("while".to_string(), KeyWord::WhileDeclaration);
    e
});



pub(crate) fn strip_literals_and_comments(program: String, file: String) -> Result<Vec<LexerToken>, LexerError> {
    
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
            println!("Parsing comment word");
            add_word(&mut tokens, &mut word, word_start_pos, file.clone())?;
            let comment = parse_singleline_comment(&mut chars, file.clone());
            tokens.push(comment);
            continue;
        }

        if is_string_literal(&chars) {
            add_word(&mut tokens, &mut word, word_start_pos, file.clone())?;
            println!("parsing string");
            let string = parse_string_literal(&mut chars, file.clone())?;
            tokens.push(string);
            continue;
        }
        
        let symbol = parse_symbol(char);
        match symbol {
            Some(symbol) => {
                add_word(&mut tokens, &mut word, word_start_pos, file.clone())?;
                tokens.push(
                    LexerToken { 
                        token_type: crate::lexer_token::TokenType::Symbol(symbol), 
                        start_position: chars.cursor(), 
                        length: 1, end_position: 
                        chars.cursor()+1,
                        file: file.clone()
                    }
                );
                chars.consume();
                continue
            },
            None => {
                if word.is_empty() {
                    word_start_pos = chars.cursor();
                }
                word.push(char);
                chars.consume();
            }
        }
    }

    Ok(tokens)
}

fn is_singleline_comment(token_stream: &TokenStream<char>) -> bool {
    matches!((token_stream.peek(), token_stream.peek_offset(1)), (Some(&'/'), Some('/')))
}

fn parse_singleline_comment(chars: &mut TokenStream<char>, file: String) -> LexerToken {
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
        length: raw_comment.len()-1,
        end_position: start + raw_comment.len()-1,
        token_type: crate::lexer_token::TokenType::Comment(
            crate::lexer_token::Comment::SingleLine { 
                value: raw_comment, 
                cleaned_value: cleaned_comment
            }
        ),
        file
    }
}

fn is_string_literal(chars: &TokenStream<char>) -> bool {
    chars.peek() == Some(&'"')
}

fn parse_string_literal(chars: &mut TokenStream<char>, file: String) -> Result<LexerToken, LexerError> {
    let start_pos = chars.cursor();
    chars.consume();
    let mut word = vec!['"'];
    loop {
        match chars.consume() {
            Some('"') => {
                chars.register_bookmark("a".to_string());
                chars.set_cursor(chars.cursor()-1);
                if chars.peek() == Some(&'\\') { // REMEMBER TO REMOVE BACKSLASH FROM PREVIOUS PARSE
                    println!("Previous character was backslash");
                    word.pop();
                    word.push('"');
                } else {
                    println!("Previous character was not backslash");
                    chars.consume();
                    word.push('"');
                    return Ok(
                        LexerToken {
                            start_position: start_pos,
                            length: word.len(),
                            end_position: start_pos + word.len(),
                            token_type: TokenType::Literal(
                                Literal::String(word.iter().collect())
                            ),
                            file
                        }
                    )
                }
                
            }
            Some(val) => {
                word.push(*val);
            }
            None => {
                println!("{:?}", word);
                return Err(LexerError::UnterminatedStringLiteral { position: chars.cursor()-1, file: file })
            }
        }
    }
}


fn add_word(tokens: &mut Vec<LexerToken>, word: &mut Vec<char>, word_start: usize, file: String) -> Result<(), LexerError> {
    let drained: String = word.drain(..).collect();
    let mut start = word_start;
    let mut current = String::new();

    for ch in drained.chars() {
        if ch.is_whitespace() {
            // finalize current word if non-empty
            if !current.is_empty() {
                process_word(tokens, &current, start, file.clone())?;
                start += current.len();
                current.clear();
            }
            // whitespace still counts toward position
            start += ch.len_utf8();
        } else {
            current.push(ch);
        }
    }

    // handle last word
    if !current.is_empty() {
        process_word(tokens, &current, start, file)?;
    }

    Ok(())
}

fn process_word(tokens: &mut Vec<LexerToken>, word: &str, start: usize, file: String) -> Result<(), LexerError> {
    if let Some(keyword) = KEYWORDS.get(word) {
        tokens.push(LexerToken {
            token_type: TokenType::Keyword(*keyword),
            start_position: start,
            length: word.len(),
            end_position: start + word.len(),
            file
        });
    } else if is_identifier(word) {
        tokens.push(LexerToken {
            token_type: TokenType::Identifier(word.to_string()),
            start_position: start,
            length: word.len(),
            end_position: start + word.len(),
            file
        });
    } else if word.parse::<u128>().is_ok() {
        tokens.push(LexerToken {
            token_type: TokenType::Literal(Literal::Integer(word.to_string())),
            start_position: start,
            length: word.len(),
            end_position: start + word.len(),
            file
        });
    }
    Ok(())
}


fn is_identifier(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    match chars.first() {
        Some(val) => {
            if val.is_alphabetic() || val == &'_' {

            } else {
                return false;
            }
        },
        _ => return false
    }

    chars.iter().all(|c| c.is_ascii_alphanumeric() || c == &'_')
}