pub enum Token {
    Illegal,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    RParen,
    Lbrace,
    Rbrace,

    // Reserved Keywords
    Function,
    Let,
}
