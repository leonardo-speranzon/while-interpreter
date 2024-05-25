use crate::analyzer::types::program::{Command, Label};
use crate::analyzer::types::{program::Program, state::AbstractState};
use crate::analyzer::types::domain::AbstractDomain;
use crate::analyzer::types::analyzer::{IterationStrategy, StaticAnalyzer};
use crate::types::ast::{PostOp, PreOp};
use std::{collections::HashMap, marker::PhantomData};

use crate::{types::ast::{Aexpr, Bexpr}, analyzer::printers::map_to_str};
use crate::analyzer::advanced_tests;
pub struct GenericAnalyzer<D, B> {    
   domain: PhantomData<D>,
   abs_state: PhantomData<B>,
}

impl<D: AbstractDomain, B: AbstractState<D>> StaticAnalyzer<D,B> for GenericAnalyzer<D,B>{
    
    fn eval_aexpr(a: &Aexpr<D>, mut s: B)-> (D, B) {
        match a {
            Aexpr::Lit(n) => (*n, s),
            Aexpr::Var(x) => (s.get(x), s),
            Aexpr::BinOp(op, a1, a2 ) => {
                let (n1, s1) = Self::eval_aexpr(a1, s);
                let (n2, s2) = Self::eval_aexpr(a2, s1);
                let d = D::abstract_operator(op, n1, n2);
                (d, s2)
            }
            Aexpr::PreOp(PreOp::Inc, x) => {
                let d = s.get(x) + D::from(1);
                s.set(x.to_string(), d);
                (d, s)
            }
            Aexpr::PreOp(PreOp::Dec, x) => {
                let d = s.get(x) - D::from(1);
                s.set(x.to_string(), d);
                (d, s)
            },
            Aexpr::PostOp(PostOp::Inc, x) => {
                let d = s.get(x);
                s.set(x.to_string(), d + D::from(1));
                (d, s)
            },
            Aexpr::PostOp(PostOp::Dec, x) =>{
                let d = s.get(x);
                s.set(x.to_string(), d - D::from(1));
                (d, s)
            },
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B {
        advanced_tests::eval_bexpr(b, s)
    }

    fn analyze(prog: Program<D>, init_state: B, iteration_strategy: IterationStrategy) -> HashMap<Label, B> {
        let mut all_state: HashMap<Label, B> = HashMap::new();


        for i in 0..prog.labels_num {
            all_state.insert(i, AbstractState::bottom());
        }


        let mut iteration_num= 1;
        
        let print_iters_enabled = std::env::var("print-iterations").is_ok_and(|s|s=="true");
        if print_iters_enabled {
            println!("╔════════════╗");
            println!("║ Iterations ║");
            println!("╚════════════╝"); 
            println!("\nINITIAL STATES:\n{}\n", map_to_str(&all_state));
        }

        if let IterationStrategy::Simple = iteration_strategy {            
            let mut new_all_state = Self::make_iteration(&prog, &init_state, all_state.clone(), StepType::NormalStep);
            while new_all_state != all_state {
                all_state = new_all_state;
                if print_iters_enabled {
                    println!("ITERATION {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                }
                new_all_state = Self::make_iteration(&prog,&init_state,  all_state.clone(), StepType::NormalStep); 
            }
        }else {
            let mut new_all_state = Self::make_iteration(&prog,&init_state,  all_state.clone(), StepType::WideningStep);
            while new_all_state != all_state {
                all_state = new_all_state;
                if print_iters_enabled {
                    println!("ITERATION (∇) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                }
                new_all_state = Self::make_iteration(&prog,&init_state,  all_state.clone(), StepType::WideningStep); 
            }

            if let IterationStrategy::WideningAndNarrowing = iteration_strategy {      
                let mut new_all_state = Self::make_iteration(&prog,&init_state,  all_state.clone(), StepType::NarrowingStep);                
                while new_all_state != all_state {
                    all_state = new_all_state;
                    if print_iters_enabled {
                        println!("ITERATION (Δ) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                    }
                    new_all_state = Self::make_iteration(&prog, &init_state, all_state.clone(), StepType::NarrowingStep); 
                }
            }   
        }
        all_state
    }
}



enum StepType {
    NormalStep,
    WideningStep,
    NarrowingStep
}

impl<D: AbstractDomain, B: AbstractState<D>> GenericAnalyzer<D,B>{
    fn make_iteration(prog: &Program<D>, init_state: &B, states: HashMap<Label, B>, step_type: StepType) -> HashMap<Label, B>{
        let mut all_states: HashMap<Label, B> = HashMap::new();
        for i in 0..=(prog.labels_num-1) {
            let arcs = prog.get_entering_arcs(i);
            let mut new_state = if i == prog.entry{
                init_state.clone()
            }else{
                B::bottom()
            };
            for (l,cmd,_) in arcs {
                match  states.get(l) {
                    Some(s) => new_state = new_state.lub(&Self::apply_cmd(cmd, s)),
                    None => panic!("Missing AbsState for label {l}"),
                };
            }

            if prog.widening_points.contains(&i) {
                let old_state = states
                    .get(&i)
                    .expect(&format!("Missing AbsState for label {i}"))
                    .clone();
                new_state = match step_type {
                    StepType::NormalStep => new_state,
                    StepType::WideningStep => old_state.widening(new_state),
                    StepType::NarrowingStep => old_state.narrowing(new_state),
                }
            }

            all_states.insert(i, new_state);
        };
        all_states
    }


    fn apply_cmd(cmd: &Command<D>, old_state: &B) -> B{
        let mut state = old_state.clone();
        match cmd {
            Command::Assignment(x, a) => {
                let (aexpr_dom, mut s2) = Self::eval_aexpr(a, state);
                s2.set(x.to_string(), aexpr_dom);
                state = s2
            },
            Command::Test(b) => {
                state = Self::eval_bexpr(b, state);
            },
        }
        state
    }

}

