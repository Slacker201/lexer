use crate::lexer_token::{Symbol};




pub(crate) fn parse_symbol(character: char) -> Option<Symbol> {
    let char = match character {
        '*' => Symbol::Star,
        '<' => Symbol::OpenAngleBracket,
        '>' => Symbol::CloseAngleBracket,
        '[' => Symbol::OpenBracket,
        ']' => Symbol::CloseBracket,
        '{' => Symbol::OpenCurl,
        '}' => Symbol::CloseCurl,
        '(' => Symbol::OpenParen,
        ')' => Symbol::CloseParen,
        '!' => Symbol::Exclamation,
        '@' => Symbol::At,
        '#' => Symbol::Hashtag,
        '$' => Symbol::DollarSign,
        '%' => Symbol::PercentSign,
        '^' => Symbol::Caret,
        '&' => Symbol::Ampersand,
        '-' => Symbol::Minus,
        '+' => Symbol::Plus,
        '=' => Symbol::Assign,
        '/' => Symbol::ForwardSlash,
        '\\' => Symbol::BackSlash,
        '?' => Symbol::Question,
        '.' => Symbol::Period,
        ';' => Symbol::Semicolon,
        ':' => Symbol::Colon,
        ',' => Symbol::Comma,
        _ => return None,
    };
    Some(char)
}