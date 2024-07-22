use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};

use crate::{analyzer::types::domain::Interval, types::ast::Num};
use crate::analyzer::types::domain::AbstractDomain;


#[derive(Debug,PartialEq,Clone,Copy)]
pub struct ExtendedSignDomain{
    positive: bool,
    zero: bool,
    negative: bool,
}

impl Display for ExtendedSignDomain{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtendedSignDomain{positive: false, zero: false, negative: false} =>  write!(f, "⊥"),
            ExtendedSignDomain{positive: false, zero: false, negative: true } =>  write!(f, "<0"),
            ExtendedSignDomain{positive: false, zero: true,  negative: false} =>  write!(f, "0"),
            ExtendedSignDomain{positive: false, zero: true,  negative: true } =>  write!(f, "≤0"),
            ExtendedSignDomain{positive: true,  zero: false, negative: false} =>  write!(f, ">0"),
            ExtendedSignDomain{positive: true,  zero: false, negative: true } =>  write!(f, "≠0"),
            ExtendedSignDomain{positive: true,  zero: true,  negative: false} =>  write!(f, "≥0"),
            ExtendedSignDomain{positive: true,  zero: true,  negative: true } =>  write!(f, "⊤"),
        }
    }
}
impl PartialOrd for ExtendedSignDomain{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other { Some(Ordering::Equal) }
        else if imply(other.positive, self.positive) && imply(other.zero, self.zero) && imply(other.negative, self.negative){
            Some(Ordering::Greater)
        } else if imply(self.positive, other.positive) && imply(self.zero, other.zero) && imply(self.negative, other.negative) {
            Some(Ordering::Less)
        }else {
            None
        }
    }
}
fn imply(b1: bool, b2: bool) -> bool{
    !b1 || b2 
}

impl From<Num> for ExtendedSignDomain{
    fn from(value: Num) -> Self {
        match value.cmp(&0){
            std::cmp::Ordering::Less => ExtendedSignDomain{positive:false, zero: false, negative: true },
            std::cmp::Ordering::Equal => ExtendedSignDomain{positive:false, zero: true, negative: false },
            std::cmp::Ordering::Greater => ExtendedSignDomain{positive:true, zero: false, negative: false },
        }
    }
}

impl From<Interval> for ExtendedSignDomain {
    fn from(value: Interval) -> Self {
        match value {
            Interval::OpenLeft(max) => {
                if max<0 { Self{positive:false,zero:false,negative:true} }
                else if max == 0 {Self{positive:false,zero:true,negative:true}}
                else {Self{positive:true,zero:true,negative:true}}
            },
            Interval::OpenRight(min) => {
                if min<0 { Self{positive:true,zero:true,negative:true} }
                else if min == 0 {Self{positive:true,zero:true,negative:false}}
                else {Self{positive:true,zero:false,negative:false}}
            },
            Interval::Closed(min, max) => {
                if min == 0 && max == 0 { Self{positive:false,zero:true,negative:false} }
                else if min == 0 {Self{positive:true,zero:true,negative:false}}
                else if min > 0 {Self{positive:true,zero:false,negative:false}}
                else if max == 0 {Self{positive:false,zero:true,negative:true}}
                else if max < 0 {Self{positive:false,zero:false,negative:true}}
                else {Self{positive:true,zero:true,negative:true}}
            },
        }
    }
}

impl FromStr for ExtendedSignDomain{
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

impl AbstractDomain for ExtendedSignDomain{
    fn bottom() -> Self {
        ExtendedSignDomain{positive:false, zero: false, negative: false }
    }

    fn top() -> Self {
        ExtendedSignDomain{positive:true, zero: true, negative: true }
    }

    fn lub(self, other: Self) -> Self {
        ExtendedSignDomain{
            positive: self.positive || other.positive,
            zero: self.zero || other.zero,
            negative: self.negative || other.negative
        }
    }

    fn glb(self, other: Self) -> Self {
        ExtendedSignDomain{
            positive: self.positive && other.positive,
            zero: self.zero && other.zero,
            negative: self.negative && other.negative
        }
    }
    

}

impl ExtendedSignDomain{
    fn is_bottom(&self) -> bool{
        return !self.negative && !self.zero && !self.positive
    }
}

impl Add for ExtendedSignDomain{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSignDomain{positive:false, zero: false, negative: false};

        res.negative = self.negative || rhs.negative;

        res.zero = self.zero && rhs.zero
                || self.negative && rhs.positive
                || self.positive && rhs.negative;

        res.positive = self.positive || rhs.positive;
        return res;
    }
}
impl Sub for ExtendedSignDomain{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSignDomain{positive:false, zero: false, negative: false};

        res.negative = self.negative || rhs.positive;

        res.zero = self.zero && rhs.zero
                || self.negative && rhs.negative
                || self.positive && rhs.positive;

        res.positive = self.positive || rhs.negative;

        return res;
    }
}
impl Mul for ExtendedSignDomain{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSignDomain{positive:false, zero: false, negative: false};

        res.negative = (self.positive && rhs.negative)
                    || (self.negative && rhs.positive);

        res.zero = (self.zero && (rhs.negative || rhs.zero || rhs.positive))
                || (rhs.zero && (self.negative || self.zero || self.positive)); //The other side must not be bottom

        res.positive = (self.positive && rhs.positive)
                    || (self.negative && rhs.negative);
        return res;
    }
}
impl Div for ExtendedSignDomain{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if self.is_bottom() || rhs.is_bottom() { return Self::bottom(); }

        let mut res = ExtendedSignDomain{positive:false, zero: false, negative: false};

        res.negative = (self.positive && rhs.negative)
                    || (self.negative && rhs.positive);

        res.zero = self.zero && (rhs.negative || rhs.positive); //The other side must not be bottom or zero

        res.positive = (self.positive && rhs.positive)
                    || (self.negative && rhs.negative);
        return res;
    }
}