use std::{collections::HashMap, marker::PhantomData};

use crate::{types::ast::{Aexpr, Bexpr}, analyzer::printers::map_to_str};

use super::{AbstractDomain, AbstractState, StaticAnalyzer, program::{Label, Command, Program, Arc}};
pub struct MyAnalyzer<D, B> {    
   domain: PhantomData<D>,
   abs_state: PhantomData<B>,
}

impl<D: AbstractDomain, B: AbstractState<D>> StaticAnalyzer<D,B> for MyAnalyzer<D,B>{
    
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
                s.set(x.clone(), d.clone());
                (d, s)
            }
            Aexpr::PreDec(x) => {
                let d = s.get(x) - D::from(1);
                s.set(x.clone(), d.clone());
                (d, s)
            },
            Aexpr::PostInc(x) => {
                let d = s.get(x);
                s.set(x.clone(), d.clone() + D::from(1));
                (d, s)
            },
            Aexpr::PostDec(x) =>{
                let d = s.get(x);
                s.set(x.clone(), d.clone() - D::from(1));
                (d, s)
            },
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, mut s: B)-> B {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                match (*a1.clone(),*a2.clone()) {
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => {
                        s.set(x.clone(), s.get(&x).glb(&c));
                        s
                    },
                    (Aexpr::Var(x), Aexpr::Var(y)) => {
                        let mut s1 = s.clone() ;
                        if s.get(&x) != D::bottom() && s.get(&x) != D::top() {
                            // s1 = Self::eval_bexpr(&Bexpr::Equal(Box::new(Aexpr::Var(x.clone())), Box::new(Aexpr::Num(s.get(&x)))), s1);
                            s1.set(y.clone(), s.get(&x).glb(&s.get(&y)));
                        };
                        let mut s2 = s.clone() ;
                        if s.get(&y) != D::bottom() && s.get(&y) != D::top() {
                            s2.set(x.clone(), s.get(&x).glb(&s.get(&y)));
                        };
                        s1.glb(&s2)
                    },

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
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => {
                        s.set(x.clone(), s.get(&x).glb(&D::lte(&c)));
                        s
                    },
                    (Aexpr::Var(x), Aexpr::Var(y)) => {
                        let x_val = s.get(&x);
                        let y_val = s.get(&y);
                        s.set(x, x_val.glb(&D::lte(&y_val)));
                        s.set(y, y_val.glb(&D::gte(&x_val)));
                        s

                    }
                    (_, _) => s
                }                
                
            },
            Bexpr::And(_, _) => todo!(),

            
            Bexpr::Not(b) => {
                match *b.clone() { 
                    Bexpr::Equal(a1, a2) => {
                        let (n1, s1) = Self::eval_aexpr(&a1, s);
                        let (n2, s2) = Self::eval_aexpr(&a2, s1);
                        // let dom = n1.glb(&n2);
                        if n1 == n2 {
                            AbstractState::bottom()
                        }else{
                            s2   
                        }
                    },
                    Bexpr::LessEq(a1, a2) => {
                        match (*a1.clone(),*a2.clone()) {
                            (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => {
                                s.set(x.clone(), s.get(&x).glb(&D::gte(&(c+D::from(1)))));
                                s
                            },
                            (Aexpr::Var(x), Aexpr::Var(y)) => {
                                let x_val = s.get(&x);
                                let y_val = s.get(&y);
                                s.set(x, x_val.glb(&D::gte(&(y_val.clone() + D::from(1)))));
                                s.set(y, y_val.glb(&D::gte(&(x_val - D::from(1)))));
                                s
        
                            }
                            (_, _) => s
                        } 
                    },
                    Bexpr::True => todo!(),
                    Bexpr::False => todo!(),
                    Bexpr::Not(_) => todo!(),
                    Bexpr::And(_, _) => todo!(),
                }
            },
            // Bexpr::Not(Box<Bexpr::False>) => todo!(),
            // Bexpr::Not(Bexpr::True) => todo!(),
            // Bexpr::Not(Bexpr::Equal(_, _)) => todo!(),
            // Bexpr::Not(Bexpr::LessEq(, )) => todo!(),
            // Bexpr::Not(Bexpr::Not(_)) => todo!(),
            // Bexpr::Not(Bexpr::And(_, _)) => todo!(),
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
    //
    //             let (d1,d2) = D::backward_abstract_operator(
    //                 op,
    //                 &n1,
    //                 &n2,
    //                 dom
    //             );
    //
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


        let iteration_num = 50;

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
            let (aexpr_dom, mut s2) = MyAnalyzer::eval_aexpr(a, state); //FIXME Self::
            // println!("aexpr_dom: {:?}", aexpr_dom);
            s2.set(x.to_string(), aexpr_dom);
            state = s2
        },
        Command::Test(b) => {
            state = MyAnalyzer::eval_bexpr(b, state); //FIXME Self::
            // println!("Apply test: {b} -> {old_state} -> {state}");
        },
    }
    state
}

fn get_entering_arcs<D>(prog: &Program<D>, label: Label) -> Vec<&Arc<D>>{
    prog.arcs.iter().filter(|(_,_,l)|l==&label).collect()
}