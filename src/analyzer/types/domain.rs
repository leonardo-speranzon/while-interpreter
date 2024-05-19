use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Debug};
use std::str::FromStr;
use crate::types::ast::{Num, Operator};

pub enum Interval {
    OpenLeft (Num),
    OpenRight (Num),
    Closed (Num,Num),
}

pub trait AbstractDomain : Debug + Display + PartialOrd + Clone + Sized + From<Num> + From<Interval> + FromStr
                           + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self>  {

    fn set_config(config_string: Option<String>) -> Result<(), String> {
        match config_string {
            None => Err(String::from("This domain does not support configuration")),
            Some(_) => Ok(())
        }
    }
    
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self {
        match op {
            Operator::Add => lhs.clone() + rhs.clone(),
            Operator::Sub => lhs.clone() - rhs.clone(),
            Operator::Mul => lhs.clone() * rhs.clone(),
            Operator::Div => lhs.clone() / rhs.clone(),
        }
    }
    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self){
        match op {
            Operator::Add => (
                lhs.glb(&Self::abstract_operator(&Operator::Sub, res, rhs)),
                rhs.glb(&Self::abstract_operator(&Operator::Sub, res, lhs)),
            ),
            Operator::Sub => (
                lhs.glb(&Self::abstract_operator(&Operator::Add, res, rhs)),
                rhs.glb(&Self::abstract_operator(&Operator::Sub, lhs, res)),
            ),
            Operator::Mul => todo!(),
            Operator::Div => todo!(),
        }
    }

    fn widening(self, other:Self) -> Self {
        self.lub(&other) //Trivial widening (possible infinite ascending chain)
    }
    fn narrowing(self, _other:Self) -> Self {
        self //Trivial narrowing (no narrowing)
    }

}

