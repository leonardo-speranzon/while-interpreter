use std::{collections::HashMap, path::Display, marker::PhantomData};

use iter_tools::Itertools;

use crate::{types::ast::{Statement, Aexpr, Bexpr, Operator}, analyzer::printers::map_to_str};

use super::{AbstractDomain, AbstractState, StaticAnalyzer, program::{Label, Command, Program, Arc}};
pub struct MyAnalyzer<D, B> {    
   domain: PhantomData<D>,
   abs_state: PhantomData<B>,
}

impl<D: AbstractDomain, B: AbstractState<D>> MyAnalyzer<D,B>{
    fn eval_aexpr_pre(a: &Aexpr<D>, mut state: B)-> B {
        match a {
            Aexpr::PreInc(x) => {
                let v = state.get(x);
                state.set(x.clone(), D::abstract_operator(&Operator::Add, &v, &D::from(1)));
                state
            },
            Aexpr::PreDec(x) => {
                let v = state.get(x);
                state.set(x.clone(), D::abstract_operator(&Operator::Sub, &v, &D::from(1)));
                state
            },
            Aexpr::BinOp(_, a1, a2) => {
                let state = Self::eval_aexpr_post(a1, state);
                Self::eval_aexpr_post(a2, state)
            }
            _ => state,
        }
    }
    fn eval_aexpr_post(a: &Aexpr<D>, mut state: B)-> B {
        match a {
            Aexpr::PostInc(x) => {
                let v = state.get(x);
                state.set(x.clone(), D::abstract_operator(&Operator::Add, &v, &D::from(1)));
                state
            },
            Aexpr::PostDec(x) => {
                let v = state.get(x);
                state.set(x.clone(), D::abstract_operator(&Operator::Sub, &v, &D::from(1)));
                state
            },
            Aexpr::BinOp(_, a1, a2) => {
                let state = Self::eval_aexpr_post(a1, state);
                Self::eval_aexpr_post(a2, state)
            }
            _ => state,
        }
    }
    fn eval_aexpr_f(a: &Aexpr<D>, s: B)-> (B, D) {
        let s = Self::eval_aexpr_pre(a, s);
        let n = Self::eval_aexpr(a, &s);
        let s = Self::eval_aexpr_post(a, s);
        (s, n)
    }
}

impl<D: AbstractDomain, B: AbstractState<D>> StaticAnalyzer<D,B> for MyAnalyzer<D,B>{
    
    fn eval_aexpr(a: &Aexpr<D>, s: &B)-> D {
        match a {
            Aexpr::Num(n) => n.clone(),
            Aexpr::Var(x) => s.get(x),
            Aexpr::BinOp(op, a1, a2 ) => {
                let n1 = Self::eval_aexpr(a1, s);
                let n2 = Self::eval_aexpr(a2, s);
                D::abstract_operator(op, &n1, &n2)
            }
            Aexpr::PreInc(x) => s.get(x),
            Aexpr::PreDec(x) => s.get(x),
            Aexpr::PostInc(x) => s.get(x),
            Aexpr::PostDec(x) => s.get(x),
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                let n1 = Self::eval_aexpr(a1, &s);
                let n2 = Self::eval_aexpr(a2, &s);
                let dom = n1.glb(&n2);
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
                // let dom: B = Self::eval_bexpr(b, &s);

                // let s = s.diff(adw);
                todo!();
                B::top().glb(&s)
                
            },
            Bexpr::And(_, _) => todo!(),
        }
    }

    // fn refine_aexpr(a: &Aexpr<D>, s: B, dom: &D) -> B {
    //     match a {
    //         Aexpr::Num(n) => {
    //             if n.glb(dom) == D::bottom() {
    //                 AbstractState::bottom()
    //             } else {
    //                 s
    //             }
    //         },
    //         Aexpr::Var(x) => {
    //             match s {
    //                 AbstractState(Some(mut s)) => {
    //                     let glb = s.get(x)
    //                         .unwrap_or(&AbstractDomain::top())
    //                         .glb(dom);
    //                     if glb == D::bottom() {
    //                         AbstractState(None)
    //                     } else {
    //                         s.insert(x.clone(), glb);
    //                         AbstractState(Some(s))
    //                     }
    //                 }
    //                 AbstractState(None) => s
    //             }
    //         },
    //         Aexpr::BinOp(op, a1, a2)=>{
    //             let n1 = Self::eval_aexpr(a1, &s);
    //             let n2 = Self::eval_aexpr(a2, &s);

    //             let (d1,d2) = D::backward_abstract_operator(
    //                 op,
    //                 &n1,
    //                 &n2,
    //                 dom
    //             );

    //             let s = Self::refine_aexpr(a1, s,&d1);
    //             let s = Self::refine_aexpr(a2, s,&d2);                
    //             s
    //         }
    //     }
    // }

    fn analyze(prog: Program<D>, init_state: B) -> HashMap<Label, B> {
        let mut all_state: HashMap<Label, B> = HashMap::new();

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


fn make_iteration<D: AbstractDomain, B: AbstractState<D>>(prog: &Program<D>, states: HashMap<Label, B>) -> HashMap<Label, B>{
    let mut all_states: HashMap<Label, B> = HashMap::new();
    for i in 0..=(prog.labels_num-1) {
        if i == prog.entry { all_states.insert(i, states.get(&i).unwrap().clone()); continue; }
        let arcs = get_entering_arcs(prog, i);
        let mut new_state = B::bottom();
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

fn apply_cmd<D: AbstractDomain, B: AbstractState<D>>(cmd: &Command<D>, old_state: &B) -> B{
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