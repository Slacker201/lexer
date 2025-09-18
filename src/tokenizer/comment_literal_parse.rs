use slk_tokenstream::tokenstream::TokenStream;

use crate::lexer_error::LexerError;





pub(crate) fn strip_literals_and_comments(program: String) -> Vec<CommentLiteralTokens> {
    
    let mut chars = TokenStream::new(program.chars().collect());
    let mut tokens = Vec::new();
    let mut word = Vec::new();
    let mut word_start_pos = 0;
    loop {
        let current_char = chars.peek();
        let next_char = chars.peek_offset(1);
        if current_char.is_none() {
            break;
        }
        if is_single_line_comment(current_char, next_char) {
            if !word.is_empty() {
                if word.iter().collect::<String>().trim().is_empty() {

                } else {
                    tokens.push(
                        CommentLiteralTokens::Unparsed { 
                            value: word.drain(..).collect(), 
                            start_position: word_start_pos, 
                            end_position: chars.cursor(), 
                            file: "TODOTODOTODO".to_string()
                        }
                    );
                }
            }
            let start_pos = chars.cursor();
            let comment = parse_single_line_comment(&mut chars);
            tokens.push(
                CommentLiteralTokens::Comment { 
                    value: comment.iter().collect(), 
                    start_position: start_pos, 
                    end_position: chars.cursor(), 
                    file: "TODOTODOTODO".to_string()
                }
            );
        } else if is_multiline_comment(current_char, next_char) {
            if !word.is_empty() {
                if word.iter().collect::<String>().trim().is_empty() {

                } else {
                    tokens.push(
                            CommentLiteralTokens::Unparsed { 
                                value: word.drain(..).collect(), 
                                start_position: word_start_pos, 
                                end_position: chars.cursor(), 
                                file: "TODOTODOTODO".to_string()
                            }
                        );
                    }
            }
            let start_pos = chars.cursor();
            let comment = parse_multiline_comment(&mut chars);
            tokens.push(
                CommentLiteralTokens::Comment { 
                    value: comment.iter().collect(), 
                    start_position: start_pos, 
                    end_position: chars.cursor(), 
                    file: "TODOTODOTODO".to_string()
                }
            );
        } else if current_char == Some(&'"') {
            if !word.is_empty() {
                if word.iter().collect::<String>().trim().is_empty() {
                    
                } else {tokens.push(
                        CommentLiteralTokens::Unparsed { 
                            value: word.drain(..).collect(), 
                            start_position: word_start_pos, 
                            end_position: chars.cursor(), 
                            file: "TODOTODOTODO".to_string()
                        }
                    );
                }
            }
            let start_pos = chars.cursor();
            match parse_string_literal(&mut chars) {
                Ok(literal) => {
                    tokens.push(
                        CommentLiteralTokens::StringLiteral { 
                            value: literal.iter().collect(), 
                            start_position: start_pos, 
                            end_position: chars.cursor(), 
                            file: "TODOTODOTODO".to_string()
                        }
                    );
                }
                Err(e) => {
                    tokens.push(
                        CommentLiteralTokens::Error { 
                            message: format!("{:?}", e), 
                            position: chars.cursor(), 
                            file: "TODOTODOTODO".to_string()
                        }
                    );
                    break;
                }
            }
        } else {
            if word.is_empty() {
                word_start_pos = chars.cursor();
            }
            word.push(current_char.unwrap().clone());
            chars.consume();
        }
    }
    if !word.is_empty() {
        if word.iter().collect::<String>().trim().is_empty() {
            return tokens;
        } else {
                tokens.push(
                CommentLiteralTokens::Unparsed { 
                    value: word.iter().collect(), 
                    start_position: word_start_pos, 
                    end_position: chars.cursor(), 
                    file: "TODOTODOTODO".to_string()
                }
            );
        }
        
    }

    tokens
}

fn is_single_line_comment(current_char: Option<&char>, next_char: Option<&char>) -> bool {
    match (current_char, next_char) {
        (Some('/'), Some('/')) => true,
        _ => false,
    }
}
fn is_multiline_comment(current_char: Option<&char>, next_char: Option<&char>) -> bool {
    match (current_char, next_char) {
        (Some('/'), Some('*')) => true,
        _ => false,
    }
}

fn parse_multiline_comment(chars: &mut TokenStream<char>) -> Vec<char> {
    let mut comment = Vec::new();
    comment.push('/');
    comment.push('*');
    chars.consume();
    chars.consume();
    loop {
        let c = chars.peek();
        match c {
            Some('*') | None => {
                if chars.peek_offset(1) == Some(&'/') {
                    comment.push(c.unwrap().clone());
                    comment.push(chars.peek_offset(1).unwrap().clone());
                    chars.consume();
                    chars.consume();
                    break;
                }
                comment.push(c.unwrap().clone());
                chars.consume();
            }
            Some(ch) => {
                comment.push(ch.clone());
                chars.consume();
            }
        }
    }
    comment
}

fn parse_single_line_comment(chars: &mut TokenStream<char>) -> Vec<char> {
    let mut comment = Vec::new();
    comment.push('/');
    comment.push('/');
    chars.consume();
    chars.consume();
    loop {
        let c = chars.peek();
        match c {
            Some('\n') => {
                comment.push('\n');
                chars.consume();
                return comment;
            }
            Some(ch) => {
                comment.push(ch.clone());
                chars.consume();
            }
            None => return comment,
        }
    }
}

fn parse_string_literal(chars: &mut TokenStream<char>) -> Result<Vec<char>, LexerError> {
    let mut literal = Vec::new();
    literal.push('"');
    chars.consume();
    loop {
        match chars.consume() {
            Some('"') => {
                literal.push('"');
                break;
            }
            Some(ch) => {
                literal.push(*ch);
            }
            None => {
                return Err(LexerError::UnterminatedStringLiteral {
                    position: chars.cursor(),
                    file: "TODOTODOTODO".to_string()
                });
            }
        }
    }
    return Ok(literal);
}
#[derive(Debug, Clone)]
pub(crate) enum CommentLiteralTokens {
    Comment {
        value: String,
        start_position: usize,
        end_position: usize,
        file: String,
    },
    StringLiteral {
        value: String,
        start_position: usize,
        end_position: usize,
        file: String,
    },
    Unparsed {
        value: String,
        start_position: usize,
        end_position: usize,
        file: String,
    },
    Error {
        message: String,
        position: usize,
        file: String,
    }
}

enum State {
    Normal,
    InSingleLineComment,
    InMultiLineComment,
    InStringLiteral,
}