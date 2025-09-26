


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    MalformedIdentifier {
        value: String,
        start_position: usize,
        end_position: usize,
        file: String,
    },
    UnterminatedStringLiteral {
        position: usize,
        file: String,
    },
    UnterminatedBlockComment {
        position: usize,
        file: String
    }
}