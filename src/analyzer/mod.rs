
pub mod domains{
    pub mod sign_domain;
    pub mod interval_domain;
    pub mod extended_num;
}
pub mod my_analyzer;
mod abs_ast;
pub mod program;
pub mod printers;

use std::ops::{Add, Div, Mul, Sub};
use std::{collections::HashMap, fmt::Display};
use std::fmt::Debug;
use crate::{interpreter::types::State, types::ast::{Statement, Aexpr, Bexpr, Operator, Num}};

use self::{abs_ast::abstract_program, program::{Program, Label}};

pub trait AbstractState<D>: Debug + Display + PartialEq + Clone {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(self, other: Self) -> Self;
    fn glb(self, other: &Self) -> Self;
    fn get(&self, k: &str) -> D;
    fn set(&mut self, k: String, v: D);
}

#[derive(Debug, PartialEq, Clone)]
pub struct HashMapState<D>(Option<State<D>>);

impl<D: AbstractDomain> AbstractState<D> for HashMapState<D>  {
    fn bottom() -> Self{
        HashMapState(None)
    }
    fn top() -> Self{
        HashMapState(Some(State::new()))
    }    
    fn lub(self, other: Self) -> Self { 
        // print!("LUB {:?} - {:?} -> ",self,other);
        let new_s=  match (self, other) {
            (HashMapState(Some(mut s1)),HashMapState(Some(s2))) => { 
                s1 = s1.into_iter().filter_map(|(k,v)|{
                    match s2.get(&k) {
                        Some(d) => Some((k, v.lub(d))),
                        None => None,
                    }
                }).collect(); 
                HashMapState(Some(s1))
            },
            (HashMapState(Some(s)), _) | (_, HashMapState(Some(s))) => HashMapState(Some(s)),
            (_, _) => HashMapState(None)
        };
        // println!("{:?}", new_s );
        new_s
    } 
    fn glb(self, other: &Self) -> Self { 
        match (self, other) {
            (HashMapState(Some(mut s1)),HashMapState(Some(s2))) => {
                for (k,v) in s2.into_iter() {
                    let new_v = match s1.get(k) {
                        Some(d) => v.glb(d),
                        None => v.clone(),
                    };
                    if new_v == D::bottom(){
                        return HashMapState(None)
                    }
                    s1.insert(k.to_string(), new_v);
                }
                HashMapState(Some(s1))
            },
            (_,_) => HashMapState(None)
        }
    }
    fn get(&self, k: &str) -> D {
        match self {
            HashMapState(Some(s)) => {
                match s.get(k) {
                    Some(n) => n.clone(),
                    None => D::top(),
                }
            },
            HashMapState(None) =>  D::bottom(),
        }
    }
    fn set(&mut self, k: String, v: D) {
        match self {
            HashMapState(Some(s)) => {
                if v == D::bottom() {
                    self.0 = None
                }else {
                    s.insert(k, v);
                }
            },
            HashMapState(None) => (),
        }
    }
}

impl<D: AbstractDomain> PartialOrd for HashMapState<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

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
            Operator::Mul => lhs.clone() * rhs.clone(),
            Operator::Div => lhs.clone() / rhs.clone(),
        }
    }
    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self);
    // fn widening();
    // fn narrowing();

    fn gte(lb: &Self) -> Self;
    fn lte(ub: &Self) -> Self;
}




pub trait StaticAnalyzer<D: AbstractDomain, B: AbstractState<D> = HashMapState<D>> {
    fn eval_aexpr(a: &Aexpr<D>, s: B)-> (D, B);
    // fn refine_aexpr(a: &Aexpr<D>,s:B, dom: &D) -> B;
    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B;

    fn init(ast: Statement<Num>) -> Program<D> {
        let p = Program::from(ast);
        println!("\n{:?}\n\n", p);
        abstract_program(p)
    }
    fn analyze(p: Program<D>, init_state: B) -> HashMap<Label, B>;
    
}

