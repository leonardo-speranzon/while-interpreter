use crate::{analyzer::types::program::Command, types::{ast::{Aexpr, Bexpr}, lit_interval::LitInterval}};

use super::types::{domain::{AbstractDomain, Interval}, program::Program};


pub fn abstract_program<D:AbstractDomain>(prog: Program<LitInterval>) -> Program<D>{
    let abs_arcs = prog.arcs.into_iter().map(|(l1,cmd,l2)|{
        let abs_cmd = match cmd {
            Command::Assignment(x, a) => Command::Assignment(x, translate_aexpr(a)),
            Command::Test(b) => Command::Test(translate_bexpr(b)),
        };
        (l1,abs_cmd,l2)
    }).collect();
    Program::new(abs_arcs,prog.widening_points)
}
fn translate_aexpr<D:AbstractDomain>(a: Aexpr<LitInterval>) -> Aexpr<D>{
    match a {
        Aexpr::Lit(n) => Aexpr::Lit(D::from(Interval::from(n))),
        Aexpr::Var(x) => Aexpr::Var(x),
        Aexpr::PreOp(op, x) => Aexpr::PreOp(op, x),
        Aexpr::PostOp(op, x) => Aexpr::PostOp(op, x),
        Aexpr::BinOp(op, a1, a2) => 
            Aexpr::BinOp(op, Box::new(translate_aexpr(*a1)), Box::new(translate_aexpr(*a2))),
    }
}
fn translate_bexpr<D:AbstractDomain>(b: Bexpr<LitInterval>) -> Bexpr<D>{
    match b {
        Bexpr::True => Bexpr::True,
        Bexpr::False => Bexpr::False,
        Bexpr::Equal(a1, a2) => 
            Bexpr::Equal(Box::new(translate_aexpr(*a1)), Box::new(translate_aexpr(*a2))),
        Bexpr::LessEq(a1, a2) => 
            Bexpr::LessEq(Box::new(translate_aexpr(*a1)), Box::new(translate_aexpr(*a2))),
        Bexpr::Not(b) => Bexpr::Not(Box::new(translate_bexpr(*b))),
        Bexpr::And(b1, b2) => 
            Bexpr::And(Box::new(translate_bexpr(*b1)), Box::new(translate_bexpr(*b2))),
    }
}
