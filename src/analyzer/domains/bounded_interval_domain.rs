use std::{ cmp::{max, min, Ordering,}, fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};
use iter_tools::Itertools;
use once_cell::sync::OnceCell;

use crate::{analyzer::types::domain::{AbstractDomain, Interval}, types::ast::{Num, Operator}};

use super::extended_num::ExtendedNum;


static BI_LOWER: OnceCell<ExtendedNum> = OnceCell::new();
static BI_UPPER: OnceCell<ExtendedNum> = OnceCell::new();

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum BoundedIntervalDomain{
    Range(ExtendedNum,ExtendedNum),
    Top,
    Bottom,
}
impl BoundedIntervalDomain {
    fn new(mut lower: ExtendedNum, mut upper: ExtendedNum) -> Self{
        match (BI_LOWER.get(), BI_UPPER.get()){
            (Some(lower_bound), Some(upper_bound)) => {       
                if lower == upper && lower != ExtendedNum::NegInf && lower != ExtendedNum::PosInf{
                    return BoundedIntervalDomain::Range(lower, lower);
                }

                lower =  if lower < *lower_bound {ExtendedNum::NegInf}
                    else if lower <= *upper_bound {lower}
                    else { *upper_bound };
                upper =  if upper < *lower_bound { *lower_bound }
                    else if upper <= *upper_bound {upper}
                    else { ExtendedNum::PosInf };

                if lower == ExtendedNum::NegInf && upper == ExtendedNum::PosInf {
                    BoundedIntervalDomain::Top
                } else if lower > upper {
                    BoundedIntervalDomain::Bottom
                } else {
                    BoundedIntervalDomain::Range(lower, upper)
                }
            },
            _ => panic!("Missing configuration for BoundedInterval Domain")
        }
    }
}


impl PartialOrd for BoundedIntervalDomain{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (BoundedIntervalDomain::Bottom, BoundedIntervalDomain::Bottom) | (BoundedIntervalDomain::Top, BoundedIntervalDomain::Top) => Some(Ordering::Equal),
            (BoundedIntervalDomain::Top, _) | (_, BoundedIntervalDomain::Bottom) => Some(Ordering::Greater),
            (BoundedIntervalDomain::Bottom, _) | (_, BoundedIntervalDomain::Top) => Some(Ordering::Less),
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) => {
                if a==c && b==d { Some(Ordering::Equal) }
                else if a<=c && b>=d { Some(Ordering::Greater)}
                else if a>=c && b<=d { Some(Ordering::Less)}
                else { None }
            },
        }
    }
}

impl From<Num> for BoundedIntervalDomain{
    fn from(value: Num) -> Self {
        BoundedIntervalDomain::new(ExtendedNum::Num(value),ExtendedNum::Num(value))
    }
}
impl From<Interval> for BoundedIntervalDomain {
    fn from(value: Interval) -> Self {
        match value {
            Interval::OpenLeft(max) => Self::new(ExtendedNum::NegInf, ExtendedNum::Num(max)),
            Interval::OpenRight(min) => Self::new(ExtendedNum::Num(min), ExtendedNum::PosInf),
            Interval::Closed(min, max) => Self::new(ExtendedNum::Num(min), ExtendedNum::Num(max)),
        }
    }
}
impl FromStr for BoundedIntervalDomain{
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

impl AbstractDomain for BoundedIntervalDomain{
    fn set_config(config_string: Option<String>) -> Result<(), String>{
        match config_string {
            Some(conf) => {
                let mut chars = conf.chars();
                match chars.next() {
                    Some('[') => (),
                    _ => return Err(format!("Expected \"[l,u]\", found {conf}")),
                }
                let lower: String = chars.take_while_ref(|c|c!=&',').collect();
                
                match chars.next() {
                    Some(',') => (),
                    _ => return Err(format!("Expected \"[l,u]\", found {conf}")),
                }
                match chars.next_back() {
                    Some(']') => (),
                    _ => return Err(format!("Expected \"[l,u]\", found {conf}")),
                }
        
                let upper: String = chars.collect();
        
                let lower = lower.parse()?;
                let upper = upper.parse()?;
                BI_LOWER.get_or_init(|| lower);
                BI_UPPER.get_or_init(|| upper);
            },
            None => {
                BI_LOWER.get_or_init(|| ExtendedNum::NegInf);
                BI_UPPER.get_or_init(|| ExtendedNum::PosInf);
            },
        };
        return Ok(());
    }


    fn bottom() -> Self { BoundedIntervalDomain::Bottom }
    fn top() -> Self { BoundedIntervalDomain::Top }

    fn lub(&self, other: &Self) -> Self {
        match (self,other) {
            (BoundedIntervalDomain::Top,_) | (_, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,
            (BoundedIntervalDomain::Bottom, i) | (i, BoundedIntervalDomain::Bottom) => i.clone(),
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) =>{
                let lower = min(a,c).clone();
                let upper = max(b,d).clone();
                BoundedIntervalDomain::new(lower, upper)
            }
        }
    }

