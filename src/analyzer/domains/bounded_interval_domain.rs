use std::{cmp::{max, min,}, fmt::Display, ops::{Add, Sub,Mul,Div}};
use crate::{analyzer::AbstractDomain, types::ast::{Num, Operator}};

use super::extended_num::ExtendedNum;


// const LOWER: ExtendedNum = ExtendedNum::Num(-1_000);
// const UPPER: ExtendedNum = ExtendedNum::Num( 1_000);
const LOWER: ExtendedNum = ExtendedNum::NegInf;
const UPPER: ExtendedNum = ExtendedNum::PosInf;


#[derive(Debug,PartialEq,Clone, Copy)]
pub enum BoundedInterval{
    Range(ExtendedNum,ExtendedNum),
    Top,
    Bottom,
}
impl BoundedInterval {
    fn new(mut lower: ExtendedNum, mut upper: ExtendedNum) -> Self{
        if lower == upper && lower != ExtendedNum::NegInf && lower != ExtendedNum::PosInf{
            return BoundedInterval::Range(lower, lower);
        }

        lower =  if lower < LOWER {ExtendedNum::NegInf}
            else if lower <= UPPER {lower}
            else { UPPER };
        upper =  if upper < LOWER { LOWER }
            else if upper <= UPPER {upper}
            else { ExtendedNum::PosInf };
            
        if lower == ExtendedNum::NegInf && upper == ExtendedNum::PosInf {
            BoundedInterval::Top
        } else if lower > upper {
            BoundedInterval::Bottom
        } else {
            BoundedInterval::Range(lower, upper)
        }
    }
}


impl PartialOrd for BoundedInterval{
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<Num> for BoundedInterval{
    fn from(value: Num) -> Self {
        BoundedInterval::new(ExtendedNum::Num(value),ExtendedNum::Num(value))
    }
}

impl AbstractDomain for BoundedInterval{
    fn bottom() -> Self { BoundedInterval::Bottom }
    fn top() -> Self { BoundedInterval::Top }

    fn lub(&self, other: &Self) -> Self {
        match (self,other) {
            (BoundedInterval::Top,_) | (_, BoundedInterval::Top) => BoundedInterval::Top,
            (BoundedInterval::Bottom, i) | (i, BoundedInterval::Bottom) => i.clone(),
            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) =>{
                let lower = min(a,c).clone();
                let upper = max(b,d).clone();
                BoundedInterval::new(lower, upper)
            }
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self,other) {
            (BoundedInterval::Bottom,_) | (_, BoundedInterval::Bottom) => BoundedInterval::Bottom,
            (BoundedInterval::Top, i) | (i, BoundedInterval::Top) => i.clone(),
            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) =>{
                let lower = max(a,c).clone();
                let upper = min(b,d).clone();
                BoundedInterval::new(lower, upper)
            }
        }
    }

    fn backward_abstract_operator(_op: &Operator, _lhs: &Self, _rhs: &Self, _res: &Self) -> (Self, Self) {
        todo!()
    }

    fn gte(lb: &Self) -> Self {
        match lb {
            BoundedInterval::Range(a, _) => BoundedInterval::new(*a, ExtendedNum::PosInf),
            BoundedInterval::Top => BoundedInterval::Top,
            BoundedInterval::Bottom =>  BoundedInterval::Bottom ,
        }
        
    }

    fn lte(ub: &Self) -> Self {
        match ub {
            BoundedInterval::Range(_, b) => BoundedInterval::new(ExtendedNum::NegInf, *b),
            BoundedInterval::Top => BoundedInterval::Top,
            BoundedInterval::Bottom =>  BoundedInterval::Bottom ,
        }
    }

    fn widening(self, other:Self) -> Self {
        match(self, other){
            (BoundedInterval::Bottom, x) | (x, BoundedInterval::Bottom) => x,
            (BoundedInterval::Top, _) | (_, BoundedInterval::Top) => BoundedInterval::Top,

            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) =>{
                let l = if a<=c { a } else { ExtendedNum::NegInf };
                let u = if b>=d { b } else { ExtendedNum::PosInf };
                BoundedInterval::new(l,u)
            }
        }
    }

    fn narrowing(self, other:Self) -> Self {
        match(self, other){
            (BoundedInterval::Bottom, _) | (_, BoundedInterval::Bottom) => Self::Bottom,
            (BoundedInterval::Top, x) | (x, BoundedInterval::Top) => x,

            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) =>{
                let l = if a == ExtendedNum::NegInf { c } else { a };
                let u = if b == ExtendedNum::PosInf { d } else { b };
                BoundedInterval::new(l,u)
            }
        }
    }

    

    
}

