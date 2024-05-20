use super::tokens::Token;


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