    fn glb(&self, other: &Self) -> Self {
        match (self,other) {
            (BoundedIntervalDomain::Bottom,_) | (_, BoundedIntervalDomain::Bottom) => BoundedIntervalDomain::Bottom,
            (BoundedIntervalDomain::Top, i) | (i, BoundedIntervalDomain::Top) => i.clone(),
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) =>{
                let lower = max(a,c).clone();
                let upper = min(b,d).clone();
                BoundedIntervalDomain::new(lower, upper)
            }
        }
    }



    fn widening(self, other:Self) -> Self {
        match(self, other){
            (BoundedIntervalDomain::Bottom, x) | (x, BoundedIntervalDomain::Bottom) => x,
            (BoundedIntervalDomain::Top, _) | (_, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,

            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) =>{
                let l = if a<=c { a } else { ExtendedNum::NegInf };
                let u = if b>=d { b } else { ExtendedNum::PosInf };
                BoundedIntervalDomain::new(l,u)
            }
        }
    }

    fn narrowing(self, other:Self) -> Self {
        match(self, other){
            (BoundedIntervalDomain::Bottom, _) | (_, BoundedIntervalDomain::Bottom) => Self::Bottom,
            (BoundedIntervalDomain::Top, x) | (x, BoundedIntervalDomain::Top) => x,

            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) =>{
                let l = if a == ExtendedNum::NegInf { c } else { a };
                let u = if b == ExtendedNum::PosInf { d } else { b };
                BoundedIntervalDomain::new(l,u)
            }
        }
    }

    

    
}

impl Display for BoundedIntervalDomain{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundedIntervalDomain::Top => write!(f, "⊤"),
            BoundedIntervalDomain::Bottom => write!(f, "⊥"),
            BoundedIntervalDomain::Range(n1, n2) => write!(f, "[{n1},{n2}]"),
        }
    }
}

impl Add for BoundedIntervalDomain{
    type Output = BoundedIntervalDomain;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedIntervalDomain::Bottom,_) | (_, BoundedIntervalDomain::Bottom) => BoundedIntervalDomain::Bottom,
            (BoundedIntervalDomain::Top, _) | (_, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) => {
                let lower = a.clone()+c.clone();
                let upper = b.clone()+d.clone();
                BoundedIntervalDomain::new(lower, upper)
            }
        }
    }
}
impl Sub for BoundedIntervalDomain{
    type Output = BoundedIntervalDomain;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedIntervalDomain::Bottom,_) | (_, BoundedIntervalDomain::Bottom) => BoundedIntervalDomain::Bottom,
            (BoundedIntervalDomain::Top, _) | (_, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) => {
                let lower = a.clone()-d.clone();
                let upper = b.clone()-c.clone();
                BoundedIntervalDomain::new(lower, upper)
            }
        }
    }
}
impl Mul for BoundedIntervalDomain{
    type Output = BoundedIntervalDomain;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (BoundedIntervalDomain::Bottom,_) | (_, BoundedIntervalDomain::Bottom) => BoundedIntervalDomain::Bottom,
            (BoundedIntervalDomain::Top, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,
            (BoundedIntervalDomain::Top, BoundedIntervalDomain::Range(a, b)) | (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedIntervalDomain::new(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    BoundedIntervalDomain::Top
                }
            },
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Range(c, d)) => {
                let points = [a*c, a*d, b*c, b*d];
                let lower = points.iter().min().unwrap().clone();               
                let upper = points.iter().max().unwrap().clone();         
                BoundedIntervalDomain::new(lower, upper)
            }
        }
    }
    
}

impl Div for BoundedIntervalDomain{
    type Output = BoundedIntervalDomain;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BoundedIntervalDomain::Bottom,_) | (_, BoundedIntervalDomain::Bottom) => BoundedIntervalDomain::Bottom,
            (BoundedIntervalDomain::Top, BoundedIntervalDomain::Top) => BoundedIntervalDomain::Top,
            (BoundedIntervalDomain::Top, BoundedIntervalDomain::Range(a, b))  =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedIntervalDomain::Bottom
                } else {
                    BoundedIntervalDomain::Top
                }
            },
            (BoundedIntervalDomain::Range(a, b), BoundedIntervalDomain::Top) =>{
                if a == ExtendedNum::Num(0) && b == ExtendedNum::Num(0){
                    BoundedIntervalDomain::new(ExtendedNum::Num(0), ExtendedNum::Num(0))
                } else {
                    BoundedIntervalDomain::Top
                }
            },
            (n1@BoundedIntervalDomain::Range(a, b), n2@BoundedIntervalDomain::Range(c, d)) => {
                if ExtendedNum::Num(1)<=c {
                    BoundedIntervalDomain::new(min(a/c, a/d), max(b/c, b/d))
                }else if d <= ExtendedNum::Num(-1) {
                    BoundedIntervalDomain::new(min(b/c, b/d), max(a/c, a/d))
                } else {
                    let d1 = n1 / n2.glb(&BoundedIntervalDomain::new(ExtendedNum::Num(1), ExtendedNum::PosInf));
                    let d2 = n1 / n2.glb(&BoundedIntervalDomain::new(ExtendedNum::NegInf, ExtendedNum::Num(-1)));
                    d1.lub(&d2)
                }
            }
        }
    }
}