impl Display for BoundedInterval{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundedInterval::Top => write!(f, "⊤"),
            BoundedInterval::Bottom => write!(f, "⊥"),
            BoundedInterval::Range(n1, n2) => write!(f, "[{n1},{n2}]"),
        }
    }
}

impl Add for BoundedInterval{
    type Output = BoundedInterval;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedInterval::Bottom,_) | (_, BoundedInterval::Bottom) => BoundedInterval::Bottom,
            (BoundedInterval::Top, _) | (_, BoundedInterval::Top) => BoundedInterval::Top,
            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) => {
                let lower = a.clone()+c.clone();
                let upper = b.clone()+d.clone();
                BoundedInterval::new(lower, upper)
            }
        }
    }
}
impl Sub for BoundedInterval{
    type Output = BoundedInterval;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedInterval::Bottom,_) | (_, BoundedInterval::Bottom) => BoundedInterval::Bottom,
            (BoundedInterval::Top, _) | (_, BoundedInterval::Top) => BoundedInterval::Top,
            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) => {
                let lower = a.clone()-d.clone();
                let upper = b.clone()-c.clone();
                BoundedInterval::new(lower, upper)
            }
        }
    }
}
impl Mul for BoundedInterval{
    type Output = BoundedInterval;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedInterval::Bottom,_) | (_, BoundedInterval::Bottom) => BoundedInterval::Bottom,
            (BoundedInterval::Top, BoundedInterval::Top) => BoundedInterval::Top,
            (BoundedInterval::Top, BoundedInterval::Range(a, b)) | (BoundedInterval::Range(a, b), BoundedInterval::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedInterval::new(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    BoundedInterval::Top
                }
            },
            (BoundedInterval::Range(a, b), BoundedInterval::Range(c, d)) => {
                let points = [a*c, a*d, b*c, b*d];
                let lower = points.iter().min().unwrap().clone();               
                let upper = points.iter().max().unwrap().clone();         
                BoundedInterval::new(lower, upper)
            }
        }
    }
    
}

impl Div for BoundedInterval{
    type Output = BoundedInterval;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundedInterval::Bottom,_) | (_, BoundedInterval::Bottom) => BoundedInterval::Bottom,
            (BoundedInterval::Top, BoundedInterval::Top) => BoundedInterval::Top,
            (BoundedInterval::Top, BoundedInterval::Range(a, b))  =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedInterval::Bottom
                } else {
                    BoundedInterval::Top
                }
            },
            (BoundedInterval::Range(a, b), BoundedInterval::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedInterval::new(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    BoundedInterval::Top
                }
            },
            (n1@BoundedInterval::Range(a, b), n2@BoundedInterval::Range(c, d)) => {
                if ExtendedNum::Num(1)<=c {
                    BoundedInterval::new(min(a/c, a/d), max(b/c, b/d))
                }else if d <= ExtendedNum::Num(-1) {
                    BoundedInterval::new(min(b/c, b/d), max(a/c, a/d))
                } else {
                    let d1 = n1 / n2.glb(&BoundedInterval::new(ExtendedNum::Num(1), ExtendedNum::PosInf));
                    let d2 = n1 / n2.glb(&BoundedInterval::new(ExtendedNum::NegInf, ExtendedNum::Num(-1)));
                    d1.lub(&d2)
                }
            }
        }
    }
}