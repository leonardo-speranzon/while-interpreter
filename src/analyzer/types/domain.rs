use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Debug};
use std::str::FromStr;
use crate::types::ast::{Num, Operator};
use crate::types::lit_interval::LitInterval;

pub enum Interval {
    OpenLeft (Num),
    OpenRight (Num),
    Closed (Num,Num),
}
impl From<LitInterval> for Interval {
    fn from(value: LitInterval) -> Self {
        Self::Closed(value.0, value.1)
    }
}


pub trait AbstractDomain : Debug + Display + Copy + Sized           // Utils
                           + From<Num> + From<Interval> + FromStr   // Conversions
                           + PartialOrd                             // Complete lattice
                           + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self>  {

    fn set_config(config_string: Option<String>) -> Result<(), String> {
        match config_string {
            None => Err(String::from("This domain does not support configuration")),
            Some(_) => Ok(())
        }
    }
    
    // complete lattice functions
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(self, other: Self) -> Self;
    fn glb(self, other: Self) -> Self;

    // Alias function for arithmetic operators
    fn abstract_operator(op: &Operator, lhs: Self, rhs: Self) -> Self {
        match op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }

    // Abstract backward operators used for advanced abstract tests
    fn backward_abstract_operator(op: &Operator, lhs: Self, rhs: Self, res: Self) -> (Self, Self){
        match op {
            Operator::Add => (
                lhs.glb(res - rhs),
                rhs.glb(res - lhs),
            ),
            Operator::Sub => (
                lhs.glb(res + rhs),
                rhs.glb(lhs - res),
            ),
            Operator::Mul => (
                lhs.glb(res / rhs),
                rhs.glb(res / lhs),
            ),
            Operator::Div => {
                let s =  res + Interval::Closed(-1, 1).into();
                (
                    lhs.glb(s * rhs),
                    rhs.glb((lhs / s).lub(Interval::Closed(0, 0).into()))
                )
            }
        }
    }

    fn widening(self, other:Self) -> Self {
        self.lub(other) //Trivial widening (possible infinite ascending chain)
    }
    fn narrowing(self, _other:Self) -> Self {
        self //Trivial narrowing (no narrowing)
    }

}

