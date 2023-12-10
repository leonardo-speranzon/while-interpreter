use super::lexer::{Token, TokenPosition};


#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnknownSymbol{pos: TokenPosition, symbol: char},
    UnexpectedToken {pos: TokenPosition, expected: Option<Token>, found: Token},
}