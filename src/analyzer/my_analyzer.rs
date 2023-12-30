use std::{collections::HashMap, path::Display};

use iter_tools::Itertools;

use crate::{types::ast::{Statement, Aexpr, Bexpr}, analyzer::printers::map_to_str};

use super::{AbstractDomain, AbstractState, StaticAnalyzer, program::{Label, Command, Program, Arc}};
pub struct MyAnalyzer{}


impl<D: AbstractDomain> StaticAnalyzer<D> for MyAnalyzer{
    fn eval_aexpr(a: &Aexpr<D>, s: &AbstractState<D>)-> D {
        match a {
            Aexpr::Num(n) => n.clone(),
            Aexpr::Var(x) => match s {
                AbstractState(Some(s)) => match s.get(x) {
                    Some(n) => n.clone(),
                    None => D::bottom(),
                },
                AbstractState(None) => D::bottom(),
            },
            Aexpr::BinOp(op, a1, a2 ) => {
                let n1 = Self::eval_aexpr(a1, s);
                let n2 = Self::eval_aexpr(a2, s);
                D::abstract_operator(op, &n1, &n2)
            }
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: AbstractState<D>)-> AbstractState<D> {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                let n1 = Self::eval_aexpr(a1, &s);
                let n2 = Self::eval_aexpr(a2, &s);
                let dom = n1.glb(&n2);
                // let s = Self::refine_aexpr(a1, s, &dom);
                // let s = Self::refine_aexpr(a2, s, &dom);
                if dom == D::bottom() {
                    AbstractState::bottom()
                }else{
                    s   
                }
            },
            Bexpr::LessEq(a1, a2) => {
                // let n1 = Self::eval_aexpr(a1, &s);
                // let n2 = Self::eval_aexpr(a2, &s);
                // let dom = n1.glb(&n2);
                // let s = Self::refine_aexpr(a1, s, &dom);
                // let s = Self::refine_aexpr(a2, s, &dom);
                // s                
                todo!()
            },
            Bexpr::Not(b) => {
                // let dom: AbstractState<D> = Self::eval_bexpr(b, &s);

                // let s = s.diff(adw);
                AbstractState::top().glb(&s)
            },
            Bexpr::And(_, _) => todo!(),
        }
    }

    fn refine_aexpr(a: &Aexpr<D>, s: AbstractState<D>, dom: &D) -> AbstractState<D> {
        match a {
            Aexpr::Num(n) => {
                if n.glb(dom) == D::bottom() {
                    AbstractState::bottom()
                } else {
                    s
                }
            },
            Aexpr::Var(x) => {
                match s {
                    AbstractState(Some(mut s)) => {
                        let glb = s.get(x)
                            .unwrap_or(&AbstractDomain::top())
                            .glb(dom);
                        if glb == D::bottom() {
                            AbstractState(None)
                        } else {
                            s.insert(x.clone(), glb);
                            AbstractState(Some(s))
                        }
                    }
                    AbstractState(None) => s
                }
            },
            Aexpr::BinOp(op, a1, a2)=>{
                let n1 = Self::eval_aexpr(a1, &s);
                let n2 = Self::eval_aexpr(a2, &s);

                let (d1,d2) = D::backward_abstract_operator(
                    op,
                    &n1,
                    &n2,
                    dom
                );

                let s = Self::refine_aexpr(a1, s,&d1);
                let s = Self::refine_aexpr(a2, s,&d2);                
                s
            }
        }
    }

    fn analyze(prog: Program<D>, init_state: AbstractState<D>) -> HashMap<Label, AbstractState<D>> {
        let mut all_state: HashMap<Label, AbstractState<D>> = HashMap::new();

        for i in 0..=(prog.labels_num-1) {
            all_state.insert(i, AbstractState::bottom());
        }
        all_state.insert(prog.entry, init_state);


        let iteration_num = 5;

        println!("\nINITIAL STATES:\n{}\n", map_to_str(&all_state));
        for i in 0..iteration_num {
            all_state = make_iteration(&prog, all_state);
            println!("\nITERATION {}:\n{:?}\n",i+1, map_to_str(&all_state));
        }

        all_state
    }
}


fn make_iteration<D: AbstractDomain>(prog: &Program<D>, states: HashMap<Label, AbstractState<D>>) -> HashMap<Label, AbstractState<D>>{
    let mut all_states: HashMap<Label, AbstractState<D>> = HashMap::new();
    for i in 0..=(prog.labels_num-1) {
        if i == prog.entry { all_states.insert(i, states.get(&i).unwrap().clone()); continue; }
        let arcs = get_entering_arcs(prog, i);
        let mut new_state = AbstractState::bottom();
        // println!("label {i} entering arcs:{:?}", &arcs);
        for (l,cmd,_) in arcs {
            match  states.get(l) {
                Some(s) => new_state = new_state.lub(apply_cmd(cmd, s)),
                None => panic!("Missing AbsState for label {l}"),
            };
        }
        // println!("label {i} computed state{:?}", &new_state);
        all_states.insert(i, new_state);
    };
    all_states
}

fn apply_cmd<D: AbstractDomain>(cmd: &Command<D>, old_state: &AbstractState<D>) -> AbstractState<D>{
    let mut state = old_state.clone();
    match cmd {
        Command::Assignment(x, a) => {
            let aexpr_dom = MyAnalyzer::eval_aexpr(a, &state); //FIXME Self::
            // println!("aexpr_dom: {:?}", aexpr_dom);
            state.set(x.to_string(), aexpr_dom);
        },
        Command::Test(b) => {
            state = MyAnalyzer::eval_bexpr(b, state); //FIXME Self::
            println!("Apply test: {b} -> {old_state} -> {state}")
        },
    };
    // println!("apply_cmd: {:?}", state);
    state
}

fn get_entering_arcs<D>(prog: &Program<D>, label: Label) -> Vec<&Arc<D>>{
    prog.arcs.iter().filter(|(_,_,l)|l==&label).collect()
}