use std::{fmt::Display, cmp::{min, max, Ordering}, ops::{Add, Div, Mul, Sub}};
use crate::{types::ast::{Num, Operator}, analyzer::AbstractDomain};


#[derive(Debug,PartialEq,Clone)]
pub enum Interval{
    Pair(ExtendedNum,ExtendedNum),
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
            Interval::Pair(lower,upper)
        }
    }
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum ExtendedNum{
    PosInf,
    NegInf,
    Num(Num)
}
impl PartialOrd for ExtendedNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ExtendedNum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (ExtendedNum::PosInf, ExtendedNum::PosInf)
            | (ExtendedNum::NegInf, ExtendedNum::NegInf) => Ordering::Equal,
            (ExtendedNum::PosInf, _ )
            | (_, ExtendedNum::NegInf ) => Ordering::Greater,
            (ExtendedNum::NegInf, _ )
            | (_, ExtendedNum::PosInf ) => Ordering::Less,
            (ExtendedNum::Num(n1),ExtendedNum::Num(n2)) =>
                n1.cmp(n2)
        }
    }
}
impl Add for ExtendedNum{
    type Output = ExtendedNum;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (ExtendedNum::PosInf, ExtendedNum::NegInf)
            | (ExtendedNum::NegInf, ExtendedNum::PosInf) => panic!("Impossible addiction inf-inf"),
            (ExtendedNum::PosInf, _ )
            | (_, ExtendedNum::PosInf ) => ExtendedNum::PosInf,
            (ExtendedNum::NegInf, _ )
            | (_, ExtendedNum::NegInf ) => ExtendedNum::NegInf,
            (ExtendedNum::Num(n1), ExtendedNum::Num(n2)) => ExtendedNum::Num(n1 + n2),
        }
    }
}
impl Sub for ExtendedNum{
    type Output = ExtendedNum;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (ExtendedNum::PosInf, ExtendedNum::PosInf)
            | (ExtendedNum::NegInf, ExtendedNum::NegInf) => panic!("Impossible addiction inf-inf"),
            (ExtendedNum::PosInf, _ ) | (_, ExtendedNum::NegInf ) => ExtendedNum::PosInf,
            (ExtendedNum::NegInf, _ ) | (_, ExtendedNum::PosInf ) => ExtendedNum::NegInf,
            (ExtendedNum::Num(n1), ExtendedNum::Num(n2)) => ExtendedNum::Num(n1 - n2),
        }
    }
}
impl Mul for ExtendedNum{
    type Output = ExtendedNum;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs){
            (ExtendedNum::PosInf, ExtendedNum::PosInf)
            | (ExtendedNum::NegInf, ExtendedNum::NegInf) => ExtendedNum::PosInf,
            (ExtendedNum::PosInf, ExtendedNum::NegInf)
            | (ExtendedNum::NegInf, ExtendedNum::PosInf) => ExtendedNum::NegInf,
            (ExtendedNum::PosInf, ExtendedNum::Num(n))
            | (ExtendedNum::Num(n), ExtendedNum::PosInf) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::NegInf,
                Ordering::Equal => ExtendedNum::Num(0),
                Ordering::Greater => ExtendedNum::PosInf,
            }
            (ExtendedNum::NegInf, ExtendedNum::Num(n))
            | (ExtendedNum::Num(n), ExtendedNum::NegInf) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::PosInf,
                Ordering::Equal => ExtendedNum::Num(0),
                Ordering::Greater => ExtendedNum::NegInf,
            }
            (ExtendedNum::Num(n1), ExtendedNum::Num(n2)) => ExtendedNum::Num(n1 * n2),
        }
    }
    
}

impl Div for ExtendedNum{
    type Output = ExtendedNum;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ExtendedNum::PosInf, ExtendedNum::PosInf) => panic!(),
            (ExtendedNum::PosInf, ExtendedNum::NegInf) => panic!(),
            (ExtendedNum::NegInf, ExtendedNum::PosInf) => panic!(),
            (ExtendedNum::NegInf, ExtendedNum::NegInf) => panic!(),
            (ExtendedNum::PosInf, ExtendedNum::Num(n)) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::NegInf,
                Ordering::Equal => ExtendedNum::PosInf,
                Ordering::Greater => ExtendedNum::PosInf,
            },
            (ExtendedNum::NegInf, ExtendedNum::Num(n)) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::PosInf,
                Ordering::Equal => ExtendedNum::NegInf,
                Ordering::Greater => ExtendedNum::NegInf,
            },
            (ExtendedNum::Num(n), ExtendedNum::PosInf) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::NegInf,
                Ordering::Equal => ExtendedNum::Num(0),
                Ordering::Greater => ExtendedNum::PosInf,
            },
            (ExtendedNum::Num(n), ExtendedNum::NegInf) => match n.cmp(&0) {
                Ordering::Less => ExtendedNum::PosInf,
                Ordering::Equal => ExtendedNum::Num(0),
                Ordering::Greater => ExtendedNum::NegInf,
            },
            (ExtendedNum::Num(n), ExtendedNum::Num(0)) => match n.cmp(&0)  {
                Ordering::Less => ExtendedNum::NegInf,
                Ordering::Equal => panic!(),
                Ordering::Greater => ExtendedNum::PosInf,
            },
            (ExtendedNum::Num(n1), ExtendedNum::Num(n2)) => ExtendedNum::Num(n1/n2),
        }
    }
}


