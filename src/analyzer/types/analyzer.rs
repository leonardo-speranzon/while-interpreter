use std::collections::HashMap;

use crate::{analyzer::abstract_translator::abstract_program, types::ast::{Aexpr, Bexpr, Num, Statement}};

use super::{domain::AbstractDomain, program::{Label, Program}, state::AbstractState};

pub trait StaticAnalyzer<D: AbstractDomain, B: AbstractState<D>> {
    fn eval_aexpr(a: &Aexpr<D>, s: B)-> (D, B);
    // fn refine_aexpr(a: &Aexpr<D>,s:B, dom: &D) -> B;
    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B;

    fn init(ast: Statement<Num>) -> Program<D> {
        let p = Program::from(ast);
        // println!("\nProgram:{:?}\n\n", p);
        let abs_prog = abstract_program(p);
        // println!("\nAbstract Program: {:?}\n\n", abs_prog);
        abs_prog
    }
    fn analyze(p: Program<D>, init_state: B, iteration_strategy: IterationStrategy) -> HashMap<Label, B>;
    
} 
#[derive(Debug)]
pub enum IterationStrategy{
    Simple,
    Widening,
    WideningAndNarrowing,
}