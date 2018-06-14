//! The monotronian parser
//!
//! Our parser turns tokens into an abstract syntax tree.
//!
//! It is designed to parse either a function, or an immediate expression.
//! The resulting AST can then either be executed immediately, or stored in memory for later execution.
//!
//! The conversion back to text currently does not support indentation.

use core::fmt;
use lexer::Token;

/// A linear series of statements
#[derive(PartialEq, Debug, Clone)]
pub struct Block(Vec<Statement>);

/// Our program is made of statements.
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

/// A look-up in our hashmap of local/global variables
#[derive(PartialEq, Debug, Clone)]
pub struct Identifier(String);

/// Expressions are how things are calculated
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    /// Loop variable, start value, end value, step
    For(
        Identifier,
        Box<Expression>,
        Box<Expression>,
        Option<Box<Expression>>,
        Box<Block>,
    ),
    /// The expression is checked for truthiness
    IfExpr(Box<Expression>, Block, Option<Block>),
    /// First expression must be a string. Second is the arguments.
    FunctionCall(Box<Expression>, Vec<Expression>),
    Array(Vec<Expression>),
    Hash(Vec<(Literal, Expression)>),
    /// The first expression must be an Array or a Hash
    Index(Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    Negate,
    Bitflip,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    String(String),
    DecimalInt(i64),
    HexInt(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum Error {
    SyntaxError,
}

pub struct Parser {
    _state: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { _state: true }
    }

    pub fn feed(&mut self, _token: Token) -> Result<(), Error> {
        Err(Error::SyntaxError)
    }

    pub fn get_tree(self) -> Result<Block, Error> {
        Err(Error::SyntaxError)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for statement in self.0.iter() {
            writeln!(fmt, "{}", statement)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id, expr) => writeln!(fmt, "let {} = {};", id, expr)?,
            Statement::Return(expr) => writeln!(fmt, "return {};", expr)?,
            Statement::Expression(expr) => writeln!(fmt, "{};", expr)?,
        }
        Ok(())
    }
}
impl fmt::Display for Identifier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(x) => write!(fmt, "\"{}\"", x),
            Literal::DecimalInt(x) => write!(fmt, "{}", x),
            Literal::HexInt(x) => write!(fmt, "0x{:x}", x),
            Literal::Bool(x) => write!(fmt, "{}", x),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prefix::Negate => write!(fmt, "-"),
            Prefix::Bitflip => write!(fmt, "!"),
        }
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Infix::Add => write!(fmt, "+"),
            Infix::Subtract => write!(fmt, "-"),
            Infix::Multiply => write!(fmt, "*"),
            Infix::Divide => write!(fmt, "/"),
            Infix::Equal => write!(fmt, "=="),
            Infix::NotEqual => write!(fmt, "!="),
            Infix::GreaterThan => write!(fmt, ">"),
            Infix::GreaterThanOrEqual => write!(fmt, ">="),
            Infix::LessThan => write!(fmt, "<"),
            Infix::LessThanOrEqual => write!(fmt, "<="),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(fmt, "{}", id),
            Expression::Literal(lit) => write!(fmt, "{}", lit),
            Expression::Prefix(prefix, expr) => write!(fmt, "{}{}", prefix, expr),
            Expression::Infix(infix, expr_l, expr_r) => {
                write!(fmt, "{} {} {}", expr_l, infix, expr_r)
            }
            Expression::For(id, start, end, step, block) => {
                if let Some(s) = step {
                    write!(fmt, "for {} in {} to {} step {} {{", id, start, end, s)?;
                } else {
                    write!(fmt, "for {} in {} to {} {{", id, start, end)?;
                }
                write!(fmt, "{}", block)?;
                writeln!(fmt, "}}")
            }
            Expression::IfExpr(expr, true_block, false_block) => {
                writeln!(fmt, "if ({}) {{", expr)?;
                write!(fmt, "{}", true_block)?;
                if let Some(f) = false_block {
                    writeln!(fmt, "}} else {{")?;
                    write!(fmt, "{}", f)?;
                }
                writeln!(fmt, "}}")
            }
            Expression::FunctionCall(expr, args) => {
                write!(fmt, "{}(", expr)?;
                if let Some((last, elements)) = args.split_last() {
                    for arg in elements {
                        write!(fmt, "{}, ", arg)?;
                    }
                    write!(fmt, "{}", last)?;
                }
                writeln!(fmt, ")")
            }
            Expression::Array(expr_arr) => {
                write!(fmt, "[")?;
                if let Some((last, elements)) = expr_arr.split_last() {
                    for arg in elements {
                        write!(fmt, "{}, ", arg)?;
                    }
                    write!(fmt, "{}", last)?;
                }
                writeln!(fmt, "]")
            }
            Expression::Hash(map) => {
                write!(fmt, "{{")?;
                if let Some((last, elements)) = map.split_last() {
                    for arg in elements {
                        writeln!(fmt, "{}: {}, ", arg.0, arg.1)?;
                    }
                    writeln!(fmt, "{}: {}", last.0, last.1)?;
                }
                writeln!(fmt, "}}")
            }
            Expression::Index(array, index) => write!(fmt, "{}[{}]", array, index),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_parser() {
        let mut _p = Parser::new();
        // assert!(p.feed(Token::DecimalIntLiteral(123)).is_ok());
        // let result = p.get_tree();
        // // This is our block
        // let expected = vec! [
        //     Statement::Expression(Expression::Literal(Literal::DecimalInt(123)))
        // ];
        // assert_eq!(result.unwrap().0, expected);
    }
}
