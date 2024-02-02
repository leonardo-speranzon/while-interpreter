use crate::analyzer::types::program::{Arc, Command, Label};
use crate::analyzer::types::{program::Program, state::AbstractState};
use crate::analyzer::types::domain::AbstractDomain;
use crate::analyzer::types::analyzer::{IterationStrategy, StaticAnalyzer};
use std::{collections::HashMap, marker::PhantomData};

use crate::{types::ast::{Aexpr, Bexpr}, analyzer::printers::map_to_str};
use crate::analyzer::tests as tests;
pub struct GenericAnalyzer<D, B> {    
   domain: PhantomData<D>,
   abs_state: PhantomData<B>,
}

impl<D: AbstractDomain, B: AbstractState<D>> StaticAnalyzer<D,B> for GenericAnalyzer<D,B>{
    
    fn eval_aexpr(a: &Aexpr<D>, mut s: B)-> (D, B) {
        match a {
            Aexpr::Num(n) => (n.clone(), s),
            Aexpr::Var(x) => (s.get(x), s),
            Aexpr::BinOp(op, a1, a2 ) => {
                let (n1, s1) = Self::eval_aexpr(a1, s);
                let (n2, s2) = Self::eval_aexpr(a2, s1);
                let d = D::abstract_operator(op, &n1, &n2);
                (d, s2)
            }
            Aexpr::PreInc(x) => {
                let d = s.get(x) + D::from(1);
                s.set(x.to_string(), d.clone());
                (d, s)
            }
            Aexpr::PreDec(x) => {
                let d = s.get(x) - D::from(1);
                s.set(x.to_string(), d.clone());
                (d, s)
            },
            Aexpr::PostInc(x) => {
                let d = s.get(x);
                s.set(x.to_string(), d.clone() + D::from(1));
                (d, s)
            },
            Aexpr::PostDec(x) =>{
                let d = s.get(x);
                s.set(x.to_string(), d.clone() - D::from(1));
                (d, s)
            },
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                match (*a1.clone(),*a2.clone()) {
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) =>
                        tests::test_eq_case_1(s, x, c),
                    (Aexpr::Var(x), Aexpr::Var(y)) => 
                        tests::test_eq_case_2(s, x, y, D::from(0)),
                    (a1,a2) => {
                        let (n1, s1) = Self::eval_aexpr(&a1, s);
                        let (n2, s2) = Self::eval_aexpr(&a2, s1);
                        let dom = n1.glb(&n2);
                        if dom == D::bottom() {
                            AbstractState::bottom()
                        }else{
                            s2   
                        }
                    }
                }
            },
            Bexpr::LessEq(a1, a2) => {
                match (*a1.clone(),*a2.clone()) {
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                        tests::test_lte_case_1(s, x, c),
                    (Aexpr::Var(x), Aexpr::Var(y)) =>
                        tests::test_lte_case_2(s, x, y),
                    (_, _) => {
                        let (_, s1) = Self::eval_aexpr(&a1, s);
                        let (_, s2) = Self::eval_aexpr(&a2, s1);
                        s2
                    }
                }                
                
            },
            Bexpr::And(_, _) => todo!(),

            
            Bexpr::Not(b) => {
                match *b.clone() { 
                    Bexpr::True => AbstractState::bottom(),
                    Bexpr::False => s,
                    Bexpr::Equal(a1, a2) => {
                        match (*a1.clone(),*a2.clone()) {
                            (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                                tests::test_neq_case_1(s, x, c),
                            // (Aexpr::Var(x), Aexpr::Var(y)) => 
                            //     tests::test_gt_case_2(s, x, y),
                            (_, _) => {
                                // Since n1 and n2 are over approximations we can't know 
                                let (_, s1) = Self::eval_aexpr(&a1, s);
                                let (_, s2) = Self::eval_aexpr(&a2, s1);
                                s2
                            }
                        } 
                    },
                    Bexpr::LessEq(a1, a2) => {
                        match (*a1.clone(),*a2.clone()) {
                            (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                                tests::test_gt_case_1(s, x, c),
                            (Aexpr::Var(x), Aexpr::Var(y)) => 
                                tests::test_gt_case_2(s, x, y),
                            (_, _) => {
                                let (_, s1) = Self::eval_aexpr(&a1, s);
                                let (_, s2) = Self::eval_aexpr(&a2, s1);
                                s2
                            }
                        } 
                    },
                    Bexpr::Not(b) => Self::eval_bexpr(&b, s),
                    Bexpr::And(_, _) => todo!(),
                }
            },
        }
    }

    fn analyze(prog: Program<D>, init_state: B, iteration_strategy: IterationStrategy) -> HashMap<Label, B> {
        let mut all_state: HashMap<Label, B> = HashMap::new();


        for i in 0..=(prog.labels_num-1) {
            all_state.insert(i, AbstractState::bottom());
        }
        all_state.insert(prog.entry, init_state);


        let mut iteration_num= 1;
        println!("\nINITIAL STATES:\n{}\n", map_to_str(&all_state));

        if let IterationStrategy::Simple = iteration_strategy {            
            let mut new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NormalStep);
            while new_all_state != all_state {
                all_state = new_all_state;
                println!("ITERATION {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NormalStep); 
            }
        }else {
            let mut new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::WideningStep);
            while new_all_state != all_state {
                all_state = new_all_state;
                println!("ITERATION (∇) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::WideningStep); 
            }
    
            if let IterationStrategy::WideningAndNarrowing = iteration_strategy {      
                let mut new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NarrowingStep);
                while new_all_state != all_state {
                    all_state = new_all_state;
                    println!("ITERATION (Δ) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
                    new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NarrowingStep); 
                }
            }   
        }

        // println!("\nFINAL STATES:\n{}\n", map_to_str(&all_state));


        all_state
    }
}

enum StepType {
    NormalStep,
    WideningStep,
    NarrowingStep
}

impl<D: AbstractDomain, B: AbstractState<D>> GenericAnalyzer<D,B>{
    fn make_iteration(prog: &Program<D>, states: HashMap<Label, B>, step_type: StepType) -> HashMap<Label, B>{
        let mut all_states: HashMap<Label, B> = HashMap::new();
        for i in 0..=(prog.labels_num-1) {
            if i == prog.entry { all_states.insert(i, states.get(&i).unwrap().clone()); continue; }
            let arcs = prog.get_entering_arcs(i);
            let mut new_state = B::bottom();
            // println!("label {i} entering arcs:{:?}", &arcs);
            for (l,cmd,_) in arcs {
                match  states.get(l) {
                    Some(s) => new_state = new_state.lub(Self::apply_cmd(cmd, s)),
                    None => panic!("Missing AbsState for label {l}"),
                };
            }

            if prog.widening_points.contains(&i) {
                // print!("{} ∇ {} = ", states.get(&i).unwrap(), new_state);
                let old_state = states
                    .get(&i)
                    .expect(&format!("Missing AbsState for label {i}"))
                    .clone();
                new_state = match step_type {
                    StepType::NormalStep => new_state,
                    StepType::WideningStep => old_state.widening(new_state),
                    StepType::NarrowingStep => old_state.narrowing(new_state),
                }
                // println!("{}", new_state);
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
                // println!("aexpr_dom: {:?}", aexpr_dom);
                s2.set(x.to_string(), aexpr_dom);
                state = s2
            },
            Command::Test(b) => {
                state = Self::eval_bexpr(b, state);
                // println!("Apply test: {b} -> {old_state} -> {state}");
            },
        }
        state
    }

}

