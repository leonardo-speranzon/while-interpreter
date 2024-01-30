use crate::{analyzer::types::program::Command, types::ast::{Aexpr, Bexpr, Num}};

use super::types::{domain::AbstractDomain, program::Program};


pub fn abstract_program<D:AbstractDomain>(prog: Program<Num>) -> Program<D>{
    let abs_arcs = prog.arcs.into_iter().map(|(l1,cmd,l2)|{
        let abs_cmd = match cmd {
            Command::Assignment(x, a) => Command::Assignment(x, translate_aexpr(a)),
            Command::Test(b) => Command::Test(translate_bexpr(b)),
        };
        (l1,abs_cmd,l2)
    }).collect();
    let mut prog = Program::new(abs_arcs);

    prog.compute_widening_point();

    println!("prog: {:?}", prog);
    prog
}
fn translate_aexpr<D:AbstractDomain>(a: Aexpr<Num>) -> Aexpr<D>{
    match a {
        Aexpr::Num(n) => Aexpr::Num(D::from(n)),
        Aexpr::Var(x) => Aexpr::Var(x),
        Aexpr::PreInc(x) => Aexpr::PreInc(x),
        Aexpr::PreDec(x) => Aexpr::PreDec(x),
        Aexpr::PostInc(x) => Aexpr::PostInc(x),
        Aexpr::PostDec(x) => Aexpr::PostDec(x),
        Aexpr::BinOp(op, a1, a2) => 
            Aexpr::BinOp(op, Box::new(translate_aexpr(*a1)), Box::new(translate_aexpr(*a2))),
    }
}
fn translate_bexpr<D:AbstractDomain>(b: Bexpr<Num>) -> Bexpr<D>{
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
