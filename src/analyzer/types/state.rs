use std::fmt::{Display, Debug};

pub trait AbstractState<D>: Debug + Display + PartialEq + Clone {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(self, other: Self) -> Self;
    fn glb(self, other: &Self) -> Self;
    fn get(&self, k: &str) -> D;
    fn set(&mut self, k: String, v: D);
    fn widening(self, other:Self) -> Self;
    fn narrowing(self, other: Self) -> Self;
}
