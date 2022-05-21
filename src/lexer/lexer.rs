use crate::token::token::Token;

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next_token(&self) -> Token {
        Token::Illegal
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"=+(){},;"#;
        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new();

        for expect in tests {
            let tok = lexer.next_token();
            assert_eq!(expect, tok);
        }
    }
}
