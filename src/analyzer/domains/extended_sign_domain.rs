use std::{fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};

use crate::types::ast::{Num, Operator};
use crate::analyzer::types::domain::AbstractDomain;

#[derive(Debug,PartialEq,Clone)]
pub struct ExtendedSign{
    positive: bool,
    zero: bool,
    negative: bool,
}

impl Display for ExtendedSign{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtendedSign{positive: false, zero: false, negative: false} =>  write!(f, "⊥"),
            ExtendedSign{positive: false, zero: false, negative: true } =>  write!(f, "<0"),
            ExtendedSign{positive: false, zero: true,  negative: false} =>  write!(f, "0"),
            ExtendedSign{positive: false, zero: true,  negative: true } =>  write!(f, "≤0"),
            ExtendedSign{positive: true,  zero: false, negative: false} =>  write!(f, ">0"),
            ExtendedSign{positive: true,  zero: false, negative: true } =>  write!(f, "≠0"),
            ExtendedSign{positive: true,  zero: true,  negative: false} =>  write!(f, "≥0"),
            ExtendedSign{positive: true,  zero: true,  negative: true } =>  write!(f, "⊤"),
        }
    }
}
impl PartialOrd for ExtendedSign{
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<Num> for ExtendedSign{
    fn from(value: Num) -> Self {
        match value.cmp(&0){
            std::cmp::Ordering::Less => ExtendedSign{positive:false, zero: false, negative: true },
            std::cmp::Ordering::Equal => ExtendedSign{positive:false, zero: true, negative: false },
            std::cmp::Ordering::Greater => ExtendedSign{positive:true, zero: false, negative: false },
        }
    }
}
impl FromStr for ExtendedSign{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self{positive:false, zero:false, negative:true}),
            "0" => Ok(Self{positive:false, zero:true, negative:false}),
            "+" => Ok(Self{positive:true, zero:false, negative:false}),
            "0+" => Ok(Self{positive:true, zero:true, negative:false}),
            "-0" => Ok(Self{positive:false, zero:true, negative:true}),
            "-+" => Ok(Self{positive:true, zero:false, negative:true}),
            _ => Err(format!("Unexpected string representing ExtendedSing domain: {s}"))
        }
    }
}

impl AbstractDomain for ExtendedSign{
    fn bottom() -> Self {
        ExtendedSign{positive:false, zero: false, negative: false }
    }

    fn top() -> Self {
        ExtendedSign{positive:true, zero: true, negative: true }
    }

    fn lub(&self, other: &Self) -> Self {
        ExtendedSign{
            positive: self.positive || other.positive,
            zero: self.zero || other.zero,
            negative: self.negative || other.negative
        }
    }

    fn glb(&self, other: &Self) -> Self {
        ExtendedSign{
            positive: self.positive && other.positive,
            zero: self.zero && other.zero,
            negative: self.negative && other.negative
        }
    }
    
    fn backward_abstract_operator(_op: &Operator, _lhs: &Self, _rhs: &Self, _res: &Self) -> (Self, Self) {
        todo!()
    }

    fn all_gte(lb: &Self) -> Self {
        if lb.negative {ExtendedSign{positive:true, zero: true, negative: true }}
        else if lb.zero {ExtendedSign{positive:true, zero: true, negative: false }}
        else if lb.positive {ExtendedSign{positive:true, zero: false, negative: false }}
        else {ExtendedSign{positive:false, zero: false, negative: false }}
    }

    fn all_lte(ub: &Self) -> Self {
        if ub.positive {ExtendedSign{positive:true, zero: true, negative: true }}
        else if ub.zero {ExtendedSign{positive:false, zero: true, negative: true }}
        else if ub.negative {ExtendedSign{positive:false, zero: false, negative: true }}
        else {ExtendedSign{positive:false, zero: false, negative: false }}
    }

}

impl ExtendedSign{
    fn is_bottom(&self) -> bool{
        return !self.negative && !self.zero && !self.positive
    }
}

impl Add for ExtendedSign{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSign{positive:false, zero: false, negative: false};

        res.negative = self.negative || rhs.negative;

        res.zero = self.zero && rhs.zero
                || self.negative && rhs.positive
                || self.positive && rhs.negative;

        res.positive = self.positive || rhs.positive;
        return res;
    }
}
impl Sub for ExtendedSign{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSign{positive:false, zero: false, negative: false};

        res.negative = self.negative || rhs.positive;

        res.zero = self.zero && rhs.zero
                || self.negative && rhs.negative
                || self.positive && rhs.positive;

        res.positive = self.positive || rhs.negative;

        return res;
    }
}
impl Mul for ExtendedSign{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSign{positive:false, zero: false, negative: false};

        res.negative = (self.positive && rhs.negative)
                    || (self.negative && rhs.positive);

        res.zero = (self.zero && (rhs.negative || rhs.zero || rhs.positive))
                || (rhs.zero && (self.negative || self.zero || self.positive)); //The other side must not be bottom

        res.positive = (self.positive && rhs.positive)
                    || (self.negative && rhs.negative);
        return res;
    }
}
impl Div for ExtendedSign{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSign{positive:false, zero: false, negative: false};

        res.negative = (self.positive && rhs.negative)
                    || (self.negative && rhs.positive);

        res.zero = self.zero && (rhs.negative || rhs.positive); //The other side must not be bottom or zero

        res.positive = (self.positive && rhs.positive)
                    || (self.negative && rhs.negative);
        return res;
    }
}