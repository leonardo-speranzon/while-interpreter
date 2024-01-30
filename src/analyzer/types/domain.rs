use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Debug};
use crate::types::ast::{Num, Operator};

pub trait AbstractDomain : Debug + Display + PartialOrd + Clone + Sized + From<Num>
                           + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self>  {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self {
        match op {
            Operator::Add => lhs.clone() + rhs.clone(),
            Operator::Sub => lhs.clone() - rhs.clone(),
            //  {
            //     let res = lhs.clone() - rhs.clone();
            //     println!("({lhs}) - ({rhs}) -> ({res})");
            //     res
            // },
            Operator::Mul => lhs.clone() * rhs.clone(),
            //  {
            //     let res = lhs.clone() * rhs.clone();
            //     println!("{lhs} * {rhs} = {res}");
            //     res
            // },
            Operator::Div => lhs.clone() / rhs.clone(),
        }
    }
    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self);

    fn widening(self, other:Self) -> Self {
        self.lub(&other) //Trivial widening
    }
    fn narrowing(self, other:Self) -> Self {
        self //Trivial narrowing
    }

    fn all_gte(lb: &Self) -> Self;
    fn all_lte(ub: &Self) -> Self;

    fn all_gt(lb: &Self) -> Self {Self::all_gte(&(lb.clone() + Self::from(1)))}
    fn all_lt(ub: &Self) -> Self {Self::all_lte(&(ub.clone() - Self::from(1)))}
}

