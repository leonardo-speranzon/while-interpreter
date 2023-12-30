use std::fmt::Display;

use crate::{types::ast::{Operator, Num}, analyzer::AbstractDomain};



#[derive(Debug,PartialEq,Clone)]
pub enum Sign{
    Top,
    Bottom,
    Positive,
    Zero,
    Negative,
}
impl Display for Sign{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl PartialOrd for Sign{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<Num> for Sign{
    fn from(value: Num) -> Self {
        if value > 0{
            Sign::Positive
        } else if value == 0 {
            Sign::Zero
        } else {
            Sign::Negative
        }
    }
}

impl AbstractDomain for Sign{
    fn bottom() -> Self {
        Sign::Bottom
    }

    fn top() -> Self {
        Sign::Top
    }

    fn lub(&self, other: &Self) -> Self {
        match (self, other) {
            (Sign::Bottom, s2) => s2.clone(),
            (s1, Sign::Bottom) => s1.clone(),
            (s1 ,s2) if s1 == s2 => s1.clone(),
            (_, _) => Sign::Top
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self, other) {
            (Sign::Top, s2) => s2.clone(),
            (s1, Sign::Top) => s1.clone(),
            (s1 ,s2) if s1 == s2 => s1.clone(),
            (_, _) => Sign::Bottom
        }
    }

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self {
        match op{
            Operator::Add => match (lhs, rhs) {
                (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,
                (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,

                (s1 ,s2) if s1 == s2 => s1.clone(),
                (Sign::Zero, s) | (s, Sign::Zero) => s.clone(),
                (_, _) => Sign::Top
            },
            Operator::Sub => match (lhs, rhs) {
                (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,
                (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,

                (Sign::Zero, Sign::Negative) => Sign::Positive,
                (Sign::Zero, Sign::Positive) => Sign::Negative,
                (s, Sign::Zero) => s.clone(),
                
                (Sign::Negative, Sign::Negative) => Sign::Top,
                (Sign::Positive, Sign::Positive) => Sign::Top,
                
                (Sign::Positive, Sign::Negative) => Sign::Positive,
                (Sign::Negative, Sign::Positive) => Sign::Negative,
            },
            Operator::Mul => match (lhs, rhs) {
                (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,

                (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
                (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,
                
                (Sign::Negative, Sign::Negative) => Sign::Positive,
                (Sign::Positive, Sign::Positive) => Sign::Positive,
                
                (Sign::Positive, Sign::Negative) => Sign::Negative,
                (Sign::Negative, Sign::Positive) => Sign::Negative,                    
            },
        }
    }

    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self) {

        match op {
            Operator::Add => todo!(),
            Operator::Sub => todo!(),
            Operator::Mul => todo!(),
        }
    }
}
