use super::ast::Num;

#[derive(Debug,Clone,PartialEq)]
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
    AddAssign,
    SubAssign,
    MulAssign,

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
