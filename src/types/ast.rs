use std::{fmt::{Display, Debug}, str::FromStr};

use iter_tools::Itertools as _;

pub type Var = String;
pub type Num = i128;


#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LitInterval (Num,Num);

pub trait NumLiteral: Debug + Display + FromStr + Copy + From<Num> + PartialEq 
    {}


#[derive(Debug, Clone)]
pub enum Operator{
    Add,Sub,Mul,Div
}

#[derive(Debug, Clone)]
pub enum Statement<D> {
    Assign (Var, Box<Aexpr<D>>),
    Skip,
    Compose    (Box<Statement<D>>, Box<Statement<D>>),
    IfThenElse (Box<Bexpr<D>>, Box<Statement<D>>, Box<Statement<D>>),
    While      (Box<Bexpr<D>>, Box<Statement<D>>),
}

#[derive(Debug, Clone)]
pub enum PrePostOp {
    Inc, Dec
}
pub type PreOp = PrePostOp;
pub type PostOp = PrePostOp;

#[derive(Debug, Clone)]
pub enum Aexpr<D>  {
    Lit  (D),
    Var  (Var),
    PreOp (PreOp, Var),
    PostOp (PostOp, Var),
    BinOp (Operator, Box<Aexpr<D>>, Box<Aexpr<D>>),
}

#[derive(Debug, Clone)]
pub enum Bexpr<D> {
    True,
    False,
    Equal  (Box<Aexpr<D>>, Box<Aexpr<D>>),
    LessEq (Box<Aexpr<D>>, Box<Aexpr<D>>),
    Not    (Box<Bexpr<D>>),
    And    (Box<Bexpr<D>>, Box<Bexpr<D>>),
}




impl NumLiteral for Num {}


impl From<Num> for LitInterval {
    fn from(value: Num) -> Self {
        LitInterval(value, value)
    }
}
impl FromStr for LitInterval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<Num>(){
            return Ok(LitInterval(n,n));
        };
        let mut chars = s.chars();
        match chars.next() {
            Some('[') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        let lower: String = chars.take_while_ref(|c|c!=&',').collect();
        
        match chars.next() {
            Some(',') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        match chars.next_back() {
            Some(']') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }

        let upper: String = chars.collect();

        let lower = lower.parse::<Num>().map_err(|e|e.to_string())?;
        let upper = upper.parse::<Num>().map_err(|e|e.to_string())?;
        Ok(LitInterval(lower,upper))
    }
}
impl Display for LitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}
impl NumLiteral for LitInterval {}