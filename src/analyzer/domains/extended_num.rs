use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}};

use crate::types::ast::Num;

#[derive(Debug,PartialEq,Eq,Clone, Copy)]
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