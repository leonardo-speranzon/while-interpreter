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
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    

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
        
        //FIXME
        lex.peek = lex.scan().unwrap();
        return lex;
    }
}
impl<'a> From<File> for MyLexer<'a> {
    fn from(value: File) -> Self {
        let reader = BufReader::new(value);
        let it = reader.lines().enumerate()
        .map(|(i,line)| 
            //FIXME
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
        
        //FIXME
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
            Some((_, _, d@'0'..='9')) => {
                let mut digits = d.to_string();
                while let Some((_, _, d)) = self.chars.next_if(|(_, _, c)| c.is_ascii_digit()){
                    digits.push(d)
                }
                Token::Num((digits).parse().unwrap())//By construction should be valid                
            }
            Some((_, _, c@('a'..='z' | 'A'..='Z' | '_'))) => {
                let mut word = c.to_string();
                while let Some((_, _, c)) = self.chars.next_if(|(_, _, c)| c.is_ascii_alphanumeric() || c == &'_'){
                    word.push(c)
                }
                match_keyword(&word).unwrap_or(Token::Id(word))
            }
            Some((_, _, c@('='|'<'|'>'|'!'|'-'|'+'|'*'|'('|')'|'{'|'}'|':'|';'))) =>{
                let mut symbol = c.to_string();
                let mut  last_valid_tok = match_symbol(&symbol);

                while let Some((_, _, c@('='|'<'|'>'|'!'|'-'|'+'|'*'|'('|')'|'{'|'}'|':'|';'))) = self.chars.peek() {
                    symbol.push(c.clone());
                    match match_symbol(&symbol){
                        Some(tok) => {
                            self.chars.next();
                            last_valid_tok = Some(tok);
                        },
                        None => break,
                    }
                    if let Some(tok) = match_symbol(&symbol)  {
                        last_valid_tok = Some(tok)
                    }
                }

                match last_valid_tok  {
                    Some(tok) => tok,
                    None => return Err(ParserError::UnknownSymbol { 
                        pos: start_pos.unwrap(),
                        symbol: symbol.pop().unwrap()
                    })
                }
            }
            Some((_,_,c))=> return Err(ParserError::UnknownSymbol{
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
        if std::env::var("print-token").is_ok_and(|s|s=="true") {
            print!("{:?} ", self.peek.as_ref().unwrap().1)
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

fn match_symbol(s: &str)-> Option<Token>{
    match s{
        ";" => Some(Token::Semicolon),
        "(" => Some(Token::BracketOpen),
        ")" => Some(Token::BracketClose),
        "{" => Some(Token::CurlyOpen),
        "}" => Some(Token::CurlyClose),

        "*" => Some(Token::Mul),
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),

        "==" => Some(Token::Eq),
        "<" => Some(Token::Lt),
        ">" => Some(Token::Gt),
        "<=" => Some(Token::Lte),
        ">=" => Some(Token::Gte),
        "!=" => Some(Token::Neq),

        ":=" => Some(Token::Assign),
        "+=" => Some(Token::Inc),
        "-=" => Some(Token::Dec),
        _ => None
    }
}

fn match_keyword(kw: &str)->Option<Token>{
    match kw {
        "if" => Some(Token::If),
        "then" => Some(Token::Then),
        "else" => Some(Token::Else),

        "while" => Some(Token::While),
        "do" => Some(Token::Do),
        "repeat" => Some(Token::Repeat),
        "until" => Some(Token::Until),
        "for" => Some(Token::For),
        "skip" => Some(Token::Skip),                    
        
        "not" => Some(Token::Not),
        "and" => Some(Token::And),
        "or" => Some(Token::Or),

        "true" => Some(Token::True),
        "false" => Some(Token::False),
        
        _ => None,
    }
}