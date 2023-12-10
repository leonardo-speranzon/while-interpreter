use std::io::{BufReader, BufRead};
use std::iter::Peekable;
use std::fs::File;
use crate::ast::Num;
use super::types::ParserError;


pub type TokenPosition = (usize,usize);

#[derive(Debug,Clone)]
#[derive(PartialEq)]
pub enum Token {
    Id(String),
    Num(Num),
    True,
    False,

    Skip,
    If,
    Then,
    Else,
    While,
    Do,
    Repeat,
    Until,
    For,

    Assign,
    Plus,
    Minus,
    Mul,
    Inc,
    Dec,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    

    Not,
    And,
    Or,


    Semicolon,
    BracketOpen,
    BracketClose,
    CurlyOpen,
    CurlyClose,
}

pub trait Lexer {
    fn peek(&self) -> Option<Token>;
    fn next(&mut self) -> Result<Option<Token>, ParserError>;
    fn match_next(&mut self, tok: Token)-> Result<(),ParserError>;
    fn unexpected_error(&self) -> ParserError;
}



pub struct MyLexer<'a> {
    chars: Peekable<Box<dyn Iterator<Item =(usize,usize,char)> + 'a>>,
    peek: Option<(TokenPosition, Token)>
}

impl<'a> From<&'a str> for MyLexer<'a>{
    fn from(value: &'a str) -> Self {
        let it= value.lines()
            .enumerate()
            .map(|(i,line)| 
                line.chars()
                    .enumerate()
                    .map(move |(j, c)|(i+1,j+1,c)))
            .flatten();
        let b: Box<dyn Iterator<Item = (usize,usize,char)>> = Box::new(it);
        let mut lex =  MyLexer {
            peek: None,
            chars: b.peekable(),
        };
        
        lex.peek = lex.scan().unwrap();
        return lex;
    }
}
impl<'a> From<File> for MyLexer<'a> {
    fn from(value: File) -> Self {
        let reader = BufReader::new(value);
        let it = reader.lines().enumerate()
        .map(|(i,line)| 
            line.unwrap().chars()
                .enumerate()
                .map(move |(j, c)|(i+1,j+1,c)).collect::<Vec<_>>()
            )
        .flatten();
        let b: Box<dyn Iterator<Item = (usize,usize,char)>> = Box::new(it);
        let mut lex =  MyLexer {
            peek: None,
            chars: b.peekable(),
        };
        
        lex.peek = lex.scan().unwrap();
        return lex;
    }
    
}


