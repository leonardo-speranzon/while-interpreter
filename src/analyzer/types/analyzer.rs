use std::collections::HashMap;

use crate::{analyzer::abstract_translator::abstract_program, types::{ast::{Aexpr, Bexpr, Statement}, lit_interval::LitInterval}};

use super::{domain::AbstractDomain, program::{Label, Program}, state::AbstractState};

pub trait StaticAnalyzer<B: AbstractDomain, D: AbstractState<B>> {
    fn eval_aexpr(a: &Aexpr<B>, s: D)-> (B, D);
    // fn refine_aexpr(a: &Aexpr<D>,s:B, dom: &D) -> B;
    fn eval_bexpr(b: &Bexpr<B>, s: D)-> D;

    fn init(ast: Statement<LitInterval>) -> Program<B> {
        let p = Program::from(ast);
        // println!("\nProgram:{:?}\n\n", p);
        let abs_prog = abstract_program(p);
        // println!("\nAbstract Program: {:?}\n\n", abs_prog);
        abs_prog
    }
    fn analyze(p: Program<B>, init_state: D, iteration_strategy: IterationStrategy) -> HashMap<Label, D>;
    
} 
#[derive(Debug)]
pub enum IterationStrategy{
    Simple,
    Widening,
    WideningAndNarrowing,
}