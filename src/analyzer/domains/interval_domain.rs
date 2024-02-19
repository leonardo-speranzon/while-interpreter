use std::{cmp::{max, min, Ordering}, fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};
use iter_tools::Itertools as _;

use crate::types::ast::{Num, Operator};
use crate::analyzer::types::domain::AbstractDomain;
use super::extended_num::ExtendedNum;


#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Interval{
    Range(ExtendedNum,ExtendedNum),
    Top,
    Bottom,
}
impl Interval {
    fn new(lower: ExtendedNum, upper: ExtendedNum) -> Self{
        if lower == ExtendedNum::NegInf && upper == ExtendedNum::PosInf {
            Interval::Top
        } else if lower > upper {
            Interval::Bottom
        } else {
            Interval::Range(lower,upper)
        }
    }
}


impl Display for Interval{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Top => write!(f, "⊤"),
            Interval::Bottom => write!(f, "⊥"),
            Interval::Range(n1, n2) => write!(f, "[{n1},{n2}]"),
        }
    }
}
impl PartialOrd for Interval{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Interval::Bottom, Interval::Bottom) | (Interval::Top, Interval::Top) => Some(Ordering::Equal),
            (Interval::Top, _) | (_, Interval::Bottom) => Some(Ordering::Greater),
            (Interval::Bottom, _) | (_, Interval::Top) => Some(Ordering::Less),
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                if a==c && b==d { Some(Ordering::Equal) }
                else if a<=c && b>=d { Some(Ordering::Greater)}
                else if a>=c && b<=d { Some(Ordering::Less)}
                else { None }
            },
        }
    }
}

impl From<Num> for Interval{
    fn from(value: Num) -> Self {
        Interval::Range(ExtendedNum::Num(value),ExtendedNum::Num(value))
    }
}
impl FromStr for Interval{
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

impl AbstractDomain for Interval{
    fn bottom() -> Self { Interval::Bottom }
    fn top() -> Self { Interval::Top }

    fn lub(&self, other: &Self) -> Self {
        match (self,other) {
            (Interval::Top,_) | (_, Interval::Top) => Interval::Top,
            (Interval::Bottom, i) | (i, Interval::Bottom) => i.clone(),
            (Interval::Range(a, b), Interval::Range(c, d)) =>{
                let lower = min(a,c).clone();
                let upper = max(b,d).clone();
                Interval::new(lower, upper)
            }
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self,other) {
            (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Top, i) | (i, Interval::Top) => i.clone(),
            (Interval::Range(a, b), Interval::Range(c, d)) =>{
                let lower = max(a,c).clone();
                let upper = min(b,d).clone();
                Interval::new(lower, upper)
            }
        }
    }

    fn backward_abstract_operator(_op: &Operator, _lhs: &Self, _rhs: &Self, _res: &Self) -> (Self, Self) {
        todo!()
    }

    fn all_gte(lb: &Self) -> Self {
        match lb {
            Interval::Range(a, _) => Interval::new(*a, ExtendedNum::PosInf),
            Interval::Top => Interval::Top,
            Interval::Bottom =>  Interval::Bottom ,
        }
        
    }

    fn all_lte(ub: &Self) -> Self {
        match ub {
            Interval::Range(_, b) => Interval::new(ExtendedNum::NegInf, *b),
            Interval::Top => Interval::Top,
            Interval::Bottom =>  Interval::Bottom ,
        }
    }

    fn widening(self, other:Self) -> Self {
        match(self, other){
            (Interval::Bottom, x) | (x, Interval::Bottom) => x,
            (Interval::Top, _) | (_, Interval::Top) => Interval::Top,

            (Interval::Range(a, b), Interval::Range(c, d)) =>{
                let l = if a<=c { a } else { ExtendedNum::NegInf };
                let u = if b>=d { b } else { ExtendedNum::PosInf };
                Interval::new(l,u)
            }
        }
    }

    
}

impl Add for Interval{
    type Output = Interval;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Top, _) | (_, Interval::Top) => Interval::Top,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let lower = a.clone()+c.clone();
                let upper = b.clone()+d.clone();
                Interval::new(lower, upper)
            }
        }
    }
}
impl Sub for Interval{
    type Output = Interval;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Top, _) | (_, Interval::Top) => Interval::Top,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let lower = a.clone()-d.clone();
                let upper = b.clone()-c.clone();
                Interval::new(lower, upper)
            }
        }
    }
}
impl Mul for Interval{
    type Output = Interval;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Top, Interval::Top) => Interval::Top,
            (Interval::Top, Interval::Range(a, b)) | (Interval::Range(a, b), Interval::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    Interval::Range(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    Interval::Top
                }
            },
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let points = [a*c, a*d, b*c, b*d];
                let lower = points.iter().min().unwrap().clone();               
                let upper = points.iter().max().unwrap().clone();         
                Interval::new(lower, upper)
            }
        }
    }
    
}

impl Div for Interval{
    type Output = Interval;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Top, Interval::Top) => Interval::Top,
            (Interval::Top, Interval::Range(a, b))  =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    Interval::Bottom
                } else {
                    Interval::Top
                }
            },
            (Interval::Range(a, b), Interval::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    Interval::Range(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    Interval::Top
                }
            },
            (n1@Interval::Range(a, b), n2@Interval::Range(c, d)) => {
                if ExtendedNum::Num(1)<=c {
                    Interval::new(min(a/c, a/d), max(b/c, b/d))
                }else if d <= ExtendedNum::Num(-1) {
                    Interval::new(min(b/c, b/d), max(a/c, a/d))
                } else {
                    let d1 = n1 / n2.glb(&Interval::new(ExtendedNum::Num(1), ExtendedNum::PosInf));
                    let d2 = n1 / n2.glb(&Interval::new(ExtendedNum::NegInf, ExtendedNum::Num(-1)));
                    d1.lub(&d2)
                }
            }
        }
    }
}