impl Display for ExtendedNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtendedNum::PosInf => write!(f, "+∞"),
            ExtendedNum::NegInf => write!(f, "-∞"),
            ExtendedNum::Num(n) => write!(f, "{n}"),
        }
    }
}
impl Display for Interval{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Top => write!(f, "⊤"),
            Interval::Bottom => write!(f, "⊥"),
            Interval::Pair(n1, n2) => write!(f, "[{n1},{n2}]"),
        }
    }
}
impl PartialOrd for Interval{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<Num> for Interval{
    fn from(value: Num) -> Self {
        Interval::Pair(ExtendedNum::Num(value),ExtendedNum::Num(value))
    }
}

impl AbstractDomain for Interval{
    fn bottom() -> Self {
        Interval::Bottom
    }

    fn top() -> Self {
        Interval::Top
    }

    fn lub(&self, other: &Self) -> Self {
        match (self,other) {
            (Interval::Top,_) | (_, Interval::Top) => Interval::Top,
            (Interval::Bottom, i) | (i, Interval::Bottom) => i.clone(),
            (Interval::Pair(a, b), Interval::Pair(c, d)) =>{
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
            (Interval::Pair(a, b), Interval::Pair(c, d)) =>{
                let lower = max(a,c).clone();
                let upper = min(b,d).clone();
                Interval::new(lower, upper)
            }
        }
    }

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self {
        match op {
            Operator::Add => {
                match (lhs,rhs) {
                    (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
                    (Interval::Top, _) | (_, Interval::Top) => Interval::Top,
                    (Interval::Pair(a, b), Interval::Pair(c, d)) => {
                        let lower = a.clone()+c.clone();
                        let upper = b.clone()+d.clone();
                        Interval::new(lower, upper)
                    }
                    
                }
            },
            Operator::Sub => {
                match (lhs,rhs) {
                    (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
                    (Interval::Top, _) | (_, Interval::Top) => Interval::Top,
                    (Interval::Pair(a, b), Interval::Pair(c, d)) => {
                        let lower = a.clone()-d.clone();
                        let upper = b.clone()-c.clone();
                        Interval::new(lower, upper)
                    }
                    
                }
            },
            Operator::Mul => {
                match (lhs,rhs) {
                    (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
                    (Interval::Top, Interval::Top) => Interval::Top,
                    (Interval::Top, Interval::Pair(a, b)) | (Interval::Pair(a, b), Interval::Top) =>{
                        if *a == ExtendedNum::Num(0) && *b == ExtendedNum::Num(0){
                            Interval::Pair(ExtendedNum::Num(0), ExtendedNum::Num(0))
                        } else {
                            Interval::Top
                        }
                    },
                    (Interval::Pair(a, b), Interval::Pair(c, d)) => {
                        let points = [
                            (a.clone())*(c.clone()),
                            (a.clone())*(d.clone()),
                            (b.clone())*(c.clone()),
                            (b.clone())*(d.clone()),
                        ];
                        let lower = points.iter().min().unwrap().clone();               
                        let upper = points.iter().max().unwrap().clone();         
                        Interval::new(lower, upper)
                    }
                    
                }
            },
            Operator::Div => {
                match (lhs,rhs) {
                    (Interval::Bottom,_) | (_, Interval::Bottom) => Interval::Bottom,
                    (Interval::Top, Interval::Top) => Interval::Top,
                    (Interval::Top, Interval::Pair(a, b))  =>{
                        if *a == ExtendedNum::Num(0) && *b == ExtendedNum::Num(0){
                            Interval::Bottom
                        } else {
                            Interval::Top
                        }
                    },
                    (Interval::Pair(a, b), Interval::Top) =>{
                        if *a == ExtendedNum::Num(0) && *b == ExtendedNum::Num(0){
                            Interval::Pair(ExtendedNum::Num(0), ExtendedNum::Num(0))
                        } else {
                            Interval::Top
                        }
                    },                   
                    (n1@Interval::Pair(a, b), n2@Interval::Pair(c, d)) => {
                        if ExtendedNum::Num(1)<=*c {
                            Interval::new(
                                min(a.clone()/c.clone(), a.clone()/d.clone()),
                                max(b.clone()/c.clone(), b.clone()/d.clone())
                            )
                        }else if *d <= ExtendedNum::Num(-1) {
                            Interval::new(
                                min(b.clone()/c.clone(), b.clone()/d.clone()),
                                max(a.clone()/c.clone(), a.clone()/d.clone())
                            )
                        } else {
                            let d1 = Interval::abstract_operator(
                                &Operator::Div, 
                                n1,
                                &n2.glb(&Interval::new(ExtendedNum::Num(1), ExtendedNum::PosInf))
                            );
                            let d2 = Interval::abstract_operator(
                                &Operator::Div, 
                                n1,
                                &n2.glb(&Interval::new(ExtendedNum::NegInf, ExtendedNum::Num(-1)))
                            );
                            d1.lub(&d2)
                        }
                    }//
                }
            }
        }
    }

    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self) {
        todo!()
    }
}