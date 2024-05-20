#[derive(Debug,Clone,PartialEq)]
pub enum Token<N> {
    Id(String),
    Lit(N),
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
    Div,
    
    AddAssign,
    SubAssign,
    MulAssign,
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
