use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};
use crate::{analyzer::types::domain::Interval, types::ast::Num};
use crate::analyzer::types::domain::AbstractDomain;


#[derive(Debug,PartialEq,Clone,Copy)]
pub enum SignDomain{
    Top,
    Bottom,
    Positive,
    Zero,
    Negative,
}
impl Display for SignDomain{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl PartialOrd for SignDomain{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (s1, s2) if s1 == s2 => Some(Ordering::Equal),

            (SignDomain::Top, _) | (_, SignDomain::Bottom)  => Some(Ordering::Greater),
            (SignDomain::Bottom, _) | (_, SignDomain::Top)  => Some(Ordering::Less),
            
            _ => None
        }
    }
}

impl From<Num> for SignDomain{
    fn from(value: Num) -> Self {
        match value.cmp(&0){
            std::cmp::Ordering::Less => SignDomain::Negative,
            std::cmp::Ordering::Equal => SignDomain::Zero,
            std::cmp::Ordering::Greater => SignDomain::Positive,
        }
    }
}

impl From<Interval> for SignDomain {
    fn from(value: Interval) -> Self {
        match value {
            Interval::OpenLeft(max) => {
                if max<0 { Self::Negative }
                else { Self::Top }
            },
            Interval::OpenRight(min) => {
                if min>0 { Self::Positive }
                else {Self::Top}
            },
            Interval::Closed(min, max) => {
                if min < 0 && max < 0 { Self::Negative }
                else if min == 0 && max == 0 {Self::Zero}
                else if min > 0 && max > 0 {Self::Positive}
                else {Self::Top}
            },
        }
    }
}

impl FromStr for SignDomain{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Negative),
            "0" => Ok(Self::Zero),
            "+" => Ok(Self::Positive),
            _ => Err(format!("Unexpected string representing Sign domain: {s}"))
        }
    }
}

impl AbstractDomain for SignDomain{
    fn bottom() -> Self {
        SignDomain::Bottom
    }

    fn top() -> Self {
        SignDomain::Top
    }

    fn lub(self, other: Self) -> Self {
        match (self, other) {
            (SignDomain::Bottom, s2) => s2,
            (s1, SignDomain::Bottom) => s1,
            (s1 ,s2) if s1 == s2 => s1,
            (_, _) => SignDomain::Top
        }
    }

    fn glb(self, other: Self) -> Self {
        match (self, other) {
            (SignDomain::Top, s2) => s2,
            (s1, SignDomain::Top) => s1,
            (s1 ,s2) if s1 == s2 => s1,
            (_, _) => SignDomain::Bottom
        }
    }


}

impl Add for SignDomain{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SignDomain::Bottom, _) | (_, SignDomain::Bottom)  => SignDomain::Bottom,
            (SignDomain::Top, _) | (_, SignDomain::Top )=> SignDomain::Top,

            (s1 ,s2) if s1 == s2 => s1,
            (SignDomain::Zero, s) | (s, SignDomain::Zero) => s,
            (_, _) => SignDomain::Top
        }
    }
}
impl Sub for SignDomain{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SignDomain::Bottom, _) | (_, SignDomain::Bottom)  => SignDomain::Bottom,
            (SignDomain::Top, _) | (_, SignDomain::Top )=> SignDomain::Top,

            (SignDomain::Zero, SignDomain::Negative) => SignDomain::Positive,
            (SignDomain::Zero, SignDomain::Positive) => SignDomain::Negative,
            (s, SignDomain::Zero) => s,
            
            (SignDomain::Negative, SignDomain::Negative) => SignDomain::Top,
            (SignDomain::Positive, SignDomain::Positive) => SignDomain::Top,
            
            (SignDomain::Positive, SignDomain::Negative) => SignDomain::Positive,
            (SignDomain::Negative, SignDomain::Positive) => SignDomain::Negative,
        }
    }
}
impl Mul for SignDomain{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SignDomain::Bottom, _) | (_, SignDomain::Bottom)  => SignDomain::Bottom,

            (SignDomain::Zero, _) | (_, SignDomain::Zero) => SignDomain::Zero,
            (SignDomain::Top, _) | (_, SignDomain::Top )=> SignDomain::Top,
            
            (SignDomain::Negative, SignDomain::Negative) => SignDomain::Positive,
            (SignDomain::Positive, SignDomain::Positive) => SignDomain::Positive,
            
            (SignDomain::Positive, SignDomain::Negative) => SignDomain::Negative,
            (SignDomain::Negative, SignDomain::Positive) => SignDomain::Negative,                    
        }
    }
}
impl Div for SignDomain{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SignDomain::Bottom, _) | (_, SignDomain::Bottom)  => SignDomain::Bottom,
            
            (_, SignDomain::Zero)  => SignDomain::Bottom,
            (SignDomain::Zero, _)  => SignDomain::Zero,

            (SignDomain::Top, _) | (_, SignDomain::Top )=> SignDomain::Top,
            
            (SignDomain::Negative, SignDomain::Negative) => SignDomain::Positive,
            (SignDomain::Positive, SignDomain::Positive) => SignDomain::Positive,
            
            (SignDomain::Positive, SignDomain::Negative) => SignDomain::Negative,
            (SignDomain::Negative, SignDomain::Positive) => SignDomain::Negative, 
        }
    }
}