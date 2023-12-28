use crate::types::ast::Operator;

use super::AbstractDomain;

#[derive(PartialEq,Clone)]
enum Sign{
    Top,
    Bottom,
    Positive,
    Zero,
    Negative,
}

impl PartialOrd for Sign{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl AbstractDomain for Sign{
    fn bottom() -> Self {
        todo!()
    }

    fn top() -> Self {
        todo!()
    }

    fn lub(&self, other: &Self) -> Self {
        todo!()
    }

    fn glb(&self, other: &Self) -> Self {
        todo!()
    }

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self {
        todo!()
    }

    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self) {
        todo!()
    }
}
