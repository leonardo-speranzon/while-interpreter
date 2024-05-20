use std::{fmt::{Display, Debug}, str::FromStr};


pub type Var = String;
pub type Num = i128;



pub trait NumLiteral: Debug + Display + FromStr + Copy + From<Num> + PartialEq {}
impl NumLiteral for Num {}


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




