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

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::{Expression, Identifier, Statement};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    fn check_parse_errors(parser: &mut Parser) {
        // TODO: Check if there are any parse errors
    }

    fn test_let_statement() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let mut l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&mut parser);

        assert_eq!(
            vec![
                Statement::Let(
                    Identifier(String::from("x")),
                    Expression::Literal(Literal::Int(5))
                ),
                Statement::Let(
                    Identifier(String::from("y")),
                    Expression::Literal(Literal::Int(10))
                ),
                Statement::Let(
                    Identifier(String::from("foobar")),
                    Expression::Literal(Literal::Int(838383)),
                ),
            ],
            program,
        );
    }
}
