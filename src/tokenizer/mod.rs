use slk_lexer_tokens::lexer_tokens::{Delimitor, Punctuation, SlkLexerToken, Symbol, TokenType};
use slk_tokenstream::tokenstream::TokenStream;







pub fn tokenize(program: String, file_name: String) -> Vec<SlkLexerToken> {
    let program = TokenStream::new(program.chars().collect());
    loop {
        let token = match program.peek() {
            Some(';') => SlkLexerToken {
                token_type: TokenType::Punctuation(Punctuation::SemiColon),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some(':') => SlkLexerToken {
                token_type: TokenType::Punctuation(Punctuation::Colon),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('.') => SlkLexerToken {
                token_type: TokenType::Punctuation(Punctuation::Period),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some(',') => SlkLexerToken {
                token_type: TokenType::Punctuation(Punctuation::Comma),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },

            // --- Delimiters ---
            Some('(') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::OpenParen),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some(')') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::CloseParen),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('[') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::OpenBracket),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some(']') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::CloseBracket),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('{') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::OpenCurl),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('}') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::CloseCurl),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('<') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::OpenAngle),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('>') => SlkLexerToken {
                token_type: TokenType::Delimitor(Delimitor::CloseAngle),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },

            // --- Symbols ---
            Some('!') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Exclamation),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('@') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::At),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('#') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::HashTag),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('$') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::DollarSign),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('%') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::PercentSign),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('^') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Caret),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('&') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Ampersand),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('*') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Star),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('-') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Minus),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('+') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Plus),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('=') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Equals),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('|') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Pipe),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('\\') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::BackSlash),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('/') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::ForwardSlash),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('~') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::Tilda),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },
            Some('`') => SlkLexerToken {
                token_type: TokenType::Symbol(Symbol::BackTick),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },

            // --- Default / Unknown ---
            Some(other) => SlkLexerToken {
                token_type: TokenType::Unknown(other.to_string()),
                char_start: program.cursor(),
                char_end: program.cursor() + 1,
                file_name: file_name.clone(),
            },

            None => {
                SlkLexerToken {
                    token_type: TokenType::EOF,
                    char_start: program.cursor(),
                    char_end: program.cursor(),
                    file_name: file_name.clone(),
                }
            }
        };

    }
    Vec::new()
}