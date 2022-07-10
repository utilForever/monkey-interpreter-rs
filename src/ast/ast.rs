pub struct Identifier(pub String);

pub enum Statement {
    Let(Identifier, Expression),
}

pub enum Expression {
    Identifier(Identifier),
}

pub type Program = Vec<Statement>;
