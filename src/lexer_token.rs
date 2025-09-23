


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LexerToken {
    pub token_type: TokenType,
    pub start_position: usize,
    pub length: usize,
    pub end_position: usize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Keyword(KeyWord),
    Symbol(Symbol),
    Identifier(String),
    Literal(Literal),
    Comment(Comment),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyWord {
    FunctionDeclaration,
    VariableDeclaration,
    ForDeclaration,
    WhileDeclaration,
    IfDeclaration,
    Return,
    Break,
    Continue,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Star, // *
    OpenAngleBracket, // <
    CloseAngleBracket, // >
    OpenBracket,
    CloseBracket,
    OpenCurl,
    CloseCurl,
    OpenParen,
    CloseParen,
    Exclamation,
    At,
    Hashtag,
    DollarSign,
    PercentSign,
    Caret,
    Ampersand,
    Minus,
    Plus,
    Assign,
    ForwardSlash,
    BackSlash,
    Question,
    Period,
    Semicolon,
    Colon,
    Comma,
    Equals,
    GreaterEquals,
    LessEquals,
    NotEquals,
    PlusEquals,
    MinusEquals,
    DivideEquals,
    MultiplyEquals,
    Increment,
    Decrement,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    String(String),
    Integer(String),
    Float{
        integer: String,
        decimal: String,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Comment {
    SingleLine {
        value: String,
        cleaned_value: String,
    },
    Multiline {
        value: String,
        cleaned_value: String,
    },
    Documentation {
        value: String,
        cleaned_value: String,
    }
}