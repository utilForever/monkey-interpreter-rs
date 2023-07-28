use std::fmt;

use crate::ast::ast::{Expression, Identifier, Literal, Precedence, Program, Statement};
use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    msg: String,
}

impl ParseError {
    fn new(kind: ParseErrorKind, msg: String) -> Self {
        ParseError { kind, msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    next_token: Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
            errors: Vec::new(),
        };

        parser.bump();
        parser.bump();

        parser
    }

    #[allow(dead_code)]
    fn get_errors(&mut self) -> Vec<ParseError> {
        self.errors.clone()
    }

    fn bump(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, token: Token) -> bool {
        self.current_token == token
    }

    fn next_token_is(&mut self, token: Token) -> bool {
        self.next_token == token
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.next_token_is(token.clone()) {
            self.bump();
            true
        } else {
            self.error_next_token(token);
            false
        }
    }

    fn error_next_token(&mut self, token: Token) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "expected next token to be {:?}, got {:?} instead",
                token, self.next_token
            ),
        ));
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            }

            self.bump();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let identifier = match self.parse_identifier() {
            Some(identifier) => identifier,
            None => return None,
        };

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        self.bump();

        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };

        if self.current_token_is(Token::Semicolon) {
            self.bump();
        }

        Some(Statement::Let(identifier, expression))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.bump();

        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };

        if self.next_token_is(Token::Semicolon) {
            self.bump();
        }

        Some(Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        match self.parse_expression(Precedence::Lowest) {
            Some(expression) => {
                if self.next_token_is(Token::Semicolon) {
                    self.bump();
                }

                Some(Statement::Expression(expression))
            }
            None => None,
        }
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression> {
        // Prefix
        let left = match self.current_token {
            Token::Ident(_) => self.parse_identifier_expression(),
            Token::Int(_) => self.parse_int_expression(),
            _ => None,
        };

        left
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        match &self.current_token {
            Token::Ident(ident) => Some(Identifier(ident.clone())),
            _ => None,
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        match self.parse_identifier() {
            Some(identifier) => Some(Expression::Identifier(identifier)),
            None => None,
        }
    }

    fn parse_int_expression(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Int(int) => Some(Expression::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::{Expression, Identifier, Literal, Statement};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    fn check_parse_errors(parser: &mut Parser) {
        let errors = parser.get_errors();

        if errors.is_empty() {
            return;
        }

        println!("\n");
        println!("parser has {} errors", errors.len());

        for error in errors {
            println!("parse error: {:?}", error);
        }

        println!("\n");
        panic!("failed");
    }

    #[test]
    fn test_let_statement() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&mut p);

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

    #[test]
    fn test_return_statement() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&mut p);

        assert_eq!(
            vec![
                Statement::Return(Expression::Literal(Literal::Int(5))),
                Statement::Return(Expression::Literal(Literal::Int(10))),
                Statement::Return(Expression::Literal(Literal::Int(993322))),
            ],
            program,
        );
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&mut p);

        assert_eq!(
            vec![Statement::Expression(Expression::Identifier(Identifier(
                String::from("foobar")
            )))],
            program,
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&mut p);

        assert_eq!(
            vec![Statement::Expression(Expression::Literal(Literal::Int(5)))],
            program,
        );
    }
}
