use std::{cmp::{max, min, Ordering}, fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};
use iter_tools::Itertools as _;

use crate::{analyzer::types::domain::Interval, types::ast::Num};
use crate::analyzer::types::domain::AbstractDomain;
use super::extended_num::ExtendedNum;


#[derive(Debug,PartialEq,Clone, Copy)]
pub enum IntervalDomain{
    Range(ExtendedNum,ExtendedNum),
    Top,
    Bottom,
}
impl IntervalDomain {
    fn new(lower: ExtendedNum, upper: ExtendedNum) -> Self{
        if lower == ExtendedNum::NegInf && upper == ExtendedNum::PosInf {
            IntervalDomain::Top
        } else if lower > upper {
            IntervalDomain::Bottom
        } else {
            IntervalDomain::Range(lower,upper)
        }
    }
}


impl Display for IntervalDomain{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalDomain::Top => write!(f, "⊤"),
            IntervalDomain::Bottom => write!(f, "⊥"),
            IntervalDomain::Range(n1, n2) => write!(f, "[{n1},{n2}]"),
        }
    }
}
impl PartialOrd for IntervalDomain{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (IntervalDomain::Bottom, IntervalDomain::Bottom) | (IntervalDomain::Top, IntervalDomain::Top) => Some(Ordering::Equal),
            (IntervalDomain::Top, _) | (_, IntervalDomain::Bottom) => Some(Ordering::Greater),
            (IntervalDomain::Bottom, _) | (_, IntervalDomain::Top) => Some(Ordering::Less),
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) => {
                if a==c && b==d { Some(Ordering::Equal) }
                else if a<=c && b>=d { Some(Ordering::Greater)}
                else if a>=c && b<=d { Some(Ordering::Less)}
                else { None }
            },
        }
    }
}

impl From<Num> for IntervalDomain{
    fn from(value: Num) -> Self {
        IntervalDomain::Range(ExtendedNum::Num(value),ExtendedNum::Num(value))
    }
}

impl From<Interval> for IntervalDomain {
    fn from(value: Interval) -> Self {
        match value {
            Interval::OpenLeft(max) => Self::Range(ExtendedNum::NegInf, ExtendedNum::Num(max)),
            Interval::OpenRight(min) => Self::Range(ExtendedNum::Num(min), ExtendedNum::PosInf),
            Interval::Closed(min, max) => Self::Range(ExtendedNum::Num(min), ExtendedNum::Num(max)),
        }
    }
}

impl FromStr for IntervalDomain{
    type Err = String;

    // "[1,10]", "[-inf,10]", "[-inf,inf]"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<Num>(){
            return Ok(Self::new(ExtendedNum::Num(n),ExtendedNum::Num(n)));
        };
        let mut chars = s.chars();
        match chars.next() {
            Some('[') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        let lower: String = chars.take_while_ref(|c|c!=&',').collect();
        
        match chars.next() {
            Some(',') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        match chars.next_back() {
            Some(']') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }

        let upper: String = chars.collect();

        let lower = lower.parse()?;
        let upper = upper.parse()?;
        Ok(Self::new(lower,upper))
    }
}

impl AbstractDomain for IntervalDomain{
    fn bottom() -> Self { IntervalDomain::Bottom }
    fn top() -> Self { IntervalDomain::Top }

