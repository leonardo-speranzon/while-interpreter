use std::fmt::Display;

use super::{ast::NumLiteral, tokens::Token};


#[derive(Debug)]
pub enum ParserError<N> {
    UnexpectedEOF,
    UnknownSymbol{pos: (usize,usize), symbol: char},
    UnexpectedToken {pos: (usize,usize), expected: Option<Token<N>>, found: Token<N>},
}

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotInitialized(String),
    NotImplemented(String)
}

impl<N: NumLiteral> Display for ParserError<N>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEOF => 
                write!(f,"Unexpected EOF encountered"),
            ParserError::UnknownSymbol { pos:(l,c), symbol } =>
                write!(f,"Unknown symbol encountered: '{symbol}' at location {l}:{c}"),
            ParserError::UnexpectedToken { pos:(l,c), expected: None, found } =>
                write!(f,"Unexpected token encountered: {:?} at location {l}:{c}", found),
            ParserError::UnexpectedToken { pos:(l,c), expected: Some(expected), found } =>
                write!(f,"Expected token {:?} but found {:?} at location {l}:{c}", expected, found),
        }
    }
}