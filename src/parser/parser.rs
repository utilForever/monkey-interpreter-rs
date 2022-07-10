use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
        };

        // parser.next_token();
        // parser.next_token();

        parser
    }
}