    fn lub(&self, other: &Self) -> Self {
        match (self,other) {
            (IntervalDomain::Top,_) | (_, IntervalDomain::Top) => IntervalDomain::Top,
            (IntervalDomain::Bottom, i) | (i, IntervalDomain::Bottom) => i.clone(),
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) =>{
                let lower = min(a,c).clone();
                let upper = max(b,d).clone();
                IntervalDomain::new(lower, upper)
            }
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self,other) {
            (IntervalDomain::Bottom,_) | (_, IntervalDomain::Bottom) => IntervalDomain::Bottom,
            (IntervalDomain::Top, i) | (i, IntervalDomain::Top) => i.clone(),
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) =>{
                let lower = max(a,c).clone();
                let upper = min(b,d).clone();
                IntervalDomain::new(lower, upper)
            }
        }
    }


    fn widening(self, other:Self) -> Self {
        match(self, other){
            (IntervalDomain::Bottom, x) | (x, IntervalDomain::Bottom) => x,
            (IntervalDomain::Top, _) | (_, IntervalDomain::Top) => IntervalDomain::Top,

            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) =>{
                let l = if a<=c { a } else { ExtendedNum::NegInf };
                let u = if b>=d { b } else { ExtendedNum::PosInf };
                IntervalDomain::new(l,u)
            }
        }
    }

    
}

impl Add for IntervalDomain{
    type Output = IntervalDomain;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (IntervalDomain::Bottom,_) | (_, IntervalDomain::Bottom) => IntervalDomain::Bottom,
            (IntervalDomain::Top, _) | (_, IntervalDomain::Top) => IntervalDomain::Top,
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) => {
                let lower = a.clone()+c.clone();
                let upper = b.clone()+d.clone();
                IntervalDomain::new(lower, upper)
            }
        }
    }
}
impl Sub for IntervalDomain{
    type Output = IntervalDomain;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (IntervalDomain::Bottom,_) | (_, IntervalDomain::Bottom) => IntervalDomain::Bottom,
            (IntervalDomain::Top, _) | (_, IntervalDomain::Top) => IntervalDomain::Top,
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) => {
                let lower = a.clone()-d.clone();
                let upper = b.clone()-c.clone();
                IntervalDomain::new(lower, upper)
            }
        }
    }
}
impl Mul for IntervalDomain{
    type Output = IntervalDomain;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (IntervalDomain::Bottom,_) | (_, IntervalDomain::Bottom) => IntervalDomain::Bottom,
            (IntervalDomain::Top, IntervalDomain::Top) => IntervalDomain::Top,
            (IntervalDomain::Top, IntervalDomain::Range(a, b)) | (IntervalDomain::Range(a, b), IntervalDomain::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    IntervalDomain::Range(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    IntervalDomain::Top
                }
            },
            (IntervalDomain::Range(a, b), IntervalDomain::Range(c, d)) => {
                let points = [a*c, a*d, b*c, b*d];
                let lower = points.iter().min().unwrap().clone();               
                let upper = points.iter().max().unwrap().clone();         
                IntervalDomain::new(lower, upper)
            }
        }
    }
    
}

impl Div for IntervalDomain{
    type Output = IntervalDomain;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (IntervalDomain::Bottom,_) | (_, IntervalDomain::Bottom) => IntervalDomain::Bottom,
            (IntervalDomain::Top, IntervalDomain::Top) => IntervalDomain::Top,
            (IntervalDomain::Top, IntervalDomain::Range(a, b))  =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    IntervalDomain::Bottom
                } else {
                    IntervalDomain::Top
                }
            },
            (IntervalDomain::Range(a, b), IntervalDomain::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    IntervalDomain::Range(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    IntervalDomain::Top
                }
            },
            (n1@IntervalDomain::Range(a, b), n2@IntervalDomain::Range(c, d)) => {
                if ExtendedNum::Num(1)<=c {
                    IntervalDomain::new(min(a/c, a/d), max(b/c, b/d))
                }else if d <= ExtendedNum::Num(-1) {
                    IntervalDomain::new(min(b/c, b/d), max(a/c, a/d))
                } else {
                    let d1 = n1 / n2.glb(&IntervalDomain::new(ExtendedNum::Num(1), ExtendedNum::PosInf));
                    let d2 = n1 / n2.glb(&IntervalDomain::new(ExtendedNum::NegInf, ExtendedNum::Num(-1)));
                    d1.lub(&d2)
                }
            }
        }
    }
}