impl<'a> MyLexer<'a>{
    fn scan(&mut self) -> Result<Option<(TokenPosition, Token)>, ParserError> {
        while let Some(_) = self.chars.next_if(|(_,_,c)|c.is_ascii_whitespace()){}

        if let Some(_) = self.chars.next_if(|(_,_,c)|c==&'/') {
            match self.chars.next() {
                Some((l,_,'/')) => {
                    while let Some(_) = self.chars.next_if(|(l2,_,_)|l==*l2){}
                    return self.scan()
                },
                Some((l,c,symbol)) => return Err(ParserError::UnknownSymbol { pos: (l,c), symbol }),
                None => return Err(ParserError::UnexpectedEOF)
            }
        }

        let start_pos: Option<TokenPosition> = self.chars.peek().map(|(l,c,_)|(l.to_owned(),c.to_owned()));
        let tok = match self.chars.next() {
            Some((_, _, ';')) => Token::Semicolon,
            Some((_, _, '=')) => Token::Eq,
            Some((_, _, '*')) => Token::Mul,
            Some((_, _, '(')) => Token::BracketOpen,
            Some((_, _, ')')) => Token::BracketClose,
            Some((_, _, '{')) => Token::CurlyOpen,
            Some((_, _, '}')) => Token::CurlyClose,
            Some((_, _, ':')) =>  match self.chars.next(){
                Some((_, _, '=')) => Token::Assign,
                Some((l,c,symbol)) => return Err(ParserError::UnknownSymbol { pos: (l,c), symbol }),
                None => return Err(ParserError::UnexpectedEOF)
            },
            Some((_, _, '!')) =>  match self.chars.next(){
                Some((_, _, '=')) => Token::Ne,
                Some((l,c,symbol)) => return Err(ParserError::UnknownSymbol { pos: (l,c), symbol }),
                None => return Err(ParserError::UnexpectedEOF)
            },
            Some((_, _, '+')) => match self.chars.next_if(|(_, _, c)|c == &'=') {
                Some((_, _, '=')) => Token::Inc,
                _ => Token::Plus,
            },
            Some((_, _, '-')) => match self.chars.next_if(|(_, _, c)|c == &'=') {
                Some((_, _, '=')) =>  Token::Dec,
                _ => Token::Minus,
            },
            Some((_, _, '<')) => match self.chars.next_if(|(_, _, c)|c == &'=') {
                Some((_, _, '=')) =>  Token::Le,
                _ => Token::Lt,
            },
            Some((_, _, '>')) =>  match self.chars.next_if(|(_, _, c)|c == &'=') {
                Some((_, _, '=')) =>  Token::Ge,
                _ => Token::Gt,
            },
            Some((_, _, d@'0'..='9')) => {
                let mut digits = d.to_string();
                while let Some((_, _, d)) = self.chars.next_if(|(_, _, c)| ('0'..='9').contains(c)){
                    digits.push(d)
                }
                Token::Num((digits).parse().unwrap())//By construction should be valid                
            }
            Some((_, _, c@('a'..='z' | 'A'..='Z' | '_'))) => {
                let mut word = c.to_string();
                while let Some((_, _, c)) = self.chars.next_if(|(_, _, c)| c.is_ascii_alphanumeric() || c == &'_'){
                    word.push(c)
                }

                let tok = match word.as_str() {
                    "if" => Token::If,
                    "then" => Token::Then,
                    "else" => Token::Else,

                    "while" => Token::While,
                    "do" => Token::Do,
                    "repeat" => Token::Repeat,
                    "until" => Token::Until,
                    "for" => Token::For,
                    "skip" => Token::Skip,                    
                    
                    "not" => Token::Not,
                    "and" => Token::And,
                    "or" => Token::Or,

                    "true" => Token::True,
                    "false" => Token::False,
                    
                    s => Token::Id(s.to_string()),
                };
                tok
            }
            Some((_, _, c)) => return Err(ParserError::UnknownSymbol{
                pos: start_pos.unwrap(),
                symbol: c,
            }),
            None => return Ok(None),
        };
        
        return Ok(Some((start_pos.unwrap(), tok)));
    }
}


impl<'a> Lexer for MyLexer<'a> {

    fn peek(&self) -> Option<Token> {
        self.peek.clone().map(|(_,t)|t)
    }
    fn next(&mut self) -> Result<Option<Token>, ParserError> {
        let tok = self.peek.clone();
        self.peek = self.scan()?;
        return Ok(tok.map(|(_,t)|t))
    }
    fn match_next(&mut self, tok: Token)-> Result<(),ParserError> { 
        match &self.peek {
            Some((pos, tok2)) => {
                if tok != *tok2 {
                    return Err(ParserError::UnexpectedToken {
                        pos: pos.clone(),
                        expected: Some(tok),
                        found: tok2.clone()
                    })
                }
            }
            None => return Err(ParserError::UnexpectedEOF),
        }   
        self.peek = self.scan()?;
        Ok(())
    }
    fn unexpected_error(&self) -> ParserError {
        match &self.peek {
            Some((pos, tok)) => {
                return ParserError::UnexpectedToken {
                    pos: pos.clone(),
                    expected: None,
                    found: tok.clone()
                }
            }
            None => return ParserError::UnexpectedEOF,
        }   
    }
}
