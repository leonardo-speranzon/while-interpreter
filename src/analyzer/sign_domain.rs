use crate::types::ast::Operator;

use super::AbstractDomain;

#[derive(PartialEq,Clone)]
enum Sign{
    Top,
    Bottom,
    Positive,
    Zero,
    Negative,
}

impl PartialOrd for Sign{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
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
        todo!()
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

                (Sign::Zero, s) | (s, Sign::Zero) => s.clone(),
                
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
