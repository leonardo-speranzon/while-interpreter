use std::fmt::{Display, Debug};

pub trait AbstractState<B>: Debug + Display + PartialOrd + Clone {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(self, other: &Self) -> Self;
    fn glb(self, other: &Self) -> Self;
    fn get(&self, k: &str) -> B;
    fn set(&mut self, k: String, v: B);
    fn widening(self, other:Self) -> Self;
    fn narrowing(self, other: Self) -> Self;
}
