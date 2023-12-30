
pub mod domains{
    pub mod sign_domain;
    pub mod interval_domain;
}
pub mod my_analyzer;
mod abs_ast;
pub mod program;
pub mod printers;

use std::{collections::HashMap, fmt::Display};
use std::fmt::Debug;
use crate::{interpreter::types::State, types::ast::{Statement, Aexpr, Bexpr, Operator, Num}};

use self::{abs_ast::abstract_program, program::{Program, Label}};

#[derive(Debug, PartialEq, Clone)]
pub struct AbstractState<D>(Option<State<D>>);

impl<D: AbstractDomain> AbstractState<D> {
    pub fn bottom() -> Self{
        AbstractState(None)
    }
    pub fn top() -> Self{
        AbstractState(Some(State::new()))
    }    
    pub fn lub(self, other: Self) -> Self { 
        // print!("LUB {:?} - {:?} -> ",self,other);
        let new_s=  match (self, other) {
            (AbstractState(Some(mut s1)),AbstractState(Some(s2))) => { 
                s1 = s1.into_iter().filter_map(|(k,v)|{
                    match s2.get(&k) {
                        Some(d) => Some((k, v.lub(d))),
                        None => None,
                    }
                }).collect(); 
                AbstractState(Some(s1))
            },
            (AbstractState(Some(s)), _) | (_, AbstractState(Some(s))) => AbstractState(Some(s)),
            (_, _) => AbstractState(None)
        };
        // println!("{:?}", new_s );
        new_s
    } 
    pub fn glb(self, other: &Self) -> Self { 
        match (self, other) {
            (AbstractState(Some(mut s1)),AbstractState(Some(s2))) => {
                for (k,v) in s2.into_iter() {
                    let new_v = match s1.get(k) {
                        Some(d) => v.glb(d),
                        None => v.clone(),
                    };
                    if new_v == D::bottom(){
                        return AbstractState(None)
                    }
                    s1.insert(k.to_string(), new_v);
                }
                AbstractState(Some(s1))
            },
            (_,_) => AbstractState(None)
        }
    }
    fn set(&mut self, k: String, v: D) {
        match self {
            AbstractState(Some(s)) => {
                if v == D::bottom() {
                    self.0 = None
                }else {
                    s.insert(k, v);
                }
            },
            AbstractState(None) => (),
        }
    }
}

impl<D: AbstractDomain> PartialOrd for AbstractState<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

pub trait AbstractDomain : Debug + Display + PartialOrd + Clone + Sized + From<Num> {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self;
    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self);
    // fn widening();
    // fn narrowing();
}

pub trait StaticAnalyzer<D: AbstractDomain> {
    fn eval_aexpr(a: &Aexpr<D>, s: &AbstractState<D>)-> D;
    fn refine_aexpr(a: &Aexpr<D>,s:AbstractState<D>, dom: &D) -> AbstractState<D>;
    fn eval_bexpr(b: &Bexpr<D>, s: AbstractState<D>)-> AbstractState<D>;

    fn init(ast: Statement<Num>) -> Program<D> {
        let p = Program::from(ast);
        println!("\n{:?}\n\n", p);
        abstract_program(p)
    }
    fn analyze(p: Program<D>, init_state: AbstractState<D>) -> HashMap<Label, AbstractState<D>>;
    
}

