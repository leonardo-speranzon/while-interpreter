use super::tokens::Token;


#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnknownSymbol{pos: (usize,usize), symbol: char},
    UnexpectedToken {pos: (usize,usize), expected: Option<Token>, found: Token},
}

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotInitialized(String)
}
