use std::{fmt::{Display, Debug}, ops::{Mul, Add, Sub}};

pub type Num = i128;
pub type Var = String;

pub trait ConcreteType: Debug + Display + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Ord + From<i128>{}
impl<T> ConcreteType for T where T: Debug + Display + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> +  Ord + From<i128> {}

#[derive(Debug, Clone)]
pub enum Statement<N: ConcreteType> {
    Assign (Var, Box<Aexpr<N>>),
    Skip,
    Compose    (Box<Statement<N>>, Box<Statement<N>>),
    IfThenElse (Box<Bexpr<N>>, Box<Statement<N>>, Box<Statement<N>>),
    While      (Box<Bexpr<N>>, Box<Statement<N>>),
}

#[derive(Debug, Clone)]
pub enum Aexpr<N: ConcreteType>  {
    Num  (N),
    Var  (Var),
    Add  (Box<Aexpr<N>>, Box<Aexpr<N>>),
    Mul (Box<Aexpr<N>>, Box<Aexpr<N>>),
    Sub  (Box<Aexpr<N>>, Box<Aexpr<N>>),
}

#[derive(Debug, Clone)]
pub enum Bexpr<N: ConcreteType> {
    True,
    False,
    Equal  (Box<Aexpr<N>>, Box<Aexpr<N>>),
    LessEq (Box<Aexpr<N>>, Box<Aexpr<N>>),
    Not    (Box<Bexpr<N>>),
    And    (Box<Bexpr<N>>, Box<Bexpr<N>>),
}


