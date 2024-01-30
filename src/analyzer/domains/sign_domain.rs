use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};
use crate::types::ast::{Operator, Num};
use crate::analyzer::types::domain::AbstractDomain;


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
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<Num> for Sign{
    fn from(value: Num) -> Self {
        match value.cmp(&0){
            std::cmp::Ordering::Less => Sign::Negative,
            std::cmp::Ordering::Equal => Sign::Zero,
            std::cmp::Ordering::Greater => Sign::Positive,
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
    
    fn backward_abstract_operator(_op: &Operator, _lhs: &Self, _rhs: &Self, _res: &Self) -> (Self, Self) {
        todo!()
    }

    fn all_gte(lb: &Self) -> Self {
        match lb{
            Sign::Bottom => Sign::Bottom,
            Sign::Positive => Sign::Positive,
            _ => Sign::Top,
        }
    }

    fn all_lte(ub: &Self) -> Self {
        match ub{
            Sign::Bottom => Sign::Bottom,
            Sign::Negative => Sign::Negative,
            _ => Sign::Top,
        }
    }

}

impl Add for Sign{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,
            (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,

            (s1 ,s2) if s1 == s2 => s1.clone(),
            (Sign::Zero, s) | (s, Sign::Zero) => s.clone(),
            (_, _) => Sign::Top
        }
    }
}
impl Sub for Sign{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,
            (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,

            (Sign::Zero, Sign::Negative) => Sign::Positive,
            (Sign::Zero, Sign::Positive) => Sign::Negative,
            (s, Sign::Zero) => s.clone(),
            
            (Sign::Negative, Sign::Negative) => Sign::Top,
            (Sign::Positive, Sign::Positive) => Sign::Top,
            
            (Sign::Positive, Sign::Negative) => Sign::Positive,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
        }
    }
}
impl Mul for Sign{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,

            (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
            (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,
            
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,                    
        }
    }
}
impl Div for Sign{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Bottom, _) | (_, Sign::Bottom)  => Sign::Bottom,
            
            (_, Sign::Zero)  => Sign::Bottom,
            (Sign::Zero, _)  => Sign::Zero,

            (Sign::Top, _) | (_, Sign::Top )=> Sign::Top,
            
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative, 
        }
    }
}